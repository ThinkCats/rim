<div align="center">

# rim 
![GitHub branch checks state](https://img.shields.io/github/checks-status/ThinkCats/rim/master)
![GitHub](https://img.shields.io/github/license/ThinkCats/rim)

</div>

rim 是一个使用rust编写的im服务端。 [English](./README_en.md)


## 前端展示
[rim-front](https://github.com/ThinkCats/rim-front) , 使用react编写，仅做演示用.

## 功能列表

* 群聊 & 单聊功能
* 用户的注册/登录
* 好友关系
* 消息的服务端存储
* 多消息类型格式支持


## demo 视频

[![video](https://i2.hdslb.com/bfs/archive/5014732f653b331de3166bbd0eb8157352d985ff.jpg)](https://player.bilibili.com/player.html?bvid=BV1D14y1g7Zy&page=1)

> 😄好友关系前端暂未实现


## rest api 文档
[api doc](doc/RIM.html)


## websocket 消息体协议

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