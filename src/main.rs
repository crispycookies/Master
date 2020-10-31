use std::net::{UdpSocket};
use std::env;
use std::time::{Duration, Instant};
use std::io::{Write};
use std::path::Path;
use std::fs::File;

fn main() -> std::io::Result<()> {
    {
        let mut iterator = 0;
        let args: Vec<String> = env::args().collect();
        if args.len() != 3{
            panic!("Invalid Count of Arguments Provided")
        }
        let send_packages = 50000;
        print!("Connecting\n");
        let timeout = Duration::from_millis(10000);
        let socket = UdpSocket::bind(args.get(1).unwrap().to_string()).expect("Could not connect to Device, is it correct?");
        let _timout_socket = socket.set_read_timeout(Option::from(timeout));
        let _timout_socket = socket.set_write_timeout(Option::from(timeout));

        let _block = socket.set_nonblocking(false);
        let _c = socket.connect(args.get(2).unwrap().to_string());

        let path = Path::new("Timings.csv");
        let mut file = File::create(&path).expect("Could not create File");

        let mut package_drop_counter = 0;

        print!("Running\n");

        for _i in 0..send_packages {
            // Receives a single datagram message on the socket. If `buf` is too small to hold
            // the message, it will be cut off.
            //let buf_send = [255; 16];
            let buf_send = [255; 16];
            let mut buf_read = [0; 16];
            let time = Instant::now();

            match socket.send_to(&buf_send, args.get(2).unwrap().to_string()){
                Err(_e) => {

                }
                _ => { match socket.recv_from(&mut buf_read){
                    Err(_e) => {
                        package_drop_counter = package_drop_counter + 1;
                    }
                    _ => {}
                }}
            }


            //socket.send_to(&buf_send, &src).expect("Failed to send the message!");

            let elapsed = time.elapsed();

            if buf_read != buf_send {
                //print!("[WARNING]Message sent and received dont match\n");
                package_drop_counter = package_drop_counter + 1;
            }else{
                let elapsed_millis =  elapsed.as_millis().to_string();
                //print!("[NOTE]Received Message; RTT -> {}ms\n", elapsed_millis);

                let builder = iterator.to_string() + "," + elapsed_millis.as_str() + "\n";

                iterator = iterator + 1;
                let _write = file.write(builder.as_bytes());
            }
            // Redeclare `buf` as slice of the received data and send reverse data back to origin.

        }
        print!("Packages Drops: {}\n", package_drop_counter);
        print!("Packages Not Drops: {}\n", iterator);
    } // the socket is closed here
    Ok(())
}

