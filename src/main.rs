#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate regex;
extern crate serde_json;

mod list_vm;

use std::process::Command;
use regex::Regex;

#[get("/")]
fn index() -> String {
    let output =    Command::new("VBoxManage")
                    .args(&["list", "vms"])
                    .output()
                    .expect("Unable to execute process");
    let output_string = String::from_utf8(output.stdout).unwrap();
    let error_string = String::from_utf8(output.stderr).unwrap();
    
    let re = Regex::new(
        r####""(.*)" \{([a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12})"####
    ).unwrap();
    
    for line in output_string.lines(){
        if re.is_match(line) {
            println!("Match!");
            let cap = re.captures(line).unwrap();
            println!("Name: {}\nID: {}", &cap[1], &cap[2]);
        }
    }
    
    
    println!("{:?}", error_string);
    output_string
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
