use kube_ingress_ctl::get_ingress_white_list;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

struct User {
    addr: String,
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

    fn add(&mut self, addr: String) {
        let user = User {
            addr: addr,
            time: SystemTime::now(),
        };
        self.users.push(user);
    }
}

fn main() {
    let mut users = UserList { users: Vec::new() };
    let users_ip: Vec<String> = get_ingress_white_list("back-main-ingress", "dev");
    users_ip.into_iter().for_each(|x| users.add(x));
    users.update();
    users.update();
    sleep(Duration::from_secs(3));
    users.update();
    users.update();
}
