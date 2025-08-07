use neli::consts::{nl::Nlmsg, rtnl::{Arphrd, Ifinfomsg, Ifla, RtAddrFamily, Rtm}, NlmF};
use neli::err::NlError;
use neli::nl::{NlPayload, Nlmsghdr};
use neli::rtnl::IfinfomsgBuilder;
use neli::socket::NlSocketHandle;
use neli::types::RtBuffer;
use neli::FromBytes; // for from_bytes
use std::io;

pub fn get_interfaces() -> Result<(), Box<dyn std::error::Error>> {
    let mut socket = NlSocketHandle::connect(
        neli::consts::socket::NlFamily::Route,
        None,
        &[],
    )?;

    let ifi = IfinfomsgBuilder::default()
        .ifi_family(RtAddrFamily::Unspecified)
        .ifi_type(Arphrd::Ether)
        .ifi_index(0)
        .ifi_flags(0)
        .ifi_change(0xffffffff)
        .build()?;

    let nlhdr = Nlmsghdr::new(
        None,
        Rtm::Getlink,
        NlmF::Request.into() | NlmF::Ack.into(),
        None,
        None,
        NlPayload::Payload(ifi),
    );

    let buf = nlhdr.to_bytes()?;
    socket.send(&buf, 0)?;

    let mut recv_buf = vec![0; 4096];
    let size = socket.recv(&mut recv_buf[..], 0)?;

    let msgs = Nlmsghdr::<Rtm, Ifinfomsg>::from_bytes(&recv_buf[..size])?;
    println!("Parsed {} bytes", size);
    println!("Got message: {:?}", msgs);
    Ok(())
}

pub fn dump_interfaces() -> Result<(), Box<dyn std::error::Error>> {
    let mut socket = NlSocketHandle::connect(
        neli::consts::socket::NlFamily::Route,
        None,
        &[],
    )?;

    let ifi = IfinfomsgBuilder::default()
        .ifi_family(RtAddrFamily::Unspecified)
        .ifi_type(Arphrd::Ether)
        .ifi_index(0)
        .ifi_flags(0)
        .ifi_change(0xffffffff)
        .build()?;

    let nlhdr = Nlmsghdr::new(
        None,
        Rtm::Getlink,
        NlmF::Request.into() | NlmF::Dump.into(),
        None,
        None,
        NlPayload::Payload(ifi),
    );

    let buf = nlhdr.to_bytes()?;
    socket.send(&buf, 0)?;

    let mut recv_buf = vec![0; 8192];
    let size = socket.recv(&mut recv_buf, 0)?;
    let msgs = Nlmsghdr::<Rtm, Ifinfomsg>::from_bytes(&recv_buf[..size?])?;

    println!("Interfaces:");
    println!("{:?}", msgs);
    Ok(())
}


