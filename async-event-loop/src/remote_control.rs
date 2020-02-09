use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::time::delay_for;


#[derive(Debug, Clone, Copy)]
pub enum UiEvent {
    PowerButton,
    VolumeButton(Direction),
    ChannelButton(Direction),
    InputButton,
    ScheduleSleep(u64),
    Sleep,
    Terminate,
}

#[derive(Debug, Clone, Copy)]
pub enum Input {
    TV,
    HDMI1,
    HDMI2,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
}

pub struct RemoteControl {
    pub sender: Sender<UiEvent>,
    receiver: Receiver<UiEvent>,
    power: bool,
    input: Input,
    sleeping: bool,
}

impl RemoteControl {
    pub fn new(queue_size: usize) -> Self {
        let (sender, receiver) = channel(queue_size);
        Self {
            sender,
            receiver,
            power: false,
            input: Input::TV,
            sleeping: false,
        }
    }

    pub fn start_event_loop(mut self) {
        tokio::spawn(async move {
            while let Some(e) = self.receiver.recv().await {
                self.process_event(e);
                if let UiEvent::Terminate = e {
                    break;
                }
            }
        });
    }

    fn process_event(&mut self, event: UiEvent) {
        match event {
            UiEvent::PowerButton => {
                self.power = !self.power;
                println!("Powering {}", if self.power { "on" } else { "off" });
            }
            UiEvent::VolumeButton(d) => {
                if self.power {
                    println!(
                        "{} volume",
                        match d {
                            Direction::Up => "Increasing",
                            Direction::Down => "Decreasing",
                        }
                    );
                } else {
                    println!("Cannot adjust volume, power is off");
                }
            }
            UiEvent::ChannelButton(d) => {
                if self.power {
                    println!(
                        "{} channel",
                        match d {
                            Direction::Up => "Increasing",
                            Direction::Down => "Decreasing",
                        }
                    );
                } else {
                    println!("Cannot adjust channel, power is off");
                }
            }
            UiEvent::InputButton => {
                if self.power {
                    self.input = match self.input {
                        Input::TV => Input::HDMI1,
                        Input::HDMI1 => Input::HDMI2,
                        Input::HDMI2 => Input::TV,
                    };
                    println!("Changing input to {:?}", self.input);
                } else {
                    println!("Cannot adjust input, power is off");
                }
            }
            UiEvent::ScheduleSleep(ms) => {
                if self.power {
                    println!("Sleep scheduled in {} ms", ms);
                    self.delayed_event(ms, UiEvent::Sleep);
                } else {
                    println!("Cannot schedule sleep, power is off");
                }
            }
            UiEvent::Sleep => {
                if self.power {
                    if self.sleeping {
                        println!("Already asleep...");
                    } else {
                        println!("Sleeping...");
                        self.sleeping = true;
                    }
                } else {
                    println!("Cannot sleep, power is off");
                }
            }
            UiEvent::Terminate => {
                println!("Shutting down!");
            }
        }
    }

    fn delayed_event(&mut self, delay_ms: u64, event: UiEvent) {
        let mut sender = self.sender.clone();
        tokio::spawn(async move {
            delay_for(Duration::from_millis(delay_ms)).await;
            if sender.send(event).await.is_err() {
                println!("failed to send event");
            }
        });
    }
}
