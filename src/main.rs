use std::process::*;

const ninty_days: u32 = 7776000;
const thirty_days: u32 = 2592000;

fn main() {
    let res1 = check("shaneqi.com".to_string(), ninty_days);
    let res2 = check("shaneqi.com".to_string(), thirty_days);
    println!("{}", res1);
    println!("{}", res2);
}

fn check(ns: String, exp: u32) -> ExitStatus {
    let cmd = Command::new("echo").stdout(Stdio::piped()).spawn().unwrap();

    let cmd2 = Command::new("/usr/bin/openssl")
        .arg("s_client")
        .arg("-servername")
        .arg("ns")
        .arg("-connect")
        .arg(format!("{}:443", ns))
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
