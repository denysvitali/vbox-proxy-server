#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate regex;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod list_vm;

use std::process::Command;
use regex::Regex;

use list_vm::ListingVM;


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
    
    let mut listing : Vec<ListingVM> = Vec::new();
    
    for line in output_string.lines(){
        if re.is_match(line) {
            println!("Match!");
            let cap = re.captures(line).unwrap();
            println!("Name: {}\nID: {}", &cap[1], &cap[2]);
            let listing_entry = ListingVM {
                id: String::from(&cap[2]),
                name: String::from(&cap[1])
            };
            
            listing.push(listing_entry);
        }
    }
    
    
    println!("{:?}", error_string);
    println!("{:?}", listing);
    serde_json::to_string(&listing).unwrap()
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
