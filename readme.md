### Chat With Rust
一款简单的未加密的Tcp聊天软件
#### Build
cargo build -r
#### Usage
##### start
`./bin`
##### commands
`conn <remote Address>` connect to another client  

`connsta` the state of connects , the short vision is `sta!`  

`msgbox <remote address> ...<msgs> ` send a raw message to remote address ,short vision is `msg!`.The remote address must be conned. 

`chatwith <remote address>`open window to chat.The remote address must be conned. short vision is `cw!`

`chatmsg ...<messages>`on window send a raw message to remote address which is selected by  `chatwith`,short vision is `chat!`