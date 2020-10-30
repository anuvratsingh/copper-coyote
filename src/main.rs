// use std::fs::OpenOptions;
use chrono::prelude::*;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

fn current_time() -> String {
    let local: DateTime<Local> = Local::now();
    let time: String = format!(
        "{:02}-{:02}-{} {:02}:{:02}:{:02}",
        local.day(),
        local.month(),
        local.year(),
        local.hour(),
        local.minute(),
        local.second()
    );
    time
}

fn add_to_logs(input: String) {
    let path = "logs.txt";
    let log = format!("{} -> {}", current_time(), input); //Dependend on current time
    let mut output = File::create(path).expect("unable to create file");
    // let mut output = OpenOptions::new()
    //         .read(true)
    //         .write(true)
    //         .create(true)
    //         .open("foo.txt");

    output.write_all(log.as_bytes()).expect("unable to write");
    // if let Err(e) = writeln!(output, "{} -> {}", current_time(), input) {
    //     eprintln!("Couldn't write to file: {}", e)
    // }
}

fn main() -> Result<(), Error> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let port: u16 = 6969;
    let socket = SocketAddrV4::new(loopback, port);
    let listener = TcpListener::bind(socket)?;

    add_to_logs(format!("Server is listening at port {}", port));
    println!("{}: Server is listening at port {}", current_time(), port);

    let (mut tcp_stream, addr) = listener.accept()?;

    add_to_logs(format!("Connection received! {:?} is sending data.", addr)); //To Logs
    println!("Connection received! {:?} is sending data.", addr);
    let mut input = String::new();
    let _ = tcp_stream.read_to_string(&mut input)?;

    add_to_logs(format!("{:?} says {}", addr, input)); // TO LOGS
    println!("{:?} says {}", addr, input);

    Ok(())
}
