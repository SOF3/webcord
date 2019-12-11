use std::error::Error;

dirmod::all!();

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    pretty_env_logger::init();
    let secrets = Secrets::try_new()?;
    let bridge = discord::Bridge::try_new(&secrets)?;
    web::run(secrets, bridge)?;
    Ok(())
}
