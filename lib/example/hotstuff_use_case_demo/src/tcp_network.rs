use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use borsh::{BorshDeserialize, BorshSerialize};
use ed25519_dalek::VerifyingKey;
use hotstuff_rs::networking::messages::Message;
use hotstuff_rs::networking::network::Network;
use hotstuff_rs::types::update_sets::ValidatorSetUpdates;
use hotstuff_rs::types::validator_set::ValidatorSet;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc as tokio_mpsc;

const VERIFYING_KEY_LEN: usize = 32;
const OUTBOUND_BUFFER: usize = 4096;
const CONNECT_TIMEOUT: Duration = Duration::from_millis(200);
const SEND_ERROR_LOG_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Clone)]
pub struct TcpNetwork {
    my_verifying_key: VerifyingKey,
    peer_addrs: Arc<Mutex<HashMap<VerifyingKey, SocketAddr>>>,
    inbound_sender: Sender<(VerifyingKey, Message)>,
    inbound_receiver: Arc<Mutex<Receiver<(VerifyingKey, Message)>>>,
    outbound_sender: tokio_mpsc::Sender<OutboundMessage>,
}

struct OutboundMessage {
    peer: VerifyingKey,
    message: Message,
}

impl TcpNetwork {
    pub fn bind(
        my_verifying_key: VerifyingKey,
        my_addr: SocketAddr,
        peer_addrs: HashMap<VerifyingKey, SocketAddr>,
    ) -> io::Result<Self> {
        let std_listener = std::net::TcpListener::bind(my_addr)?;
        std_listener.set_nonblocking(true)?;
        let local_addr = std_listener.local_addr()?;

        let (inbound_sender, inbound_receiver) = mpsc::channel();
        let (outbound_sender, outbound_receiver) = tokio_mpsc::channel(OUTBOUND_BUFFER);
        let network = Self {
            my_verifying_key,
            peer_addrs: Arc::new(Mutex::new(peer_addrs)),
            inbound_sender,
            inbound_receiver: Arc::new(Mutex::new(inbound_receiver)),
            outbound_sender,
        };

        network.spawn_io_thread(std_listener, outbound_receiver, local_addr);
        Ok(network)
    }

    fn spawn_io_thread(
        &self,
        std_listener: std::net::TcpListener,
        outbound_receiver: tokio_mpsc::Receiver<OutboundMessage>,
        local_addr: SocketAddr,
    ) {
        let inbound_sender = self.inbound_sender.clone();
        let peer_addrs = Arc::clone(&self.peer_addrs);
        let my_verifying_key = self.my_verifying_key;

        std::thread::spawn(move || {
            let runtime = match tokio::runtime::Builder::new_multi_thread().enable_all().build() {
                Ok(runtime) => runtime,
                Err(error) => {
                    eprintln!("[hotstuff_tcp_network] runtime 启动失败: {error}");
                    return;
                }
            };

            runtime.block_on(async move {
                let listener = match TcpListener::from_std(std_listener) {
                    Ok(listener) => listener,
                    Err(error) => {
                        eprintln!("[hotstuff_tcp_network] listener 初始化失败: {error}");
                        return;
                    }
                };

                println!("[hotstuff_tcp_network] TCP listener 已启动: {local_addr}");
                tokio::join!(
                    accept_loop(listener, inbound_sender),
                    outbound_loop(my_verifying_key, peer_addrs, outbound_receiver)
                );
            });
        });
    }

    fn enqueue_local(&self, message: Message) {
        let _ = self.inbound_sender.send((self.my_verifying_key, message));
    }

    fn enqueue_outbound(&self, peer: VerifyingKey, message: Message) {
        if peer == self.my_verifying_key {
            self.enqueue_local(message);
            return;
        }

        let _ = self.outbound_sender.try_send(OutboundMessage { peer, message });
    }
}

impl Network for TcpNetwork {
    fn init_validator_set(&mut self, _: ValidatorSet) {}

    fn update_validator_set(&mut self, _: ValidatorSetUpdates) {}

    fn broadcast(&mut self, message: Message) {
        let peers = self
            .peer_addrs
            .lock()
            .map(|peers| peers.keys().copied().collect::<Vec<_>>())
            .unwrap_or_default();
        for peer in peers {
            self.enqueue_outbound(peer, message.clone());
        }
    }

    fn send(&mut self, peer: VerifyingKey, message: Message) {
        self.enqueue_outbound(peer, message);
    }

    fn recv(&mut self) -> Option<(VerifyingKey, Message)> {
        let Ok(inbox) = self.inbound_receiver.lock() else {
            return None;
        };
        inbox.try_recv().ok()
    }
}

