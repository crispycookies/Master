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
        if args.len() != 5{
            panic!("Invalid Count of Arguments Provided")
        }
        let send_packages = args.get(4).unwrap().parse::<u32>().expect("No Valid Sample Size Provided");
        print!("Connecting\n");
        let timeout = Duration::from_millis(args.get(3).unwrap().parse::<u64>().expect("No Valid Timeout Provided"));
        let socket = UdpSocket::bind(args.get(1).unwrap().to_string()).expect("Could not connect to Device");
        let _timout_socket = socket.set_read_timeout(Option::from(timeout));
        let _timout_socket = socket.set_write_timeout(Option::from(timeout));

        let _block = socket.set_nonblocking(false);
        let _c = socket.connect(args.get(2).unwrap().to_string());

        let path = Path::new("Timings.csv");
        let mut file = File::create(&path).expect("Could not create File");

        let mut package_drop_counter = 0;

        let mut timings = Vec::with_capacity(send_packages as usize);

        print!("Running\n");

        for i in 0..send_packages {
            let buf_send = "ping ping ping  ".as_bytes();
            let mut buf_read = [0; 16];
            let time = Instant::now();
            let mut double_count = false;

            match socket.send_to(&buf_send, args.get(2).unwrap().to_string()){
                Err(_e) => {

                }
                _ => { match socket.recv_from(&mut buf_read){
                    Err(_e) => {
                        package_drop_counter = package_drop_counter + 1;
                        double_count = true;
                    }
                    _ => {}
                }}
            }

            let elapsed = time.elapsed();

            if buf_read != buf_send {
                if !double_count {
                    package_drop_counter = package_drop_counter + 1;
                }
            }else{
                iterator = iterator + 1;

                timings[i as usize] = elapsed.as_millis();

                //let _write = file.write(builder.as_bytes());
            }
        }
        print!("Packages Dropped: {}\n", package_drop_counter);
        print!("Packages Not Dropped: {}\n", iterator);

        print!("Saving File\n");

        for i in 0..timings.len() {
            let builder = i.to_string() + "," + &*timings[i as usize].to_string();
            let _write = file.write(builder.as_bytes());
        }
    }
    Ok(())
}

