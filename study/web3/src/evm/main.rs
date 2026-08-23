pub mod instruction;
pub mod interpreter;

use interpreter::{ExecError, Interpreter};

fn main() {
    let code = vec![
        0x60, 0x02, // PUSH1 2
        0x60, 0x03, // PUSH1 3
        0x01, // ADD
        0x00, // STOP
    ];

    let mut interpreter = Interpreter::new(code, 10_000);

    match interpreter.execute() {
        Err(ExecError::Stop) => {
            println!(
                "execution stopped: stack_depth={}, gas_used={}",
                interpreter.stack.depth(),
                interpreter.context.gas_used
            );
        }
        Ok(return_data) => {
            println!("execution returned {} bytes", return_data.len());
        }
        Err(error) => {
            eprintln!("execution failed: {error:?}");
            std::process::exit(1);
        }
    }
}
