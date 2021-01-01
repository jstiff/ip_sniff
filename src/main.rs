// this is a code along from 'Rustlang Project: Port Sniffer CLI' produced by
// Tensor Programming. 
// https://www.youtube.com/watch?v=-Jp7sabBCp4&list=PLJbE2Yu2zumDD5vy2BuSHvFZU0a6RDmgb
// this command line script with have three arguments in the shape of. 
// 1. ip_sniff.exe -h ...this will provide a help page to stdOut
// 2. ip_sniff.exe -j 100 192.168.1.1 .....this says 100 threads at 192 ...ipaddress
// 3. ip_sniff.exe 192.168.1.1 ...will use defaul number of thread for that ip.

// bring in env namespace to allow us to pull out the arguments from the command line. 

use std::env;
use std::io::{self, Write}; 
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;  //FromStr is a trait
use std::process; 
use std::sync::mpsc::{Sender, channel};
use std::thread; 

const MAX: u16 = 65535;

// IpAddr is an enum that is either ip4 or ip6 types. 
struct Arguments {
    flag: String, 
    ipaddr: IpAddr,
    threads: u16, 
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        else if args.len() > 4 {
            return Err("too many arguments");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f){
            return Ok(Arguments { flag: String::from(""), ipaddr, threads: 4})
        }else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2{
                println!("Usage help yo: -j to select how many threads you want \r\n or -h & -help to show this help page which you just did!");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help"){
                return Err("too many arguments with help...if you wanted help just use -h alone")
            }else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]){
                    Ok(s) => s,
                    Err(_) => return Err("must be a valid Ip4 or Ip6 address")
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread count!")
                };
                return Ok(Arguments{threads, flag, ipaddr});
            }else {
                return Err("invalid syntax....EVErything else failed ...line 51")
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16){
    let mut port: u16 = start_port +1;
    loop {
        match TcpStream::connect((addr, port)){
            Ok(_) => {
                print!("*found one!*");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }
        if (MAX - port) <=  num_threads {
            break;
        }
        port += num_threads;
    }

}

fn main() {
    // collect() will take any iterator and make a collection out of it. It's a methog on the Iterator Trait. 
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            }else {
                eprintln!("{} problem parsing arguments: {} ", program, err);
                process::exit(0);
            }
        }
    );

    let num_threads = arguments.threads; 
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn( move || {
            scan(tx, i , addr, num_threads);
        });
    }
    
    let mut out = vec![];
    drop(tx);    // drop tx from main thread...exist only in other threads. ???
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("port: {} is open right now!", v);
    }

}
