use std::time::Duration;

use futures_util::SinkExt;
use log::info;
use rppal::{gpio::Gpio, system::DeviceInfo};

use tokio::{io::AsyncReadExt, time::sleep};
use warp::filters::ws::Message;

use crate::websocket::Users;

use debouncr::{debounce_3, Edge};

const GPIO_REED_SWITCH: u8 = 21;

pub async fn gpio_manager(users: Users) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "Loading GPIO, running on {}",
        DeviceInfo::new()
            .expect("Failed to load Device info, likely not running on a PI")
            .model()
    );
    let reed_pin = Gpio::new()?.get(GPIO_REED_SWITCH)?.into_input_pullup();

    // Requires 3 samples to be the same to change state, so the switch has to be stable for 7.5ms
    let mut debouncer = debounce_3(false);

    // Magnet inside is two-polled, meaning it triggers twice very fast.
    // Rather than debouncing, we poll very fast to capture both triggers, and only send every second poll
    let mut alternating_switch = true;

    loop {
        let new_state = reed_pin.is_low();
        match debouncer.update(new_state) {
            Some(Edge::Rising) => {
                if alternating_switch {
                    info!("Reed switch opened");
                    for user in users.lock().await.values_mut() {
                        user.send(Message::text("click")).await?;
                    }
                    alternating_switch = false;
                } else {
                    alternating_switch = true;
                }
            }
            _ => {}
        }
        sleep(Duration::from_micros(2500)).await;
    }
}
pub async fn keyboard_manager(users: Users) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdin = tokio::io::stdin();
    let mut buf: [u8; 1] = [0];
    loop {
        stdin
            .read_exact(&mut buf)
            .await
            .expect("Failed to read from stdin");
        for user in users.lock().await.values_mut() {
            user.send(Message::text("click")).await?;
        }
    }
}
