use futures::StreamExt;
use libp2p::{identity, Multiaddr, PeerId, Swarm};
use libp2p::ping::Behaviour;
use libp2p::swarm::SwarmEvent;
use crate::core::corex::CoreX;
use crate::logger;

impl CoreX {
    pub async fn run(&self) {
        let mut log = logger::get_logger();
        log.info(&format!("{} running", self.node_name));


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
        if let Some(addr) = std::env::args().nth(1) {
            let remote: Multiaddr = addr.parse().unwrap();
            swarm.dial(remote).unwrap();
            println!("Dialed {addr}")
        }

        loop {
            match swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
                SwarmEvent::Behaviour(event) => println!("{event:?}"),
                _ => {}
            }
        }

    }
}