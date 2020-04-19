#[macro_use]
extern crate nom;

#[macro_use]
extern crate log;

mod account;
mod packets;

fn main() -> std::io::Result<()> {
    env_logger::init();

    account::server::run();

    Ok(())
}
