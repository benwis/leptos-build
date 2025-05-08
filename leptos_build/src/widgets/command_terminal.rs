use std::{io::{self, BufWriter}, sync::mpsc::channel};

use ratatui::{crossterm::{event::{self, Event, KeyCode, KeyEventKind}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}}, prelude::{Backend, CrosstermBackend}, style::{Modifier, Style}, text::Line, widgets::{Block, Borders}, Frame, Terminal};
use tui_term::{vt100::Screen, widget::PseudoTerminal};
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};

use crate::state::State;

pub struct CommandTerminal;

impl CommandTerminal{
    /// Create a new empty struct
    pub fn new() -> Self{
        Self
    }
    /// Configure the UI for the CommandTerminal Widget
    fn ui(f: &mut Frame, screen: &Screen, title: &str)
    where
        Self: Sized {
    let title = format!("[ Running: {} ]", title);
    let title = Line::from(title);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Style::default().add_modifier(Modifier::BOLD));

    let pseudo_term = PseudoTerminal::new(screen).block(block.clone());
    f.render_widget(pseudo_term, f.area());

    }
    /// Configure the terminal to correctly intercept stdout, create a new backend for it, and size correctly
    fn setup_terminal(&self) -> io::Result<(Terminal<CrosstermBackend<BufWriter<io::Stdout>>>, TerminalSize)> {
        enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(BufWriter::new(stdout));
        let mut terminal = Terminal::new(backend)?;
        let initial_size = terminal.size()?;
        let size = TerminalSize {
            rows: initial_size.height,
            cols: initial_size.width,
        };
        execute!(terminal.backend_mut(), EnterAlternateScreen)?;
        Ok((terminal, size))
    }
    
    fn cleanup_terminal(&self,
        terminal: &mut Terminal<CrosstermBackend<BufWriter<io::Stdout>>>,
    ) -> io::Result<()> {
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
        terminal.show_cursor()?;
        terminal.clear()?;
        Ok(())
    }
    /// Runs the CommandTerminal in a loop. This seems a little divorced from the standard Ratatui flow
    fn run<B: Backend>(&self, terminal: &mut Terminal<B>, screen: &Screen, title: &str) -> io::Result<()> {
        loop {
            terminal.draw(|f| CommandTerminal::ui(f, screen, title))?;
    
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if let KeyCode::Char('q') = key.code {
                        return Ok(());
                    }
                }
            }
        }
    }
}

    /// Spawns a new terminal with all the fixings to run the command
    pub fn spawn_command_terminal(ct: CommandTerminal, state: &State)-> eyre::Result<()>{
        let (mut terminal, size) = ct.setup_terminal().unwrap();

        let pty_system = NativePtySystem::default();
        let cwd = std::env::current_dir().unwrap();
        let mut cmd = CommandBuilder::new("ls");
        cmd.cwd(cwd);
    
        let pair = pty_system
            .openpty(PtySize {
                rows: size.rows,
                cols: size.cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .unwrap();
        let mut child = pair.slave.spawn_command(cmd).unwrap();
        drop(pair.slave);
    
        let (tx, rx) = channel();
        let mut reader = pair.master.try_clone_reader().unwrap();
        let mut parser = vt100::Parser::new(size.rows - 1, size.cols - 1, 0);
    
        std::thread::spawn(move || {
            // Consume the output from the child
            let mut s = String::new();
            reader.read_to_string(&mut s).unwrap();
            tx.send(s).unwrap();
        });
    
        {
            // Drop writer on purpose
            let _writer = pair.master.take_writer().unwrap();
        }
    
        // Wait for the child to complete
        let _child_exit_status = child.wait().unwrap();
    
        drop(pair.master);
    
        let output = rx.recv().unwrap();
        parser.process(output.as_bytes());
    
        (&ct).run(&mut terminal, parser.screen(), "Command Title")?;
    
        // restore terminal
        ct.cleanup_terminal(&mut terminal).unwrap();
        Ok(())
    }

#[derive(Debug, Clone)]
struct TerminalSize {
    cols: u16,
    rows: u16,
}