use std::{
    collections::HashMap,
    env, io,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    future, pin_mut, StreamExt, TryStreamExt,
};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

type Sender = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Sender>>>;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    println!("Start Ws ...");

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:3012".to_string());
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Ws Listening on: {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr));
    }

    Ok(())
}

async fn handle_connection(state: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Tcp connection from: {}", addr);
    let ws_stream = accept_async(raw_stream)
        .await
        .expect("error during handshake");
    println!("websocket connection established: {}", addr);

    let (sender, receiver) = unbounded();

    state.lock().unwrap().insert(addr, sender);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        println!("received msg from:{}, msg:{}", addr, msg.to_text().unwrap());
        let peer = state.lock().unwrap();

        // We want to broadcast the message to everyone except ourselves.
        let broadcast_recipients = peer
            .iter()
            .filter(|(peer_addr, _)| peer_addr != &&addr)
            .map(|(addr, _)| addr);

        for recp in broadcast_recipients {
            println!("response to other client addr:{}, msg:{}", recp, msg);
            let sender = peer.get(recp).expect(
                format!("can not get sender from state hashmap for addr:{}", addr).as_str(),
            );
            sender.unbounded_send(msg.clone()).unwrap();
        }

        future::ok(())
    });

    let receive_from_others = receiver.map(Ok).forward(outgoing);
    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    state.lock().unwrap().remove(&addr);
}
