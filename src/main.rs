use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};

fn main() {
    color_eyre::install().unwrap();
    ratatui::run(app).unwrap();
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let key_event_q = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.as_key_press_event() == Some(key_event_q) {
            break Ok(());
        }
        }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
