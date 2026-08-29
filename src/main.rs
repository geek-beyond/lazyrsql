use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
mod config;

use config::{Config};

const QUIT_SESSION_KEYS: [KeyEvent; 1] = [KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)];

fn main() -> std::io::Result<()> {
    let config = config::ask_config()?;
    ratatui::run(|terminal| app(terminal, &config))
}

fn app(terminal: &mut DefaultTerminal, config: &Config) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if let Some(key) = crossterm::event::read()?.as_key_press_event()
            && should_quit(key)
        {
            break Ok(());
        }
    }
}

fn should_quit(key: KeyEvent) -> bool {
    QUIT_SESSION_KEYS.contains(&key)
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_quit_returns_true_on_q_key() {
        let key_event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        assert!(should_quit(key_event))
    }

    #[test]
    fn should_quit_returns_false_on_non_q_key() {
        let key_event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
        assert!(!should_quit(key_event))
    }
}
