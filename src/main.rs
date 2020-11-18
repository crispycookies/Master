use std::net::{UdpSocket};
use std::env;
use std::time::{Duration, Instant};
use std::io::{Write};
use std::path::Path;
use std::fs::File;

struct Rval {
    packages_dropped : u32,
    packages_survived : u32,
    timings : Vec<u128>
}

fn store(timings: Vec<u128>, filename: &str) {
    let path = Path::new(filename);
    let mut file = File::create(&path).expect("Could not create File");
    for i in 0..timings.len() {
        let builder = i.to_string() + "," + &*timings[i as usize].to_string();
        let _write = file.write(builder.as_bytes());
    }
}

fn run(socket: UdpSocket, send_packages: u32, addr : &str) -> Rval {
    let mut ret_val : Rval = Rval{packages_dropped : 0, packages_survived : 0, timings : Vec::with_capacity(send_packages as usize)};

    for i in 0..send_packages {
        let buf_send = "ping ping ping  ".as_bytes();
        let mut buf_read = [0; 16];
        let time = Instant::now();
        let mut double_count = false;

        let elapsed = time.elapsed();
        match socket.send_to(&buf_send, addr) {
            Err(_e) => {}
            _ => {
                match socket.recv_from(&mut buf_read) {
                    Err(_e) => {
                        ret_val.packages_dropped = ret_val.packages_dropped + 1;
                        double_count = true;
                    }
                    _ => {}
                }
            }
        }

        if buf_read != buf_send {
            if !double_count {
                ret_val.packages_dropped = ret_val.packages_dropped + 1;
            }
        } else {
            ret_val.timings[i as usize] = elapsed.as_millis();
            ret_val.packages_survived = ret_val.packages_survived + 1;
        }
    }
    return ret_val;
}

fn main() -> std::io::Result<()> {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() != 5 {
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

        print!("Running\n");

        let result = run(socket, send_packages, args.get(2).unwrap().to_string().as_str());

        print!("Packages Dropped: {}\n", result.packages_dropped);
        print!("Packages Not Dropped: {}\n", result.packages_survived);

        print!("Saving File\n");

        let _ = store(result.timings, "Timings.csv");
    }
    Ok(())
}

