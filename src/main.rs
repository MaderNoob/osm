mod osm;

use anyhow::Context;
use thiserror::Error;

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {}", error)
    }
}

fn run() -> anyhow::Result<()> {
    println!("{:?}", osm::get_domain_list().unwrap());
    let mail_addr = "a@esiix.com".parse().unwrap();
    println!("{:?}", osm::get_messages(&mail_addr));
    Ok(())
}
