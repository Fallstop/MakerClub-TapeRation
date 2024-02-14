use std::time::Duration;
use dialoguer::{theme::ColorfulTheme, Select, Input};


use futures_util::SinkExt;
use log::info;
use rppal::{gpio::Gpio, system::DeviceInfo};

use tokio::{io::AsyncReadExt, time::sleep};
use console::{style, Style, Term};
use warp::filters::ws::Message;

use crate::actions::State;

use debouncr::{debounce_3, Edge};

const GPIO_REED_SWITCH: u8 = 21;

pub async fn gpio_manager(state: State) -> Result<(), Box<dyn std::error::Error>> {
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

fn better_theme() -> ColorfulTheme {
    ColorfulTheme {
        defaults_style: Style::new(),
        inactive_item_style: Style::new(),
        active_item_style: Style::new().bold(),
        active_item_prefix: style(">".to_string()).for_stderr().bold().green(),
        ..ColorfulTheme::default()
    }
}


pub async fn keyboard_manager(state: State) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let options = vec!["Scan Card", "Remove Card", "Select Length"];

        let action_selection = Select::with_theme(&better_theme())
            .with_prompt(&format!(
                "{}",
                style("Select an action").blue().underlined().bold()
            ))
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();

        match action_selection {
            0 => {
                let card_id: String = Input::new()
                .with_prompt("Card ID")
                .interact_text()
                .unwrap();
                state.lock().await.scan_card(&card_id).await;
            },
            1 => {
                state.lock().await.unscan_card().await;

            },
            2 => {
                let tape_length: usize = Input::new()
                .with_prompt("Tape Length")
                .interact_text()
                .unwrap();
            
                state.lock().await.select_tape_length(tape_length).await;

            },
            _ => {}
        }
    }
}