async fn accept_loop(listener: TcpListener, inbound_sender: Sender<(VerifyingKey, Message)>) {
    loop {
        match listener.accept().await {
            Ok((stream, _peer_addr)) => {
                let inbound_sender = inbound_sender.clone();
                tokio::spawn(async move {
                    if let Err(error) = read_connection(stream, inbound_sender).await {
                        eprintln!("[hotstuff_tcp_network] TCP receive 失败: {error}");
                    }
                });
            }
            Err(error) => {
                eprintln!("[hotstuff_tcp_network] TCP accept 失败: {error}");
            }
        }
    }
}

async fn outbound_loop(
    my_verifying_key: VerifyingKey,
    peer_addrs: Arc<Mutex<HashMap<VerifyingKey, SocketAddr>>>,
    mut outbound_receiver: tokio_mpsc::Receiver<OutboundMessage>,
) {
    let mut streams = HashMap::new();
    let mut last_error_logs = HashMap::new();

    while let Some(outbound) = outbound_receiver.recv().await {
        let peer_addr = peer_addrs.lock().ok().and_then(|peers| peers.get(&outbound.peer).copied());
        let Some(peer_addr) = peer_addr else {
            continue;
        };

        if let Err(error) =
            write_message(peer_addr, my_verifying_key, &outbound.message, &mut streams).await
        {
            log_send_error(peer_addr, &error, &mut last_error_logs);
        }
    }
}

async fn write_message(
    peer_addr: SocketAddr,
    sender: VerifyingKey,
    message: &Message,
    streams: &mut HashMap<SocketAddr, TcpStream>,
) -> io::Result<()> {
    let mut frame = Vec::with_capacity(VERIFYING_KEY_LEN + 1024);
    frame.extend_from_slice(&sender.to_bytes());
    frame.extend_from_slice(&message.try_to_vec()?);

    if !streams.contains_key(&peer_addr) {
        let stream = connect_stream(peer_addr).await?;
        streams.insert(peer_addr, stream);
    }

    let write_result = match streams.get_mut(&peer_addr) {
        Some(stream) => write_frame(stream, &frame).await,
        None => Err(io::Error::new(io::ErrorKind::NotConnected, "missing peer stream")),
    };
    if write_result.is_ok() {
        return Ok(());
    }

    let _ = streams.remove(&peer_addr);
    let mut stream = connect_stream(peer_addr).await?;
    write_frame(&mut stream, &frame).await?;
    streams.insert(peer_addr, stream);
    Ok(())
}

async fn connect_stream(peer_addr: SocketAddr) -> io::Result<TcpStream> {
    tokio::time::timeout(CONNECT_TIMEOUT, TcpStream::connect(peer_addr))
        .await
        .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "connect timeout"))?
}

async fn write_frame(stream: &mut TcpStream, frame: &[u8]) -> io::Result<()> {
    let frame_len = u32::try_from(frame.len())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "frame too large"))?;
    stream.write_u32(frame_len).await?;
    stream.write_all(&frame).await?;
    stream.flush().await
}

fn log_send_error(
    peer_addr: SocketAddr,
    error: &io::Error,
    last_error_logs: &mut HashMap<SocketAddr, Instant>,
) {
    let now = Instant::now();
    let should_log = last_error_logs
        .get(&peer_addr)
        .map(|last_log| now.duration_since(*last_log) >= SEND_ERROR_LOG_INTERVAL)
        .unwrap_or(true);
    if should_log {
        eprintln!("[hotstuff_tcp_network] TCP send 到 {peer_addr} 失败: {error}");
        last_error_logs.insert(peer_addr, now);
    }
}

async fn read_connection(
    mut stream: TcpStream,
    inbound_sender: Sender<(VerifyingKey, Message)>,
) -> io::Result<()> {
    loop {
        let frame_len = match stream.read_u32().await {
            Ok(frame_len) => frame_len as usize,
            Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => return Ok(()),
            Err(error) => return Err(error),
        };
        if frame_len < VERIFYING_KEY_LEN {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "frame shorter than sender key",
            ));
        }

        let mut frame = vec![0; frame_len];
        stream.read_exact(&mut frame).await?;

        let mut sender_bytes = [0u8; VERIFYING_KEY_LEN];
        sender_bytes.copy_from_slice(&frame[..VERIFYING_KEY_LEN]);
        let sender = VerifyingKey::from_bytes(&sender_bytes)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid sender key"))?;
        let message = Message::try_from_slice(&frame[VERIFYING_KEY_LEN..])
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        let _ = inbound_sender.send((sender, message));
    }
}
