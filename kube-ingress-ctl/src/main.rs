use std::net::IpAddr;
use std::net::SocketAddr;
use std::str::FromStr;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

struct User {
    addr: SocketAddr,
    time: SystemTime,
}

struct UserList {
    users: Vec<User>,
}

impl UserList {
    fn update(&mut self) {
        println!("Update");
        let mut index = 0;
        loop {
            match self.users.get(index) {
                Some(user) => match SystemTime::now().duration_since(user.time) {
                    Ok(dura) => {
                        if dura.lt(&Duration::from_secs(3)) {
                            println!("Ok {:?}", dura);
                            index += 1;
                        } else {
                            println!("Timeout {:?}", dura);

                            self.users.remove(index);
                        }
                    }
                    Err(e) => {
                        self.users.remove(index);
                        println!("Err {:?}", e);
                    }
                },
                None => {
                    break;
                }
            }
        }
    }

    fn add(&mut self, addr: SocketAddr) {
        let user = User {
            addr: addr,
            time: SystemTime::now(),
        };
        self.users.push(user);
    }
}

fn main() {
    let mut users = UserList { users: Vec::new() };

    users.add(SocketAddr::new(
        IpAddr::from_str("123.123.123.123").unwrap(),
        234,
    ));

    users.add(SocketAddr::new(
        IpAddr::from_str("123.123.123.123").unwrap(),
        234,
    ));

    users.update();
    users.update();
    sleep(Duration::from_secs(3));
    users.update();
    users.update();
}
