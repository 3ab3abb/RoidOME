mod app;
mod ui;
mod mqtt;

use app::App;
use std::sync::{Arc, Mutex};
use crossterm::event::{self, Event, KeyCode};

use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Arc::new(Mutex::new(App::new()));

    // MQTT runs in async background task
    let app_mqtt = Arc::clone(&app);
    tokio::spawn(async move {
        mqtt::start_mqtt(app_mqtt).await;
    });

    // TUI runs directly in main — no spawn_blocking
    let mut terminal = ratatui::init();
    let app_tui = Arc::clone(&app);

    loop {
        {
            let app = app_tui.lock().unwrap();
            if !app.running {
                break;
            }
            terminal.draw(|frame| ui::ui(frame, &app)).unwrap();
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

