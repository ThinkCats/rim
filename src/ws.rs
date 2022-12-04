use std::{
    collections::HashMap,
    env,
    io::Error,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    future, pin_mut, StreamExt, TryStreamExt,
};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use crate::{message::message_model::MsgEvent, wss::ws_service::handle_ws_msg};

pub type Sender = UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Sender>>>;
pub type UserPeerMap = Arc<Mutex<HashMap<u64, Sender>>>;

pub async fn launch_ws() -> Result<(), Error> {
    //TODO handle Ctrl-C command
    println!("Start Ws ...");
    let conn_state = PeerMap::new(Mutex::new(HashMap::new()));
    let user_state = UserPeerMap::new(Mutex::new(HashMap::new()));

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:3012".to_string());
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Ws Listening on: {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(
            conn_state.clone(),
            user_state.clone(),
            stream,
            addr,
        ));
    }

    Ok(())
}

async fn handle_connection(
    state: PeerMap,
    user_state: UserPeerMap,
    raw_stream: TcpStream,
    addr: SocketAddr,
) {
    println!("Tcp connection from: {}", addr);
    let ws_stream = accept_async(raw_stream)
        .await
        .expect("error during handshake");
    println!("websocket connection established: {}", addr);

    let (sender, receiver) = unbounded();

    state.lock().unwrap().insert(addr, sender);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        let peer = state.lock().unwrap();
        let self_sender = peer.get(&addr).unwrap();

        let msg_json = msg.to_text().unwrap();
        println!("received msg from:{}, msg:{}", addr, msg_json);

        //parse msg body
        let msg_event = parse_msg(msg_json);
        if msg_event.is_none() {
            println!("msg body error,skip.");
            return future::ok(());
        }

        let m_event = msg_event.unwrap();
        println!("msg event:{}", m_event.body.content);
        //TODO main process and send to receiver
        handle_ws_msg(&m_event, &user_state, &self_sender);

        // let resp = format!("ACK for your: {}", m_event.body.content);
        //ack and send msg
        //send_msg(self_sender, resp);

        // We want to broadcast the message to everyone except ourselves.
        // let broadcast_recipients = peer
        //     .iter()
        //     .filter(|(peer_addr, _)| peer_addr != &&addr)
        //     .map(|(addr, _)| addr);

        // for recp in broadcast_recipients {
        //     println!("response to other client addr:{}, msg:{}", recp, msg);
        //     let sender = peer.get(recp).expect(
        //         format!("can not get sender from state hashmap for addr:{}", addr).as_str(),
        //     );
        //     // sender.unbounded_send(Message::Text(resp.clone())).unwrap();
        // }

        future::ok(())
    });

    let receive_from_others = receiver.map(Ok).forward(outgoing);
    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    state.lock().unwrap().remove(&addr);
}

fn parse_msg(msg: &str) -> Option<MsgEvent> {
    if msg.is_empty() {
        return None;
    }
    let msg_event: Result<MsgEvent, serde_json::Error> = serde_json::from_str(msg);
    match msg_event {
        Ok(event) => {
            let body = &event.body;
            if body.client_msg_id.is_empty() {
                return None;
            }
            return Some(event);
        }
        Err(_) => {
            println!("find error, do nothing");
            None
        },
    }
}
