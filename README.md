# rim

rim is an im server based on tokio-ws and rocket.

## front-end
[rim-front](https://github.com/ThinkCats/rim-front)

## features
### rest
* user login and register
* chat group and group user manage
* chat message manage

### websocket
* event type: Login,Msg,Read,Heart,Logout,Ack


## demo video

[![video](https://i2.hdslb.com/bfs/archive/5014732f653b331de3166bbd0eb8157352d985ff.jpg)](https://player.bilibili.com/player.html?bvid=BV1D14y1g7Zy&page=1)


## websocket protocol

### struct
Msg Struct
```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct MsgEvent {
    //event: Login,Msg,Read,Heart,Logout,Ack
    pub event: EventType,
    pub body: MsgBody,
}

```

MsgBody
```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct MsgBody {
     //message type: Text,RichText,Image...
    pub kind: MessageType,
    pub uid: u64,
    pub gid: Option<u64>,
    pub content: String,
    #[serde(rename(serialize = "clientMsgId", deserialize = "clientMsgId"))]
    pub client_msg_id: String,
    #[serde(rename(serialize = "msgId", deserialize = "msgId"))]
    pub msg_id: Option<u64>,
}
```

Server Ack
```rust
#[derive(Serialize, Deserialize)]
pub struct MsgAck {
    #[serde(rename(serialize = "clientMsgId", deserialize = "clientMsgId"))]
    pub client_msg_id: String,
    #[serde(rename(serialize = "serverMsgId", deserialize = "serverMsgId"))]
    pub server_msg_id: Option<u64>,
    pub kind: EventType,
    pub content: String,
}
```

### msg example
1. Login message 
```json
{
    "event": "Login",
    "body": {
        "kind": "Text",
        "content": "d5219358-f96b-4bfc-8672-d01ba95d16fa",
        "uid": 1,
        "clientMsgId": "72aebfd4aeef7634"
    }
}
```

server ack:
```json
{
	"clientMsgId": "72aebfd4aeef7634",
	"serverMsgId": null,
	"kind": "Ack",
	"content": "Ok"
}
```

2. Msg message

```json
{
    "event": "Msg",
    "body": {
        "kind": "Text",
        "content": "Hello World",
        "uid": 1,
        "gid": 1,
        "clientMsgId": "badc6d33aebfd40ab3cda"
    }
}
```

server ack:
```json
{
	"clientMsgId": "badc6d33aebfd40ab3cda",
	"serverMsgId": 28,
	"kind": "Ack",
	"content": "Ok"
}
```
> client msg body is same as server msg body 

3. Logout message
```json
{
    "event": "Logout",
    "body": {
        "kind": "Text",
        "content": "", //empty string
        "uid": 1,
        "clientMsgId": "72aebfd4aeef7634"
    }
}
```

server ack:
```json
{
	"clientMsgId": "72aebfd4aeef7634",
	"serverMsgId": null,
	"kind": "Ack",
	"content": "Ok"
}
```

4. client ack message
```json
{
    "event": "Ack",
    "body": {
        "kind": "Text",
        "content": "", 
        "uid": 2,
        "gid": 3,
        "clientMsgId": "72aebfd4aeefaaa",
        "msgId": 29
    }
}
```

server ack:
```json
{
	"clientMsgId": "72aebfd4aeefaaa",
	"serverMsgId": null,
	"kind": "Ack",
	"content": "Ok"
}
```

5. client read message
```json
{
    "event": "Read",
    "body": {
        "kind": "Text",
        "content": "", 
        "uid": 2,
        "gid": 3,
        "clientMsgId": "72aebfd4aeefaaa",
		"msgId": 29
    }
}
```
server ack:
```json
{
	"clientMsgId": "72aebfd4aeefaaa",
	"serverMsgId": 29,
	"kind": "Ack",
	"content": "Ok"
}
```