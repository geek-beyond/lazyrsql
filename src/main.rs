use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};

const QUIT_SESSION_KEYS: [KeyEvent; 1] = [KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)];

fn main() -> std::io::Result<()> {
    ratatui::run(app)
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
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
