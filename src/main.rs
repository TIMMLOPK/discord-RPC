mod discord_client;
mod model;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    let mut client = discord_client::Client::new("968700214837936168")?;

    client.connect()?;

    loop {
        let activity = model::Activity::new().state("Hello, World!").details("I'm a bot!");

        client.set_activity(activity)?;

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
