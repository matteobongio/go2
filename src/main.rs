use std::{io::{stderr, Stderr}, path::PathBuf};
use clap::Parser;
use cli::{Args, Commands};
use color_eyre::Result;
mod cli;
mod config;
use config::get_paths;
use ratatui::{
    crossterm::{
        execute,
        terminal::{enable_raw_mode, EnterAlternateScreen},
    }, prelude::*, restore, widgets::{BorderType, Borders, List, ListState}
};
use rustic_fuzz::fuzzy_sort_in_place;

use crossterm::event::{self, KeyCode};
use ratatui::{
    Frame,
    widgets::{Block, Paragraph},
};

#[derive(Debug, Default)]
pub struct App {
    search: String,
    paths: Vec<String>,
    selected: Option<usize>,
    exit: bool,
}

impl App {
    fn new() -> Self {
        let search = String::new();
        let paths = get_paths();
        let selected = { if paths.is_empty() { None } else { Some(0) } };
        let exit = false;
        Self {
            search,
            paths,
            selected,
            exit,
        }
    }

    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stderr>>) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events();
        }
        Ok(())
    }

    fn handle_events(&mut self) {
        let e = event::read().expect("can't read events");
        if let Some(key) = e.as_key_press_event() {
            match key.code {
                KeyCode::Char(c) => {
                    self.search.push(c);
                    fuzzy_sort_in_place(&mut self.paths, &self.search);
                }
                KeyCode::Up => {
                    if self.paths.is_empty() {
                        return;
                    }
                    if self.selected.is_none() {
                        self.selected = Some(1);
                        return;
                    }
                    if let Some(mut n) = self.selected {
                        if n == 0 {
                            n = self.paths.len() - 1;
                        } else {
                            n -= 1;
                        }
                        self.selected = Some(n);
                    }
                }
                KeyCode::Down => {
                    if self.paths.is_empty() {
                        return;
                    }
                    if self.selected.is_none() {
                        self.selected = Some(1);
                        return;
                    }
                    if let Some(mut n) = self.selected {
                        n += 1;
                        n %= self.paths.len();
                        self.selected = Some(n);
                    }
                }
                KeyCode::Backspace => {
                    self.search.pop();
                    fuzzy_sort_in_place(&mut self.paths, &self.search);
                }
                KeyCode::Enter => {
                    if let Some(selection) = self.selected {
                        println!("{}", self.paths[selection]);
                    } else {
                        println!(".");
                    }
                    self.exit = true;
                }
                KeyCode::Tab => {
                    open::that(config::get_config()).unwrap();
                }
                KeyCode::Esc => self.exit = true,
                _ => {}
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let search_bar = Paragraph::new(self.search.as_str()).block(
            Block::new()
                .title("Search")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

        let mut state = ListState::default().with_selected(self.selected);

        let paths = List::new(self.paths.clone()) // TODO: don t like this
            .block(Block::bordered().title("GOTO"))
            // .highlight_style(Style::new().reversed())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        let help = Paragraph::new("tab: edit").block(
            Block::new()
                .title("Help")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Percentage(100),
                Constraint::Length(3),
            ])
            .split(frame.area());
        frame.render_widget(search_bar, layout[0]);
        frame.render_stateful_widget(paths, layout[1], &mut state);
        frame.render_widget(help, layout[2]);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    if let Some(Commands::Add { path }) = args.command {
        if let Some(add) = path.as_deref() {
            let path = PathBuf::from(add);
            let path = realpath::realpath(&path).unwrap();
            config::add_path(path.display().to_string() + "\n");
            Ok(())
        } else {
            eprintln!("missing path to add.");
            Ok(())
        }
    } else {
        // init
        set_panic_hook();
        enable_raw_mode()?;
        execute!(stderr(), EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stderr());
        let mut terminal = Terminal::new(backend).unwrap();
        let result = App::new().run(&mut terminal);
        ratatui::restore();
        result
    }
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |info| {
        restore();

        hook(info);
    }));
}
