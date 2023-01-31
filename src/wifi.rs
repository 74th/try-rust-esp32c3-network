use std::thread::sleep;
use std::time::Duration;

use embedded_svc::wifi::Wifi;
use esp_idf_hal;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_sys;

use crate::secrets;

pub struct WiFiConn<'a> {
    wifi_driver: esp_idf_svc::wifi::EspWifi<'a>,
}

impl<'a> WiFiConn<'a> {
    pub fn new(modem: esp_idf_hal::modem::Modem) -> Self {
        unsafe {
            esp_idf_sys::nvs_flash_init();
        }

        let sysloop = EspSystemEventLoop::take().unwrap();
        let nvs = esp_idf_svc::nvs::EspDefaultNvsPartition::take().unwrap();

        esp_idf_sys::link_patches();

        let mut wifi_driver = esp_idf_svc::wifi::EspWifi::new(modem, sysloop, Some(nvs)).unwrap();
        wifi_driver
            .set_configuration(&embedded_svc::wifi::Configuration::Client(
                embedded_svc::wifi::ClientConfiguration {
                    ssid: secrets::WIFI_SSID.into(),
                    password: secrets::WIFI_PASSWORD.into(),
                    ..Default::default()
                },
            ))
            .unwrap();

        Self {
            wifi_driver: wifi_driver,
        }
    }

    pub fn start(&mut self) {
        self.wifi_driver.start().unwrap();
        self.wifi_driver.connect().unwrap();

        sleep(Duration::new(1, 0));

        while !self.wifi_driver.is_connected().unwrap() {
            let config = self.wifi_driver.get_configuration().unwrap();
            println!("Waiting for station {:?}", config);
            sleep(Duration::from_secs(1));
        }

        sleep(Duration::new(1, 0));

        println!(
            "IP info: {:?} {:?}",
            self.wifi_driver.sta_netif().get_ip_info().unwrap(),
            self.wifi_driver.is_connected(),
        );
    }
}
