use std::time::Duration;
use dialoguer::{theme::ColorfulTheme, Select, Input};


use futures_util::SinkExt;
use log::info;
use rppal::{gpio::Gpio, system::DeviceInfo};

use tokio::{io::AsyncReadExt, time::sleep};
use console::{style, Style, Term};
use warp::filters::ws::Message;

use crate::actions::State;




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
        let options = vec!["Scan Card", "Select Length"];

        let action_selection = Select::with_theme(&better_theme())
            .with_prompt(&format!(
                "{}",
                style("Select an action").blue().underlined().bold()
            ))
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();

        match options[action_selection] {
            "Select Length" => {
                let tape_length: f32 = Input::new()
                    .with_prompt("Tape Length")
                    .interact_text()
                    .unwrap();

                state.lock().await.select_tape_length(tape_length).await;
            },
            "Scan Card" => {
                let card_id: String = Input::new()
                .with_prompt("Card ID")
                .interact_text()
                .unwrap();
                state.lock().await.scan_card(&card_id).await;
            },
            _ => {}
        }
    }
}
