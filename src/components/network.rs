use neli::{
    consts::{nl::NlmF, rtnl::{Arphrd, Ifla, Iff, RtAddrFamily, Rtm}, socket::NlFamily},
    err::NlError,
    nl::Nlmsghdr,
    rtnl::Ifinfomsg,
    socket::NlSocket,
    types::RtBuffer,
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
    let mut socket = NlSocket::connect(NlFamily::Route, None, &[])?;

    let index = get_interface_index(iface)?
        .ok_or_else(|| NlError::Msg("Interface not found".into()))?;

    let mut attrs = RtBuffer::new();

    let ifmsg = Ifinfomsg::new(
        RtAddrFamily::Unspecified,
        Arphrd::Ether,
        index as i32,
        if up { Iff::Up.bits() } else { 0 },
        Iff::Up.bits(),
        attrs,
    );

    let nlhdr = Nlmsghdr::new(
        None,
        Rtm::Newlink,
        NlmF::REQUEST | NlmF::ACK,
        None,
        None,
        ifmsg,
    );

    socket.send(&nlhdr)?;
    let mut buf = Vec::new();
    socket.recv(&mut buf)?;

    Ok(())
}

fn get_interface_index(name: &str) -> Result<Option<u32>, NlError> {
    let mut socket = NlSocket::connect(NlFamily::Route, None, &[])?;

    let ifmsg = Ifinfomsg::new(
        RtAddrFamily::Unspecified,
        Arphrd::Ether,
        0,
        0,
        0,
        RtBuffer::new(),
    );

    let nlhdr = Nlmsghdr::new(
        None,
        Rtm::Getlink,
        NlmF::REQUEST | NlmF::DUMP,
        None,
        None,
        ifmsg,
    );

    socket.send(&nlhdr)?;
    let mut buf = Vec::new();

    loop {
        if let Some(response) = socket.recv::<Ifinfomsg>(&mut buf)? {
            let attrs = response.nl_payload.rtattrs();
            for attr in attrs.iter() {
                if attr.rta_type == Ifla::Ifname {
                    if let Ok(n) = String::from_utf8(attr.rta_payload.clone()) {
                        if n == name {
                            return Ok(Some(response.nl_payload.ifi_index as u32));
                        }
                    }
                }
            }
        } else {
            break;
        }
    }

    Ok(None)
}
