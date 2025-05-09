use color_eyre::Result;
use ratatui::{prelude::*, symbols::border, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct CommandOutput {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    command_string: String,
    lines: Vec<String>,
    scroll: u16,
}

impl CommandOutput {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for CommandOutput {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        
        let text = self.lines.join("\n");
        
        let para = Paragraph::new(text)
            .block(Block::default().title(self.command_string.clone()).borders(Borders::ALL))
            .wrap(Wrap { trim: false })
            .scroll((self.scroll, 0));
        frame.render_widget(para, area);
        Ok(())
    }
}
