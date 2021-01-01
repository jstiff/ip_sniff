// this is a code along from 'Rustlang Project: Port Sniffer CLI' produced by
// Tensor Programming. 
// https://www.youtube.com/watch?v=-Jp7sabBCp4&list=PLJbE2Yu2zumDD5vy2BuSHvFZU0a6RDmgb
// this command line script with have three arguments in the shape of. 
// 1. ip_sniff.exe -h ...this will provide a help page to stdOut
// 2. ip_sniff.exe -j 100 192.168.1.1 .....this says 100 threads at 192 ...ipaddress
// 3. ip_sniff.exe 192.168.1.1 ...will use defaul number of thread for that ip.

// bring in env namespace to allow us to pull out the arguments from the command line. 

use std::env;

fn main() {
    // collect() will take any iterator and make a collection out of it. It's a methog on the Iterator Trait. 
    let args: Vec<String> = env::args().collect();

    for argument in &args {
        println!("Arguments passed: {:?}", argument)
    }
}
