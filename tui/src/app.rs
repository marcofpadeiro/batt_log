use std::error::Error;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::Backend,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Paragraph, Widget},
    Terminal,
};
use rusqlite::Connection;
use tui::SessionList;

pub struct App {
    items: SessionList,
}

impl App {
    pub fn new(conn: Connection) -> Self {
        Self {
            items: SessionList::new(conn),
        }
    }

    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> Result<(), Box<dyn Error>> {
        loop {
            self.draw(&mut terminal)?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    use KeyCode::*;
                    match key.code {
                        Char('q') | Esc => return Ok(()),
                        Char('j') | Down => self.items.next(),
                        Char('k') | Up => self.items.previous(),
                        Char('l') | Left => self.items.previous_page(),
                        Char('h') | Right => self.items.next_page(),
                        Char('g') => self.items.go_top(),
                        Char('G') => self.items.go_bottom(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<(), Box<dyn Error>> {
        terminal.draw(|f| f.render_widget(self, f.size()))?;
        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ]);
        let [header_area, rest_area, footer_area] = vertical.areas(area);

        let horizontal =
            Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)]);
        let [left_area, right_area] = horizontal.areas(rest_area);

        let vertical = Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)]);
        let [graph_area, info_area] = vertical.areas(left_area);

        let vertical = Layout::horizontal([Constraint::Length(2), Constraint::Min(0)]);
        let [mode_area, list_area] = vertical.areas(right_area);

        render_title(header_area, buf);
        self.render_graph(graph_area, buf);
        self.render_info(info_area, buf);
        self.render_mode(mode_area, buf);
        self.render_list(list_area, buf);
        render_footer(footer_area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Batt_log").centered().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Paragraph::new("\nUse ↓↑ or j/k to move, g/G to go top/bottom.")
        .centered()
        .render(area, buf);
}
