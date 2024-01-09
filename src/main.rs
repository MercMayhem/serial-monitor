use std::time::Duration;
use std::io::Write;
use serialport;

fn main() {
    let open_port_config = serialport::new("/dev/cu.usbmodem1201", 9600).timeout(Duration::from_secs(10));
    let open_port_result = open_port_config.open();

    let mut port = match open_port_result {
                    Ok(res) => res,
                    Err(_) => panic!("Couldn't open the port")
                };
    
    println!("Flow control: {}", port.flow_control().unwrap());
    println!("Data bits: {}", port.data_bits().unwrap());
    println!("Parity: {}", port.parity().unwrap());
    println!("Stop bits: {}", port.stop_bits().unwrap());

    loop {
      let mut serial_buf: [u8; 8] = [0; 8];
        port.read(&mut serial_buf[..]).expect("Found No data");
        let output = String::from_utf8(serial_buf.to_vec()).unwrap();
        print!("{}",  output);
        let _ = std::io::stdout().flush();
    }
}
