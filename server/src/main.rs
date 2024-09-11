use std::net::SocketAddr;
use std::net::UdpSocket;
use std::process::Command;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

struct User {
    addr: SocketAddr,
    time: SystemTime,
}

fn iptables_ctrl() {
    let output = Command::new("ls")
        .arg("-l") // 传递参数
        .output() // 执行命令并获取输出
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Command output: \n{}", stdout);
}

fn user_update(users: &mut Vec<User>) {
    for user in users.iter_mut() {
        match SystemTime::now().duration_since(user.time) {
            Ok(duration) => {
                if duration.le(&Duration::from_secs(20)) {
                    println!("time touch`");
                }
            }
            Err(e) => println!("Error {:?}", e),
        }
    }
}

fn user_in(addr: SocketAddr, users: &mut Vec<User>) {
    let mut exist: bool = false;
    for user in users.iter_mut() {
        if user.addr == addr {
            user.time = SystemTime::now();
            exist = true;
        }
    }
    if !exist {
        let new_user: User = User {
            addr: addr,
            time: SystemTime::now(),
        };
        users.push(new_user);
    }
}

fn recv_socket(socket: UdpSocket, users: &mut Vec<User>) {
    let mut buf = [0; 10];
    let (amt, src) = socket.recv_from(&mut buf).unwrap();
}

fn main() -> std::io::Result<()> {
    let users: Vec<User> = Vec::new();
    let socket = UdpSocket::bind("127.0.0.1:34254")?;

    loop {
        user_update(&mut users);
    recv_socket(socket,
    }
    Ok(())
}
