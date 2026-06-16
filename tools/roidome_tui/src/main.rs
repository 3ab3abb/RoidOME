mod app;
mod ui;
mod mqtt;

use app::App;
use ratatui_image::picker::Picker;
use std::sync::{Arc, Mutex};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

// ← sync outer function — NO tokio runtime yet
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Picker query MUST run before tokio AND before ratatui::init()
    let picker = Picker::from_query_stdio()?;
    eprintln!("Protocol: {:?}", picker.protocol_type());

    // now start tokio runtime and hand off the picker
    tokio::runtime::Runtime::new()?.block_on(async_main(picker))
}

async fn async_main(picker: Picker) -> Result<(), Box<dyn std::error::Error>> {
    let app = Arc::new(Mutex::new(App::new_with_picker(picker)));

    // MQTT task
    let app_mqtt = Arc::clone(&app);
    tokio::spawn(async move {
        mqtt::start_mqtt(app_mqtt).await;
    });

    // TUI — ratatui::init() AFTER picker, AFTER tokio starts
    let mut terminal = ratatui::init();
    let app_tui = Arc::clone(&app);

    loop {
        {
            let mut app = app_tui.lock().unwrap();
            if !app.running {
                break;
            }
            terminal.draw(|frame| {
                ui::ui(frame, &mut app);
            })?;
        }

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        app_tui.lock().unwrap().running = false;
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    ratatui::restore();
    Ok(())
}
