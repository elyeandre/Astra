use neli::{
    consts::{
        nl::NlmF,
        rtnl::{Arphrd, IffFlags, Ifla, RtAddrFamily, Rtm},
        socket::NlFamily,
    },
    err::NlError,
    nl::{NlPayload, Nlmsghdr},
    rtnl::Ifinfomsg,
    socket::NlSocket,
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
    let mut socket = NlSocket::connect(NlFamily::Route, None, &[])?;

    let mut attrs = RtBuffer::new();
    let ifmsg = Ifinfomsg::new(
        RtAddrFamily::Unspecified,
        Arphrd::UnrecognizedConst(0),
        0,
        IffFlags::empty(),
        IffFlags::empty(),
        attrs,
    );

    let mut nlhdr = Nlmsghdr::new(
        None,
        if up { Rtm::Setlink } else { Rtm::Dellink },
        NlmF::Request | NlmF::Ack,
        None,
        None,
        NlPayload::Payload(ifmsg),
    );

    if let Some(idx) = get_interface_index(iface)? {
        if let NlPayload::Payload(ref mut msg) = nlhdr.nl_payload {
            msg.ifi_index = idx as i32;
        } else {
            return Err(NlError::Msg("Unexpected payload type".into()));
        }
    } else {
        return Err(NlError::Msg("Interface not found".into()));
    }

    socket.send(&nlhdr, 0)?;

    let mut buf = vec![0; 4096];
    let _ = socket.recv(&mut buf, 0)?;
    Ok(())
}

fn get_interface_index(name: &str) -> Result<Option<u32>, NlError> {
    let mut socket = NlSocket::connect(NlFamily::Route, None, &[])?;

    let ifmsg = Ifinfomsg::new(
        RtAddrFamily::Unspecified,
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

    socket.send(&nlhdr, 0)?;

    let mut buf = vec![0; 8192];
    let size = socket.recv(&mut buf, 0)?;
    let msgs = Nlmsghdr::<Rtm, Ifinfomsg>::from_bytes(&buf[..size])?;

    for msg in msgs {
        if let NlPayload::Payload(payload) = msg.nl_payload {
            if let Some(attrs) = payload.rtattrs.get_attr_handle().get_attr_payload_as::<Vec<u8>>(&Ifla::Ifname).ok() {
                if let Ok(ifname) = String::from_utf8(attrs) {
                    if ifname == name {
                        return Ok(Some(payload.ifi_index as u32));
                    }
                }
            }
        }
    }

    Ok(None)
}
