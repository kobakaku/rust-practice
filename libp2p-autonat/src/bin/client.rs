use futures::prelude::*;
use std::{
    error::Error, // , net::Ipv4Addr
    time::Duration,
};

use clap::Parser;
use libp2p::{
    autonat,
    identify,
    identity,
    // multiaddr::Protocol,
    noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp,
    yamux,
    Multiaddr,
    PeerId,
};
use log::{info, warn};

#[derive(Debug, Parser)]
#[clap(name = "libp2p autonat")]
struct Opt {
    #[clap(long, short)]
    listen_port: Option<u16>,

    #[clap(long)]
    server_address: Multiaddr,

    #[clap(long)]
    server_peer_id: PeerId,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let opt = Opt::parse();

    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| Behaviour::new(key.public()))?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse::<Multiaddr>()?)?;
    // swarm.listen_on(
    //     Multiaddr::empty()
    //         .with(Protocol::Ip4(Ipv4Addr::UNSPECIFIED))
    //         .with(Protocol::Tcp(opt.listen_port.unwrap_or(0))),
    // )?;

    swarm
        .behaviour_mut()
        .auto_nat
        .add_server(opt.server_peer_id, Some(opt.server_address));

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => info!("Listening on {address:?}"),
            SwarmEvent::ConnectionEstablished { .. } => info!("Connection established !!!"),
            SwarmEvent::ConnectionClosed { .. } => warn!("Connection Closed"),
            SwarmEvent::Behaviour(BehaviourEvent::Identify(event)) => {
                info!("identify: {event:?}");
            }
            SwarmEvent::Behaviour(BehaviourEvent::AutoNat(event)) => {
                info!("Auto nat: {event:?}");
            }
            e => info!("{e:?}"),
        }
    }
}

#[derive(NetworkBehaviour)]
struct Behaviour {
    identify: identify::Behaviour,
    auto_nat: autonat::Behaviour,
}

impl Behaviour {
    fn new(local_public_key: identity::PublicKey) -> Self {
        Self {
            identify: identify::Behaviour::new(identify::Config::new(
                "/ipfs/0.1.0".into(),
                local_public_key.clone(),
            )),
            auto_nat: autonat::Behaviour::new(
                local_public_key.to_peer_id(),
                autonat::Config {
                    retry_interval: Duration::from_secs(10),
                    refresh_interval: Duration::from_secs(30),
                    boot_delay: Duration::from_secs(5),
                    throttle_server_period: Duration::ZERO,
                    only_global_ips: false,
                    ..Default::default()
                },
            ),
        }
    }
}

// #[derive(Debug)]
// #[allow(clippy::large_enum_variant)]
// enum Event {
//     AutoNat(autonat::Event),
//     Identify(identify::Event),
// }

// impl From<identify::Event> for Event {
//     fn from(v: identify::Event) -> Self {
//         Self::Identify(v)
//     }
// }

// impl From<autonat::Event> for Event {
//     fn from(v: autonat::Event) -> Self {
//         Self::AutoNat(v)
//     }
// }
