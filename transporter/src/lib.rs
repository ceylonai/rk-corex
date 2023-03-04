use futures::StreamExt;
use libp2p::{identity, Multiaddr, PeerId, ping, Swarm};
use libp2p::swarm::{keep_alive, NetworkBehaviour, SwarmEvent};

#[derive(NetworkBehaviour, Default)]
struct Behaviour {
    keep_alive: keep_alive::Behaviour,
    ping: ping::Behaviour,
}

pub async fn start_transporter(tx: tokio::sync::mpsc::Sender<String>, connection: Option<String>) {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {local_peer_id:?}");

    let transport = libp2p::development_transport(local_key).await.unwrap();

    let mut swarm = Swarm::with_async_std_executor(transport, Behaviour::default(), local_peer_id);

    // Tell the swarm to listen on all interfaces and a random, OS-assigned
    // port.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

    // Dial the peer identified by the multi-address given as the second
    // command-line argument, if any.
    if let Some(addr) = connection {
        let remote: Multiaddr = addr.parse().unwrap();
        match swarm.dial(remote) {
            Ok(()) => println!("Dialed {addr}"),
            Err(e) => println!("Failed to dial {addr}: {e}"),
        };
        println!("Dialed {addr}");
    }

    tx.send(format!("Listening on")).await.unwrap();

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => {
                tx.send(format!("{:?}", event)).await.unwrap();
            }
            _ => {}
        }
    }
}