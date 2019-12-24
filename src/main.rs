#![feature(async_closure)]
#![recursion_limit = "1024"]

use std::error::Error;
use std::fmt::Display;
use std::io;

dirmod::all!();

fn ctx<D: Display, E: Error>(context: D) -> impl Fn(E) -> io::Error {
    move |err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("error during {}: {}", &context, err).as_str(),
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    let secrets = Secrets::try_new().map_err(ctx("loading secrets"))?;
    let index = index::Index::try_new(&secrets).map_err(ctx("connecting to database"))?;
    let bridge = discord::Bridge::try_new(&secrets, &index).map_err(ctx("starting discord bot"))?;
    web::run(secrets, index, bridge).map_err(ctx("starting web server"))?;
    Ok(())
}
