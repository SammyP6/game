#![feature(nll)]

extern crate client;
extern crate get_if_addrs;
extern crate syrup;
extern crate common;

use std::io;
use std::sync::mpsc;
use std::net::SocketAddr;

use syrup::Window;

use client::{Client, ClientMode};

fn main() {
    println!("Starting headless client...");

    let ip = std::net::IpAddr::V4(std::net::Ipv4Addr::new(0,0,0,0));
    let mut port = String::new();
    println!("Local port [autodetect-59001]:");
    io::stdin().read_line(&mut port).unwrap();
    let mut port = port.trim();
    if port.len() == 0 {
        port = "59001";
    }
    let port = u16::from_str_radix(&port.trim(), 10).unwrap();

    println!("Binding local port to {}:{}...", ip.to_string(), port);

    let mut remote_addr = String::new();
    println!("Remote server address [127.0.0.1:59003]:");
    io::stdin().read_line(&mut remote_addr).unwrap();
    let mut remote_addr = remote_addr.trim();
    if remote_addr.len() == 0 {
        remote_addr = "127.0.0.1:59003";
    } else if remote_addr == "m" {
        remote_addr = "91.67.21.222:38888";
    }

    let name_seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().subsec_micros();
    let default_alias = common::NameGenerator::new(name_seed).get().to_string();
    let mut alias = String::new();

    println!("Alias: [{}]", default_alias);
    io::stdin().read_line(&mut alias).unwrap();
    let mut alias = alias.trim().to_string();
    if alias.len() == 0 {
        alias = default_alias;
    }

    let mut win = Window::initscr();
    win.writeln("Welcome to the Veloren headless client.");

    let client = match Client::new(ClientMode::Headless, alias, SocketAddr::new(ip, port), &remote_addr.trim()) {
        Ok(c) => c,
        Err(e) => panic!("An error occured when attempting to initiate the client: {:?}", e),
    };

    let (tx, rx) = mpsc::channel();
    client.callbacks().set_recv_chat_msg(move |alias, msg| {
        tx.send(format!("{}: {}", alias, msg)).unwrap();
    });

    loop {
        if let Ok(msg) = rx.try_recv() {
            win.writeln(format!("{}", msg));
        }

        if let Some(msg) = win.get() {
            if msg.starts_with("!") {
                client.send_cmd(&msg[1..]);
            }
            else {
                client.send_chat_msg(&msg);
            }
        }
    }
}
