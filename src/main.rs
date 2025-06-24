pub fn main() {
    let ports = serialport::available_ports().expect("could not retrieved serial ports");

    for port in ports {
        println!("{:?}", port);
    }
}
