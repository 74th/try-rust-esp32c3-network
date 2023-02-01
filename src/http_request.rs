use embedded_svc::{
    http::{client::Client as HttpClient, Method, Status},
    io::Write,
    utils::io,
};
use esp_idf_svc::http::client::{Configuration as HttpConfiguration, EspHttpConnection};

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub timestamp: String,
    pub co2_mhz19c: u32,
    pub co2_ccs811: u32,
    pub tvoc_ccs811: u32,
    pub temperature: u32,
    pub humidity: u32,
    pub location: String,
}

impl Data {
    pub fn to_json(&self, data: Data) -> Result<String> {
        let j = serde_json::to_string(&data)?;
        Ok(j)
    }
}

fn setup_client() -> HttpClient<EspHttpConnection> {
    HttpClient::wrap(
        EspHttpConnection::new(&HttpConfiguration {
            crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach), // Needed for HTTPS support
            ..Default::default()
        })
        .unwrap(),
    )
}

pub fn main() -> anyhow::Result<()> {
    // GET
    get_request()?;
    get_request()?;
    get_request()?;
    get_request()?;

    // POST
    post_request()?;

    // POST JSON
    post_json_request()?;

    Ok(())
}

/// Send a HTTP GET request.
/// copy of https://github.com/esp-rs/esp-idf-svc/blob/master/examples/http_request.rs
fn get_request() -> anyhow::Result<()> {
    let mut client = setup_client();
    // Prepare headers and URL
    let headers = [("accept", "text/plain"), ("connection", "close")];
    let url = "http://ifconfig.net/";

    // Send request
    //
    // Note: If you don't want to pass in any headers, you can also use `client.get(url, headers)`.
    let request = client.request(Method::Get, &url, &headers)?;
    println!("-> GET {}", url);
    let mut response = request.submit()?;

    // Process response
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

    // Drain the remaining response bytes
    while body.read(&mut buf)? > 0 {}

    client.release();

    Ok(())
}

/// Send a HTTP POST request.
/// copy of https://github.com/esp-rs/esp-idf-svc/blob/master/examples/http_request.rs
fn post_request() -> anyhow::Result<()> {
    let mut client = setup_client();
    // Prepare payload
    let payload = b"Hello world!";

    // Prepare headers and URL
    let content_length_header = format!("{}", payload.len());
    let headers = [
        ("accept", "text/plain"),
        ("content-type", "text/plain"),
        ("connection", "close"),
        ("content-length", &*content_length_header),
    ];
    let url = "http://192.168.1.182:30500/api/echo";

    // Send request
    let mut request = client.post(&url, &headers)?;
    request.write_all(payload)?;
    request.flush()?;
    println!("-> POST {}", url);
    let mut response = request.submit()?;

    // Process response
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

    // Drain the remaining response bytes
    while body.read(&mut buf)? > 0 {}

    client.release();

    Ok(())
}

fn post_json_request() -> anyhow::Result<()> {
    let mut client = setup_client();

    let data = Data {
        timestamp: "2021-01-01 00:00:00".to_string(),
        co2_mhz19c: 1000,
        co2_ccs811: 1000,
        tvoc_ccs811: 1000,
        temperature: 20,
        humidity: 50,
        location: "test".to_string(),
    };
    let json_str = serde_json::to_string(&data).unwrap();
    print!("json_str: {}", json_str);
    let payload = json_str.as_bytes();
    print!("json_byte: {}", payload.len());

    // Prepare headers and URL
    let content_length_header = format!("{}", payload.len());
    let headers = [
        ("accept", "text/plain"),
        ("content-type", "text/plain"),
        ("connection", "close"),
        ("content-length", &*content_length_header),
    ];
    let url = "http://192.168.1.182:30500/api/echo";

    // Send request
    let mut request = client.post(&url, &headers)?;
    request.write_all(payload)?;
    request.flush()?;
    println!("-> POST {}", url);
    let mut response = request.submit()?;

    // Process response
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

    // Drain the remaining response bytes
    while body.read(&mut buf)? > 0 {}

    client.release();

    Ok(())
}
