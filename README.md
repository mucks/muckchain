# Muckchain
Simple modular blockchain written in Rust.


### modules

- config (contains the config)
- core (contains the blockchain, blocks, transactions, etc.)
- net (contains everything related to networking, like the transport layer, message processor, etc.)
- crypto (contains the crypto stuff, like the private key, signature, etc.)

### modular parts

- Encoding
- VM
- State (State for VM)
- BlockValidator
- Transport (Local, TCP, UDP)
- Storage (used to store blocks)
- Hasher (used to hash Transactions, Blocks, etc.)


### Simple Class Diagram
```mermaid
classDiagram
  Node --> Validator: if validator
  Node --> MessageProcessor
  Node --> MessageSender
  Node --> Blockchain
  Node: id
  Node: config
  Blockchain --> VM
  Blockchain --> Block
  Blockchain --> Storage
  Storage: get(key)
  Storage: put(key, value)
  Storage: delete(key)
  Storage *-- MemStorage
  Block --> Transaction
  Block: sign(privKey)
  Block: verify()
  Transaction: sign(privKey)
  Transaction: verify()
  Transaction: data
  Transaction: pubKey
  Transaction: signature
  Transaction --> VM
  Blockchain: add_block(block)
  VM *-- ByteCodeVM
  VM: execute()
  note for VM "executes transactions\nhandles contract state"
  VM <--> State
  Transport *-- LocalTransport
  Transport *-- TcpTransport
  Transport: send(to, msg)
  Transport: broadcast(to, msg)
  note for Transport "communication between nodes"
  MessageProcessor --> MessageSender
  MessageProcessor --> Transport
  MessageProcessor: process_message(from, msg)
  MessageSender: send(to, msg)
  MessageSender: broadcast(to, msg)
  MessageSender --> Transport
  Validator --> MessageSender
  Validator: privKey
  Validator: create_new_block()


```

### Node communication

```mermaid
sequenceDiagram
  participant LocalNode
  participant RemoteNode
  participant LateNode
  Note over LocalNode: is validator
  Note left of RemoteNode: Block Syncing
  LocalNode->>RemoteNode: get status
  RemoteNode->>LocalNode: status
  LocalNode->>RemoteNode: get blocks
  RemoteNode->>LocalNode: blocks
  LocalNode->>LateNode: get status
  LateNode->>LocalNode: status
  LocalNode->>LateNode: get blocks
  LateNode->>LocalNode: blocks
  Note left of RemoteNode: Block Creation
  loop blocktime
    LocalNode-->LocalNode: Create Block
    LocalNode->>RemoteNode: send block
    LocalNode->>LateNode: send block
  end
```

### Roadmap

- add multiple consensus algorithms
- add multiple vm's (maybe support a webscraping vm?)

