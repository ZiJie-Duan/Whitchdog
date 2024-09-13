use std::{process::Command, string};

fn 

fn main() {
    let mut op = Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output()
        .expect("failed to execute process");

    let tt = op
        .stdout
        .iter_mut()
        .map(|&mut x| x as char)
        .collect::<Vec<char>>()
        .iter()
        .collect::<String>();

    let opstr = op
        .stdout
        .iter()
        .map(|&x| x - 1)
        .collect::<Vec<u8>>()
        .iter()
        .map(|&x| x as char)
        .collect::<String>();
    println!("{}", String::from_utf8_lossy(&op.stdout));
    println!("{}", opstr);
    println!("{}", tt);
}
