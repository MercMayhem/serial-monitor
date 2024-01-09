pub fn check_ports() {
    let check_ports = serialport::available_ports();
    let ports = match check_ports {
        Ok(res) => res,
        Err(_) => panic!("No ports found!")
    };

    for p in ports{
        println!("{}", p.port_name)
    }
}
