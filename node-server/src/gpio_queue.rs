use std::time::Duration;

use log::info;
use rppal::{gpio::Gpio, system::DeviceInfo};
use tokio::{sync::mpsc::Receiver, time::sleep};

use crate::actions::State;


pub async fn gpio_manager(state: State, queue_receiver: &mut Receiver<f32>) -> Result<(), Box<dyn std::error::Error>> {
    // info!(
    //     "Loading GPIO, running on {}",
    //     DeviceInfo::new()
    //         .expect("Failed to load Device info, likely not running on a PI")
    //         .model()
    // );

    // let gpio = Gpio::new()?;
    // let mut tape_motor = gpio.get(17)?.into_output();


    while let Some(tape_length) = queue_receiver.recv().await {
    }

    Ok(())
}
