use ratatui::{layout::{Constraint, Direction, Layout}, widgets::StatefulWidget};

use crate::state::State;

#[derive(Default, Debug)]
pub struct Shell{state: State};

impl Shell{
    pub fn new(state: State)-> Self{
        Self{state}
    }
}

impl StatefulWidget for &Shell{
    type State = State;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        // TODO: Flesh this out
        //1. This is the shell/template of the layout
        let layout = Layout::new(Direction::Vertical, [Constraint::Percentage(10), Constraint::Percentage(80), Constraint::Percentage(10)]);

        let body = match &state.command.layout{
            AppLayout::Single => todo!(),
            AppLayout::Double => todo!(),
            - => todo!(),
        };

        //2. Command chosen would determine the internals
        // - AppLayout should be set by the command provided
        todo!()
    }
}