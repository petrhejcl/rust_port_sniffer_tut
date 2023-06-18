use std::{env, io, process, thread};
use std::io::Write;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};

const MAX: u16 = 65535;

struct Arguments {
    ip_address: IpAddr,
    threads: u16
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }
        let f = args[1].clone();
        if let Ok(ip_address) = IpAddr::from_str(&f) {
            return Ok(Arguments{ip_address, threads: 4 })
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") {
                if args.len() == 2 {
                    println!("Use -j to specify how many threads you want or just put IP address without flags");
                    return Err("help");
                } else {
                    return Err("Too many arguments");
                }
            } else if flag.contains("-j") {
                let ip_address = match IpAddr::from_str(&args[3]){
                    Ok(s) => s,
                    Err(_) => return Err("IP Adress is not valid")
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Incorrect number of threads")
                };
                return Ok(Arguments{ip_address, threads });
            } else {
                return Err("Wrong arguments");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, ip_address: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((ip_address, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) < num_threads {
            break;
        }

        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
        }
    );
    let threads_num = arguments.threads;
    let address = arguments.ip_address;
    let (tx, rx) = channel();
    for i in 0..threads_num {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, address, threads_num);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!();
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}
