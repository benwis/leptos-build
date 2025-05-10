use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, state::State};

#[derive(Default)]
pub struct CommandOutput {
    command_tx: Option<UnboundedSender<Action>>,
    state: State,
    command_string: String,
    // First vec determines pane # or tab #, second contains lines for the command
    lines: Vec<Vec<String>>,
    scroll: Vec<u16>,
}

impl CommandOutput {
    pub fn new() -> Self {
        let comm_1 = vec!["Hello".to_string()];
        let comm_2 = vec!["World".to_string()];
        Self{
        lines: vec![comm_1, comm_2],
        scroll: vec![0, 0],
        .. Default::default()
    }
}
}

impl Component for CommandOutput {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_state_handler(&mut self, state: State) -> Result<()> {
        self.state = state;
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

        match &self.lines.len(){
            0 => panic!("You must have a command to show a CommandOutput"),
            1 => {
                let layout: [Rect; 1] = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(100)]).areas(area);
                let text = self.lines.get(0).unwrap().join("\n");
                let para = Paragraph::new(text)
                .block(Block::default().title(self.command_string.clone()).borders(Borders::ALL))
                .wrap(Wrap { trim: false })
                .scroll((self.scroll[0], 0));
                frame.render_widget(para, layout[0]);

            }, 
            2 => {
                let layout: [Rect; 2] = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(area);
                let text_0 = self.lines.get(0).unwrap().join("\n");
                let text_1 = self.lines.get(1).unwrap().join("\n");

                let para_0 = Paragraph::new(text_0)
                    .block(Block::default().title(self.command_string.clone()).borders(Borders::ALL))
                    .wrap(Wrap { trim: false })
                    .scroll((self.scroll[0], 0));
                frame.render_widget(para_0, layout[0]);

            
                let para_1 = Paragraph::new(text_1)
                    .block(Block::default().title(self.command_string.clone()).borders(Borders::ALL))
                    .wrap(Wrap { trim: false })
                    .scroll((self.scroll[1], 0));
                frame.render_widget(para_1, layout[1]);
            },
            _ => panic!("2? You can output data from more than 2!")
        }
        
        
        
        Ok(())
    }
}
