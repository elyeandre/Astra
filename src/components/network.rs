use neli::{
    consts::{nl::Nlmsg, rtnl::{Arphrd, Ifla, IflaInfo, IflaInfoData, Iff, RtAddrFamily, Rtm}, socket::NlFamily, NlmF},
    err::NlError,
    nl::{NlPayload, Nlmsghdr},
    rtnl::{Ifinfomsg, Rtattr},
    socket::NlSocketHandle,
    types::RtBuffer,
    utils::U32Bitmask,
};
use tokio::task;

pub struct NetworkComponent;

impl NetworkComponent {
    pub async fn register_to_lua(lua: &mlua::Lua) -> mlua::Result<()> {
        let astra_table: mlua::Table = if let Ok(table) = lua.globals().get("Astra") {
            table
        } else {
            let table = lua.create_table()?;
            lua.globals().set("Astra", table.clone())?;
            table
        };

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

    let index = get_interface_index(iface)?
        .ok_or_else(|| NlError::Msg("Interface not found".into()))?;

    let flags = if up {
        Iff::Up
    } else {
        Iff::empty()
    };

    let ifi_flags = U32Bitmask::from(flags);
    let change_mask = U32Bitmask::from(Iff::Up);

    let ifmsg = Ifinfomsg {
        ifi_family: RtAddrFamily::Unspecified,
        ifi_type: Arphrd::Ether,
        ifi_index: index as i32,
        ifi_flags,
        ifi_change: change_mask,
    };

    let nlhdr = Nlmsghdr::new(
        None,
        Rtm::Newlink,
        NlmF::Request | NlmF::Ack,
        None,
        None,
        NlPayload::Payload(ifmsg),
    );

    socket.send(nlhdr)?;
    socket.recv()?; // Wait for ACK

    Ok(())
}

fn get_interface_index(name: &str) -> Result<Option<u32>, NlError> {
    let mut socket = NlSocketHandle::connect(NlFamily::Route, None, &[])?;

    let ifmsg = Ifinfomsg::default();

    let nlhdr = Nlmsghdr::new(
        None,
        Rtm::Getlink,
        NlmF::Request | NlmF::Dump,
        None,
        None,
        NlPayload::Payload(ifmsg),
    );

    socket.send(nlhdr)?;

    while let Some(response) = socket.recv()? {
        if let NlPayload::Payload(msg) = response.nl_payload {
            for attr in msg.rtattrs.iter() {
                if attr.rta_type == Ifla::Ifname {
                    if let Ok(n) = String::from_utf8(attr.rta_payload.clone()) {
                        if n == name {
                            return Ok(Some(msg.ifi_index as u32));
                        }
                    }
                }
            }
        }
    }

    Ok(None)
}
