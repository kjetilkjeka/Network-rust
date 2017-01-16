
#![feature(mpsc_select)]

extern crate rand;
extern crate network_rust;

use std::io;
use std::thread;
use std::sync::mpsc::channel;

use rand::Rng;

use network_rust::localip::get_localip;
use network_rust::peer::{PeerTransmitter, PeerReceiver, PeerUpdate};
use network_rust::bcast::{BcastTransmitter, BcastReceiver};
.unwrap()
const PEER_PORT: u16 = 9877;
const BCAST_PORT: u16 = 9876;

fn main() {
    let unique = rand::thread_rng().gen::<u16>();

    // Spawn peer transmitter and receiver
    thread::spawn(move || {
        let id = format!("{}:{}", get_localip().unwrap(), unique);
        PeerTransmitter::new(PEER_PORT)
            .expect("Error creating PeerTransmitter")
            .run(&id);
    });
    let (peer_tx, peer_rx) = channel::<PeerUpdate<String>>();
    thread::spawn(move|| { 
        PeerReceiver::new(PEER_PORT)
            .expect("Error creating PeerReceiver")
            .run(peer_tx);
    });

    // Spawn broadcast transmitter and receiver
    let (transmit_tx, transmit_rx) = channel::<String>();
    let (receive_tx, receive_rx) = channel::<String>();
    thread::spawn(move|| {
        BcastTransmitter::new(BCAST_PORT)
            .expect("Error creating ")
            .run(transmit_rx);
    });
    thread::spawn(move|| {
        BcastReceiver::new(BCAST_PORT)
            .expect("Error creating BcastReceiver")
            .run(receive_tx);
    });

    // Spawn user interface
    thread::spawn(move|| {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Couldn't read line");
            transmit_tx.send(input).unwrap();
        }
    });

    // Start infinite loop waiting on either bcast msg or peer update
    loop {
        select! {
            update = peer_rx.recv() => {
                println!("{}", update.unwrap());
            },
            bcast_msg = receive_rx.recv() => {
                println!("Got bcast_msg: {}", bcast_msg.unwrap());
            }
        }
    }
}
