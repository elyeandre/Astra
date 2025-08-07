use neli::{
    consts::{nl::NlmF, rtnl::RtAddr, socket::NlFamily},
    err::NlError,
    nl::Nlmsghdr,
    rtnl::{Ifinfomsg, Rtmsg},
    socket::tokio::NlSocket,
    utils::Groups,
};
use tokio::runtime::Handle;

/// Network component for interface control
pub struct NetworkComponent;

impl NetworkComponent {
    pub async fn register_to_lua(lua: &mlua::Lua) -> mlua::Result<()> {
        // Get or create Astra table
        let astra_table: mlua::Table = if let Ok(table) = lua.globals().get::<_, mlua::Table>("Astra") {
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
    let handle = Handle::current();
    handle
        .spawn_blocking(move || blocking_set_link_state(iface, up))
        .await
        .map_err(|e| mlua::Error::runtime(e.to_string()))?
        .map_err(|e| mlua::Error::runtime(e.to_string()))
}

fn blocking_set_link_state(iface: &str, up: bool) -> Result<(), NlError> {
    // Create netlink socket
    let mut socket = NlSocket::new(NlFamily::Route)?;

    // Build interface message
    let ifmsg = Ifinfomsg::new(
        RtAddr::Unspecified,   // family
        0,                     // link layer type
        0,                     // interface index (will be set later)
        0,                     // flags
        Groups::empty(),       // change mask
    );

    // Create netlink header
    let mut nlhdr = Nlmsghdr::new(
        None,
        if up {
            neli::consts::rtnl::Rtm::Setlink
        } else {
            neli::consts::rtnl::Rtm::Dellink
        },
        NlmF::REQUEST | NlmF::ACK,
        None,
        None,
        ifmsg,
    );

    // Set interface index
    if let Some(idx) = get_interface_index(iface)? {
        nlhdr.nl_payload.ifi_index = idx as i32;
    } else {
        return Err(NlError::Msg("Interface not found".into()));
    }

    // Send request
    socket.send(nlhdr)?;

    // Wait for ACK
    match socket.recv::<_, Rtmsg>() {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn get_interface_index(name: &str) -> Result<Option<u32>, NlError> {
    let mut socket = NlSocket::new(NlFamily::Route)?;
    
    let ifmsg = Ifinfomsg::new(
        RtAddr::Unspecified,
        0,
        0,
        0,
        Groups::empty(),
    );
    
    let nlhdr = Nlmsghdr::new(
        None,
        neli::consts::rtnl::Rtm::Getlink,
        NlmF::REQUEST | NlmF::DUMP,
        None,
        None,
        ifmsg,
    );
    
    socket.send(nlhdr)?;
    
    for msg in socket.iter::<Ifinfomsg>() {
        let msg = msg?;
        if let Some(ifname) = msg.ifa_label {
            if ifname == name {
                return Ok(Some(msg.ifi_index as u32));
            }
        }
    }
    
    Ok(None)
}
