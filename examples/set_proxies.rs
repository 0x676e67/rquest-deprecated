use reqwest::impersonate::Impersonate;
use reqwest_impersonate as reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build a client to mimic Chrome126
    let mut client = reqwest::Client::builder()
        .impersonate(Impersonate::Chrome126)
        .enable_ech_grease()
        .permute_extensions()
        .cookie_store(true)
        .build()?;

    let resp = client.get("https://api.ip.sb/ip").send().await?;
    println!("{}", resp.text().await?);

    let proxy = reqwest::Proxy::all("socks5h://127.0.0.1:1080")?;
    client.set_proxies(vec![proxy]);

    let resp = client.get("https://api.ip.sb/ip").send().await?;
    println!("{}", resp.text().await?);

    Ok(())
}
