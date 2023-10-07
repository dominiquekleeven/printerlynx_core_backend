use tokio_serial::available_ports;
use tracing::info;

pub fn check_serial_connections() -> bool {
    let ports = available_ports().expect("No ports found!");

    // It should also be noted that on macOS, both the Callout (/dev/cu.*) and Dial-in ports (/dev/tty.*)
    // ports are enumerated, resulting in two available ports per connected serial device.

    info!("Found {} serial ports", ports.len());
    true
}
