use esp_backtrace as _;
use esp_idf_hal::prelude::Peripherals;
use std::thread::sleep;
use std::time::Duration;

mod http_request;
mod secrets;
mod wifi;

fn main() -> anyhow::Result<()> {
    println!("start");

    let peripherals = Peripherals::take().unwrap();
    let mut conn = wifi::WiFiConn::new(peripherals.modem);
    conn.start();

    http_request::main()?;

    loop {
        sleep(Duration::from_secs(1));
    }

    Ok(())
}
