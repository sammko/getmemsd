mod dbus;

use std::str;
use std::str::FromStr;

use anyhow::{Context, Result};
use number_prefix::NumberPrefix;
use zbus::xml::Node;

fn get_focus_pid() -> Result<u32> {
    let cmd_output = std::process::Command::new("xdotool")
        .arg("getactivewindow")
        .arg("getwindowpid")
        .output();

    let stdout = match cmd_output {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(e).context("xdotool is required"),
        Err(e) => Err(e.into()),
        Ok(x) => Ok(x),
    }?
    .stdout;

    str::from_utf8(&stdout)?
        .trim()
        .parse::<u32>()
        .context("Invalid PID")
}

struct UnitInfo {
    id: String,
    memory_bytes: u64,
}

fn get_unit_info_pid(conn: zbus::Connection, pid: u32) -> Result<UnitInfo> {
    let sd_proxy = dbus::SystemdManagerProxy::new(&conn)?;
    let obj = sd_proxy.get_unit_by_pid(pid).context("No service")?;

    let intro = zbus::fdo::IntrospectableProxy::new_for(&conn, "org.freedesktop.systemd1", &obj)?;
    let intro_node = Node::from_str(&intro.introspect()?)?;

    let mem_bytes = intro_node
        .interfaces()
        .iter()
        .find_map(|i| match i.name() {
            dbus::SD_INTERFACE_SERVICE => {
                Some(dbus::ServiceProxy::new_for_path(&conn, &obj).and_then(|s| s.memory_current()))
            }
            dbus::SD_INTERFACE_SCOPE => {
                Some(dbus::ScopeProxy::new_for_path(&conn, &obj).and_then(|s| s.memory_current()))
            }
            _ => None,
        })
        .context("Only .service and .scope units are supported.")??;

    let unit_id = dbus::UnitProxy::new_for_path(&conn, &obj)?.id()?;

    Ok(UnitInfo {
        id: unit_id,
        memory_bytes: mem_bytes,
    })
}

fn main() -> Result<()> {
    let conn = zbus::Connection::new_session()?;

    let pid = get_focus_pid()?;
    let uinfo = get_unit_info_pid(conn, pid)?;

    println!(
        "{}: {}",
        uinfo.id,
        match NumberPrefix::binary(uinfo.memory_bytes as f64) {
            NumberPrefix::Standalone(bytes) => format!("{:.2} B", bytes),
            NumberPrefix::Prefixed(prefix, n) => format!("{:.2} {}B", n, prefix),
        }
    );

    Ok(())
}
