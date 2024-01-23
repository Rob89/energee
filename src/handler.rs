use crate::app::{App, AppResult, GroupBy};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        KeyCode::Right => {
            app.next_meter().await?;
        }
        KeyCode::Left => {
            app.previous_meter().await?;
        }
        KeyCode::Char('d') | KeyCode::Char('D') => {
            if !matches!(app.group_by, GroupBy::Day) {
                app.group_by = GroupBy::Day;
                app.load_data().await?;
            }
        }
        KeyCode::Char('h') | KeyCode::Char('H') => {
            if !matches!(app.group_by, GroupBy::Hour) {
                app.group_by = GroupBy::Hour;
                app.load_data().await?;
            }
        }
        KeyCode::Char('a') | KeyCode::Char('A') => {
            if !matches!(app.group_by, GroupBy::HalfHour) {
                app.group_by = GroupBy::HalfHour;
                app.load_data().await?;
            }
        }
        KeyCode::Char('w') | KeyCode::Char('W') => {
            if !matches!(app.group_by, GroupBy::Week) {
                app.group_by = GroupBy::Week;
                app.load_data().await?;
            }
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
