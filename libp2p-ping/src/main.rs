use futures::prelude::*;
use libp2p::{
    identify, noise,
    ping::{self},
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr,
};
use log::{info, warn};
use std::{error::Error, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    #[derive(NetworkBehaviour)]
    struct MyBehaviour {
        identify: identify::Behaviour,
        ping: ping::Behaviour,
    }

    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| MyBehaviour {
            identify: identify::Behaviour::new(identify::Config::new(
                "/ipfs/0.1.0".into(),
                key.public(),
            )),
            ping: ping::Behaviour::new(ping::Config::new()),
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse::<Multiaddr>()?)?;

    if let Some(addr) = std::env::args().nth(1) {
        let remote = addr.parse::<Multiaddr>()?;
        swarm.dial(remote)?;
        info!("Dialed {addr}")
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr {
                address,
                listener_id,
            } => info!("Listening on {address:?}, listener id {listener_id:?}"),
            SwarmEvent::ConnectionEstablished { .. } => info!("connection establish"),
            SwarmEvent::ConnectionClosed { .. } => warn!("connection close"),
            SwarmEvent::Behaviour(MyBehaviourEvent::Identify(event)) => {
                info!("identify: {event:?}");
            }
            _ => {}
        }
    }
}
