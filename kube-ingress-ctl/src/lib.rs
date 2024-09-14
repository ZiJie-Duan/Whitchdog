use std::{process::Command, str::FromStr};

pub fn get_ingress_white_list(ingress_name: &str, name_space: &str) -> Vec<String> {
    let mut op = Command::new("kubectl")
        .arg("get")
        .arg("ingress")
        .arg(ingress_name)
        .arg("-n")
        .arg(name_space)
        .arg("-o")
        .arg("jsonpath='{.metadata.annotations.nginx\\.ingress\\.kubernetes\\.io/whitelist-source-range}'")
        .output()
        .expect("failed to execute process");

    let result = String::from_utf8_lossy(&op.stdout);
    let result: Vec<String> = result.split(", ").map(|x| x.to_string()).collect();
    return result;
}
