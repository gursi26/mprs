use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use ratatui::{prelude::*, widgets::*};
use anyhow::Result;
use rspotify::ClientCredsSpotify;
use std::sync::{Arc, Mutex};
use crate::state::app_state::AppState;
use crate::tui::update::update;
use crate::tui::ui::ui;
use crate::consts::*;
use core::time::Duration;
use std::thread::sleep;

pub fn run<'a>(app_state: Arc<Mutex<AppState<'a>>>, spotify: &mut ClientCredsSpotify) -> Result<()> {
    enable_raw_mode()?;

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let mut t = Terminal::new(CrosstermBackend::new(stdout))?;

    loop {
        let mut curr_app_state_rc = app_state.lock().unwrap();

        update(&mut curr_app_state_rc, Some(spotify), false);

        t.draw(|f| {
            ui(&mut curr_app_state_rc, f);
        })?;

        if curr_app_state_rc.should_quit {
            break;
        }

        drop(curr_app_state_rc);
        sleep(Duration::from_millis(UI_SLEEP_DURATION_MS));
    }

    let mut stdout = std::io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;
    Ok(())
}
