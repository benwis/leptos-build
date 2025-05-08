use ratatui::{layout::{Constraint, Direction, Layout}, widgets::{StatefulWidget, Widget}};

use crate::{state::State, widgets::command_terminal::spawn_command_terminal};

use super::{app_footer::AppFooter, app_header::AppHeader, command_terminal::CommandTerminal};

#[derive(Default, Debug)]
pub struct Shell{state: State, header: AppHeader,footer: AppFooter}

impl Shell{
    pub fn new(state: State, header: AppHeader, footer: AppFooter)-> Self{
        Self{state, header, footer}
    }
}

impl StatefulWidget for &Shell{
    type State = State;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        // TODO: Flesh this out
        //1. This is the shell/template of the layout
        // let command_layout = match &state.command{
        //     crate::state::Commands::Build => todo!(),
        //     _ => todo!(),
        // };
        let layout = Layout::new(Direction::Vertical, [Constraint::Percentage(10), Constraint::Percentage(80), Constraint::Percentage(10)]).split(area);

        // let body = match &state.command.layout{
        //     AppLayout::Single => todo!(),
        //     AppLayout::Double => todo!(),
        //     - => todo!(),
        // };

        let command_terminal = CommandTerminal::new();
        //I feel like this should take an area to spawn it in. TODO: CHECK THAT AND REMOVE THIS UNWRAP
        self.header.render(layout[0], buf);
        spawn_command_terminal(command_terminal, state, layout[1]).unwrap();
        self.footer.render( layout[2], buf);

        //2. Command chosen would determine the internals
        // - AppLayout should be set by the command provided
        
    }
}