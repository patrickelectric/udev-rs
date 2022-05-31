extern crate udev;

use std::io;

fn main() -> io::Result<()> {
    let mut enumerator = udev::Enumerator::new()?;

    dbg!(1);
    enumerator.match_property("DEVNAME", "/dev/ttyUSB1")?;
    if let Some(device) = enumerator.scan_devices().unwrap().next() {
        dbg!(device.devnode());
        for property in device.properties() {
            println!("    - {:?} {:?}", property.name(), property.value());
        }

        println!("  [attributes]");
        for attribute in device.attributes() {
            println!("    - {:?} {:?}", attribute.name(), attribute.value());
        }
    }

    dbg!(2);
    enumerator.match_property("DEVNAME", "/dev/ttyUSB0")?;
    if let Some(device) = enumerator.scan_devices().unwrap().next() {
        dbg!(device.devnode());
        for property in device.properties() {
            println!("    - {:?} {:?}", property.name(), property.value());
        }

        println!("  [attributes]");
        for attribute in device.attributes() {
            println!("    - {:?} {:?}", attribute.name(), attribute.value());
        }
    }

    Ok(())
}
