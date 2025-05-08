use std::sync::Arc;

use eyre::Result;
use parking_lot::RwLock;
use crate::state::State;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, StatefulWidget, Widget},
    DefaultTerminal, Frame,
};

use super::shell::Shell;

// Determines the needed layout for each command
#[derive(Debug, Clone, Default)]
pub enum AppLayout{Single, Double, #[default]Default}

#[derive(Debug, Clone, Default)]
pub struct App{
    pub state: Arc<RwLock<State>>,
    pub exit: bool,
    pub layout: AppLayout,
}

impl App{
    /// runs the application's main loop until the user quits
    /// structure for widget rendering is main() -> run() -> draw() on terminal -> draw() on App -> render() on the widgets as defined in the Layout
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
    while !self.exit {
        terminal.draw(|frame| self.draw(frame))?;
        self.handle_events()?;
    }
    Ok(())
    }
    // draw will create the layout of the App and begin rendering of each widget in assigned areas
    fn draw(&self, frame: &mut Frame) {
        let layout = Layout::new(Direction::Horizontal,[Constraint::Percentage(50), Constraint::Percentage(50)]).split(frame.area());
        frame.render_widget(self, layout[0]);
        frame.render_widget(self, layout[1]);

    }

        /// updates the application's state based on user input
        fn handle_events(&mut self) -> Result<()> {
            match event::read()? {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
            Ok(())
        }
    
        // ANCHOR: handle_key_event fn
        fn handle_key_event(&mut self, key_event: KeyEvent) {
            match key_event.code {
                KeyCode::Char('q') => self.exit(),
                _ => {}
            }
        }


    fn exit(&mut self) {
        self.exit = true;
    }

}

impl Widget for &App {
    // render is what the App should look like 
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = self.state.write();


        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            "YES".to_string().yellow(),
        ])]);

        let p = Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);

        let shell: Shell = Shell::new();

        StatefulWidget::render(&shell, area, buf, &mut state);
    }
}