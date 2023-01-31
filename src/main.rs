use esp_backtrace as _;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_sys;
use std::time::Duration;

use embedded_svc::{
    http::{client::Client as HttpClient, Method, Status},
    utils::io,
};
use esp_idf_svc::http::client::{Configuration as HttpConfiguration, EspHttpConnection};

mod secrets;
mod wifi;

fn main() -> anyhow::Result<()> {
    println!("start");

    let peripherals = Peripherals::take().unwrap();
    let mut conn = wifi::WiFiConn::new(peripherals.modem);
    conn.start();

    let mut client = HttpClient::wrap(EspHttpConnection::new(&HttpConfiguration {
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach), // Needed for HTTPS support
        ..Default::default()
    })?);

    let url = "http://192.168.1.182:30500/api/ip";
    let request = client.request(Method::Get, &url, &[])?;
    println!("-> GET {}", url);
    let mut response = request.submit()?;
    let status = response.status();
    println!("<- {}", status);
    println!();

    let (_headers, mut body) = response.split();
    let mut buf = [0u8; 1024];
    let bytes_read = io::try_read_full(&mut body, &mut buf).map_err(|e| e.0)?;
    println!("Read {} bytes", bytes_read);
    match std::str::from_utf8(&buf[0..bytes_read]) {
        Ok(body_string) => println!(
            "Response body (truncated to {} bytes): {:?}",
            buf.len(),
            body_string
        ),
        Err(e) => eprintln!("Error decoding response body: {}", e),
    };

    while body.read(&mut buf)? > 0 {}

    Ok(())
}
