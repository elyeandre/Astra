use neli::{
    consts::{
        nl::NlmF,
        rtnl::{Arphrd, IffFlags, Ifla, RtAddr, Rtm},
        socket::NlFamily,
    },
    err::NlError,
    nl::{Nlmsghdr, NlPayload},
    rtnl::{Ifinfomsg, Rtattr},
    socket::NlSocketHandle,
    types::RtBuffer,
};
use tokio::task;

/// Network component for interface control
pub struct NetworkComponent;

impl NetworkComponent {
    pub async fn register_to_lua(lua: &mlua::Lua) -> mlua::Result<()> {
        // Get or create Astra table
        let astra_table: mlua::Table = if let Ok(table) = lua.globals().get("Astra") {
            table
        } else {
            let table = lua.create_table()?;
            lua.globals().set("Astra", table.clone())?;
            table
        };

        // Create net subtable
        let net_table = lua.create_table()?;

        net_table.set(
            "set_link_up",
            lua.create_async_function(|_, iface: String| async move {
                set_link_state(&iface, true).await
            })?,
        )?;

        net_table.set(
            "set_link_down",
            lua.create_async_function(|_, iface: String| async move {
                set_link_state(&iface, false).await
            })?,
        )?;

        astra_table.set("net", net_table)?;
        Ok(())
    }

    pub fn lua_code() -> &'static str {
        include_str!("network.lua")
    }
}

async fn set_link_state(iface: &str, up: bool) -> mlua::Result<()> {
    let iface = iface.to_string();
    task::spawn_blocking(move || blocking_set_link_state(&iface, up))
        .await
        .map_err(|e| mlua::Error::runtime(e.to_string()))?
        .map_err(|e| mlua::Error::runtime(e.to_string()))
}

fn blocking_set_link_state(iface: &str, up: bool) -> Result<(), NlError> {
    let mut socket = NlSocketHandle::connect(NlFamily::Route, None, &[])?;

    let mut attrs = RtBuffer::new();

    // Set interface flags (IFF_UP) for link up/down
    let flags = if up {
        IffFlags::IFF_UP
    } else {
        IffFlags::empty()
    };

    let mut ifmsg = Ifinfomsg::new(
        RtAddr::Unspecified,
        Arphrd::UnrecognizedConst(0),
        0, // Will set index later
        flags,
        IffFlags::IFF_UP, // Change mask
        attrs,
    );

    // Get index
    if let Some(idx) = get_interface_index(iface)? {
        ifmsg.ifi_index = idx as i32;
    } else {
        return Err(NlError::Msg("Interface not found".into()));
    }

    let nlhdr = Nlmsghdr::new(
        None,
        if up { Rtm::Setlink } else { Rtm::Dellink },
        NlmF::Request | NlmF::Ack,
        None,
        None,
        NlPayload::Payload(ifmsg),
    );

    socket.send(nlhdr, 0)?;

    let mut buf = vec![0; 4096];
    let _ = socket.recv(&mut buf, 0)?;
    Ok(())
}

fn get_interface_index(name: &str) -> Result<Option<u32>, NlError> {
    let mut socket = NlSocketHandle::connect(NlFamily::Route, None, &[])?;

    let ifmsg = Ifinfomsg::new(
        RtAddr::Unspecified,
        Arphrd::UnrecognizedConst(0),
        0,
        IffFlags::empty(),
        IffFlags::empty(),
        RtBuffer::new(),
    );

    let nlhdr = Nlmsghdr::new(
        None,
        Rtm::Getlink,
        NlmF::Request | NlmF::Dump,
        None,
        None,
        NlPayload::Payload(ifmsg),
    );

    socket.send(nlhdr, 0)?;

    let mut buf = vec![0; 4096];
    let size = socket.recv(&mut buf, 0)?;

    let messages = Nlmsghdr::<Rtm, Ifinfomsg>::from_bytes(&buf[..size])?;

    for msg in messages {
        if let NlPayload::Payload(ifinfo) = msg.nl_payload {
            for attr in ifinfo.rtattrs.iter() {
                if attr.rta_type == Ifla::Ifname {
                    let ifname = std::str::from_utf8(attr.rta_payload.as_ref()).unwrap_or("");
                    if ifname == name {
                        return Ok(Some(ifinfo.ifi_index as u32));
                    }
                }
            }
        }
    }

    Ok(None)
}
