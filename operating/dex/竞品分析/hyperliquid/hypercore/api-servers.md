# API servers

API servers monitor updates from a node and keep the blockchain state synchronized locally. They provide information about this state and relay user transactions to the node. The API offers two data sources: REST and Websocket.

When users submit transactions to an API server, they get forwarded to the connected node, which then distributes the transaction through the HyperBFT consensus algorithm. After the transaction is included in a committed block on the L1, the API server sends back the execution response from the L1 to the original request.
