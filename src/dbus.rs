use anyhow::Result;
use zbus::{dbus_proxy, export::zvariant::OwnedObjectPath};

#[dbus_proxy(
    interface = "org.freedesktop.systemd1.Manager",
    default_service = "org.freedesktop.systemd1",
    default_path = "/org/freedesktop/systemd1"
)]
pub trait SystemdManager {
    #[dbus_proxy(name = "GetUnitByPID")]
    fn get_unit_by_pid(&self, pid: u32) -> Result<OwnedObjectPath>;
}

#[dbus_proxy(
    default_service = "org.freedesktop.systemd1",
    interface = "org.freedesktop.systemd1.Service"
)]
pub trait Service {
    #[dbus_proxy(property)]
    fn memory_current(&self) -> zbus::Result<u64>;
}

#[dbus_proxy(
    default_service = "org.freedesktop.systemd1",
    interface = "org.freedesktop.systemd1.Scope"
)]
pub trait Scope {
    #[dbus_proxy(property)]
    fn memory_current(&self) -> zbus::Result<u64>;
}

#[dbus_proxy(
    default_service = "org.freedesktop.systemd1",
    interface = "org.freedesktop.systemd1.Unit"
)]
pub trait Unit {
    #[dbus_proxy(property)]
    fn id(&self) -> zbus::Result<String>;
}

pub const SD_INTERFACE_SERVICE: &str = "org.freedesktop.systemd1.Service";
pub const SD_INTERFACE_SCOPE: &str = "org.freedesktop.systemd1.Scope";
