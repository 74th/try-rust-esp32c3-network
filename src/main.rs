use esp_backtrace as _;
use esp_idf_hal::prelude::Peripherals;

mod http_request;
mod secrets;
mod wifi;

fn main() -> anyhow::Result<()> {
    println!("start");

    let peripherals = Peripherals::take().unwrap();
    let mut conn = wifi::WiFiConn::new(peripherals.modem);
    conn.start();

    http_request::main()?;

    Ok(())
}
