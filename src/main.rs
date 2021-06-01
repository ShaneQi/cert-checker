use std::process::*;
extern crate clap;
use clap::{App, Arg};
use std::vec::Vec;

const EIGHTY_DAYS: u32 = 6912000;

fn main() {
    let matches = App::new("Cert Checker")
        .version("1.0")
        .author("Shane Qi <qizengtai@gmail.com>")
        .about("Check if certs are expiring.")
        .arg(
            Arg::with_name("server_name")
                .value_name("SERVER NAME")
                .help("One or multiple server names.")
                .takes_value(true)
                .min_values(1)
                .required(true),
        )
        .arg(
            Arg::with_name("expiration")
                .short("e")
                .long("expiration")
                .help("The expiration to check against.")
                .takes_value(true)
        )
        .get_matches();

    let exp = matches
        .value_of("expiration")
        .and_then(|x| x.parse::<u32>().ok())
        .unwrap_or(EIGHTY_DAYS);
    let mut passed = Vec::<String>::new();
    let mut failed = Vec::<String>::new();
    let server_names = matches.values_of("server_name").unwrap();
    for server_name in server_names {
        if check(server_name.to_string(), exp).success() {
            passed.push(server_name.to_string());
        } else {
            failed.push(server_name.to_string());
        }
    }
    println!("====Result====");
    println!("====Expiration used: {}====", exp);
    if !passed.is_empty() {
        println!("PASSED:");
        for server_name in passed {
            println!("{}", server_name);
        }
    }
    if !failed.is_empty() {
        println!("FAILED:");
        for server_name in failed {
            println!("{}", server_name);
        }
    }
}

fn check(server_name: String, exp: u32) -> ExitStatus {
    let cmd = Command::new("echo").stdout(Stdio::piped()).spawn().unwrap();

    let cmd2 = Command::new("/usr/bin/openssl")
        .arg("s_client")
        .arg("-servername")
        .arg(server_name.clone())
        .arg("-connect")
        .arg(format!("{}:443", server_name))
        .stdin(cmd.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let cmd3 = Command::new("openssl")
        .arg("x509")
        .arg("-enddate")
        .arg("-noout")
        .arg("-checkend")
        .arg(format!("{}", exp))
        .stdin(cmd2.stdout.unwrap())
        .output()
        .unwrap();

    cmd3.status
}
