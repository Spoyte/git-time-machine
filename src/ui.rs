use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;

use crate::git::{CommitInfo, GitExplorer};

pub struct App {
    explorer: GitExplorer,
    commits: Vec<CommitInfo>,
    selected: usize,
    mode: Mode,
}

enum Mode {
    Timeline,
    CommitDetail,
}

impl App {
    pub fn new(explorer: GitExplorer) -> Result<Self> {
        let commits = explorer.get_commit_history()?;
        Ok(Self {
            explorer,
            commits,
            selected: 0,
            mode: Mode::Timeline,
        })
    }

    pub fn run(mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = self.run_app(&mut terminal);

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        res
    }

    fn run_app(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => {
                        if self.selected < self.commits.len().saturating_sub(1) {
                            self.selected += 1;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if self.selected > 0 {
                            self.selected -= 1;
                        }
                    }
                    KeyCode::Enter => {
                        self.mode = Mode::CommitDetail;
                    }
                    KeyCode::Char('t') => {
                        self.mode = Mode::Timeline;
                    }
                    _ => {}
                }
            }
        }
    }

    fn ui(&self,
        frame: &mut ratatui::Frame<CrosstermBackend<io::Stdout>>,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.size());

        match self.mode {
            Mode::Timeline => self.render_timeline(frame, chunks[0]),
            Mode::CommitDetail => self.render_detail(frame, chunks[0]),
        }

        let help = Paragraph::new("j/k: navigate | Enter: view | t: timeline | q: quit")
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(help, chunks[1]);
    }

    fn render_timeline(
        &self,
        frame: &mut ratatui::Frame<CrosstermBackend<io::Stdout>>,
        area: ratatui::layout::Rect,
    ) {
        let items: Vec<ListItem> = self
            .commits
            .iter()
            .enumerate()
            .map(|(i, commit)| {
                let time = chrono::DateTime::from_timestamp(commit.time, 0)
                    .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                    .unwrap_or_else(|| "???".to_string());
                
                let content = Line::from(vec![
                    Span::styled(
                        format!("{} ", commit.short_id),
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::styled(
                        format!("{} ", time),
                        Style::default().fg(Color::Cyan),
                    ),
                    Span::raw(&commit.message),
                ]);

                let style = if i == self.selected {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                };

                ListItem::new(content).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().title("Git Timeline").borders(Borders::ALL));

        frame.render_widget(list, area);
    }

    fn render_detail(
        &self,
        frame: &mut ratatui::Frame<CrosstermBackend<io::Stdout>>,
        area: ratatui::layout::Rect,
    ) {
        let commit = &self.commits[self.selected];
        let time = chrono::DateTime::from_timestamp(commit.time, 0)
            .map(|dt| dt.to_rfc2822())
            .unwrap_or_else(|| "???".to_string());

        let text = Text::from(vec![
            Line::from(vec![
                Span::styled("Commit: ", Style::default().fg(Color::Yellow)),
                Span::raw(&commit.id),
            ]),
            Line::from(vec![
                Span::styled("Author: ", Style::default().fg(Color::Yellow)),
                Span::raw(&commit.author),
            ]),
            Line::from(vec![
                Span::styled("Date:   ", Style::default().fg(Color::Yellow)),
                Span::raw(time),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Message:", Style::default().fg(Color::Yellow)),
            ]),
            Line::raw(&commit.message),
        ]);

        let paragraph = Paragraph::new(text)
            .block(Block::default().title("Commit Detail").borders(Borders::ALL));

        frame.render_widget(paragraph, area);
    }
}
