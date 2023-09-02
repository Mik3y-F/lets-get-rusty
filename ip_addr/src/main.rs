mod dice;

use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn main() {
    let home = IpAddr::V4(Ipv4Addr::LOCALHOST);

    let loopback = IpAddr::V6(Ipv6Addr::LOCALHOST);

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

    route(home);
    route(loopback);

    let x: i8 = 5;
    let y: Option<i8> = None;
    let sum = x + match y {
        Some(i) => i,
        None => 0,
    };

    println!("sum: {:?}", sum);

    dice::dice_roll();
}

fn route(ip_kind: IpAddr) {
    println!("ip_kind: {:?}", ip_kind);
}
