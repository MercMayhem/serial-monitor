mod check_ports;

use std::env;
use std::time::Duration;
use std::io::Write;
use serialport;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().position(|arg| arg == "-c") != None{
        
        check_ports::check_ports();

    } else if args.iter().position(|arg| arg == "-h") != None{
        
        println!("use \"-c\" to list ports and identify the arduino");

    } else if args.iter().len() > 1 {
        
        let open_port_config = serialport::new(&args[1], 9600).timeout(Duration::from_secs(10));
        let open_port_result = open_port_config.open();

        let mut port = match open_port_result {
                        Ok(res) => res,
                        Err(e) => panic!("Couldn't open the port: {}", e)
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
}
