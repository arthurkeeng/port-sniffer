use std::{env, net::{IpAddr, TcpStream}};
use std::str::FromStr;
use std::io::{self , Write};
use std::process;
use std::sync::mpsc::{Sender , channel};
use std::thread;


const MAX : u16 = 65535;
struct Arguments{
    flag : String , 
    ipaddr : IpAddr, 
    threads : u16
}

// this implementation will help create a new Argument instance;
impl Arguments{
    // takes a reference to a string and returns a result .
    // this result returns an argument struct if ok or a str with a static lifetime

    fn new(args : &[String]) -> Result<Arguments , &'static str>{
        // the minimum number of cli argument is 2 so if < 2
        if args.len() < 2 {
            return Err("Not enough arguments");
        }
        // the maximum number of cli argument is 4 so if > 4
        else if args.len() > 4 {
            return Err("too many arguments");
        }
        
        // we are checking the first argument . If it can be converted to an ip address
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f){
            // returns the Ok result if the conversion was successful
            // note the flag is a simple ""
            // this is because if the second argument from the commandline was 
            // convertible to an ipaddr, it means there was no flag
            return Ok(Arguments{
                flag : String::from("") , ipaddr , threads : 4
            })

        }
        // if the conversion was not successful, it means the second argument
        // was not an ipaddress and is probably a flag
        else{

            let flag = args[1].clone();
            // we check if this flag contains h or help and the length of the
            // argument is = 2
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2{
                println!("Usage : -j to select how many threads you want");
                println!("-h or -help to show this help message");
                return Err("help");
            }
            // this else if implies that it contains h or help but it is > 2
            // we just return an error
            else if flag.contains("-h") || flag.contains("-help"){
                return Err("too many arguments");

            }
            // if it contains a j, 
            // we match the next argument
            else if flag.contains("-j"){
                // we check if the 4th argument is convertible to an ipaddress
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    // returns the ip if convertible
                    Ok(s) => s , 
                    // returns an error
                    Err(_) => return Err("not a valid ip address : must be ipv4 or ipb6")
                };
                // next we check if the third argument is parseable to a number

                let threads = match args[2].parse::<u16>() {
                    // we return the number if it can be parsed
                    Ok(n) => n , 
                    // else we return an error
                    Err(_) => return Err("could not parse thread argument into an integer")
                };
                // after all these checks we return the argument
                return Ok(Arguments{
                    flag , ipaddr , threads
                });
            }
            // else , the user typed an invalid syntax
            else{
                return Err("Invalid syntax")
            }
            
        }
       
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
   let program = args[0].clone();
   let arguments = Arguments::new(&args).unwrap_or_else(|err|{
        if err.contains("help"){
            process::exit(0);
        }else{
        eprint!("{} problem parsing arguments : {}" , program , err);
        process::exit(0);
        }
   }
);

    let num_threads = arguments.threads;
    let (tx ,rx ) = channel();
    for i in 0..num_threads{
        let tx = tx.clone();

        thread::spawn(move || {
            scan( tx , i , arguments.ipaddr , num_threads);
        });
    }
    let mut out =  vec![];
        drop(tx);
        for p in rx {
            println!("{}" , p);
            out.push(p);
        }
        println!("");
        out.sort();
        for v in out{
            println!("{} is open " , v);
        }
}

fn scan ( tx : Sender<u16> , start_port : u16 , addr : IpAddr , num_threads : u16){
    let mut port = start_port + 1;
    loop{
        match TcpStream::connect((addr , port)) {
            Ok(_) => {
                println!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
            
        }
        if (MAX - port ) < num_threads{
            break;
        }
        port += num_threads;
    }
}
// ip_sniffer.exe -h
// ip_sniffer.ext -j 100 192.160.1.1
// ip_sniffer.ext 192.168.1.1
