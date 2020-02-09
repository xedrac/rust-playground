use remote_control::*;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::time::delay_for;

mod remote_control;

#[tokio::main]
async fn main() {
    let remote = RemoteControl::new(32);
    let tx = remote.sender.clone();
    remote.start_event_loop();
    test_remote_control(tx).await;
}

/*
use tokio::runtime::Builder;

// The tokio::main macro makes this a lot cleaner, but I wanted to know what it
// looks like if we don't control the main function from within Rust.
fn main() {
    let remote = RemoteControl::new(32);
    let tx = remote.sender.clone();

    let mut rt = Builder::new()
        .core_threads(1)
        .max_threads(1)
        .enable_all()
        .thread_name("input_thread")
        .basic_scheduler()
        .threaded_scheduler()
        .build()
        .expect("Failed to create tokio runtime!");

    // Start the event loop from within the tokio runtime
    rt.block_on(async {
        remote.start_event_loop();
        test_remote_control(tx.clone()).await;
    });
}
*/

async fn test_remote_control(mut tx: Sender<UiEvent>) {
    send_event(&mut tx, UiEvent::PowerButton).await;
    send_event(&mut tx, UiEvent::VolumeButton(Direction::Up)).await;
    send_event(&mut tx, UiEvent::VolumeButton(Direction::Down)).await;
    send_event(&mut tx, UiEvent::ChannelButton(Direction::Up)).await;
    send_event(&mut tx, UiEvent::ChannelButton(Direction::Down)).await;
    send_event(&mut tx, UiEvent::InputButton).await;
    send_event(&mut tx, UiEvent::InputButton).await;
    send_event(&mut tx, UiEvent::ScheduleSleep(3000)).await;
    send_event(&mut tx, UiEvent::ScheduleSleep(1000)).await;
    delay_for(Duration::from_secs(5)).await;
    send_event(&mut tx, UiEvent::Terminate).await;
}

async fn send_event(sender: &mut Sender<UiEvent>, event: UiEvent) {
    if sender.send(event).await.is_err() {
        println!("failed to send event: {:?}", event);
    }
}
