mod discord_client;
mod model;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    let mut client = discord_client::Client::new("968700214837936168")?;

    client.connect()?;

    Ok(())
}
