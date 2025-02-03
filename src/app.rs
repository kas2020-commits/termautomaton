use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget, DefaultTerminal, Frame};

use crate::{automata::CA, grid::Grid, rulesets::conways_game_of_life, state::State};

#[derive(Debug)]
pub struct App {
    ca: CA<State>,
    exit: bool,
}

impl App {
    pub fn new(grid: Grid<State>) -> Self {
        let ca = CA::new(grid);
        Self { ca, exit: false }
    }

    fn on_tick(&mut self) {
        self.ca.advance(conways_game_of_life);
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        let tick_rate = Duration::from_millis(50);
        let mut last_tick = Instant::now();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());

            // Poll to ensure the read won't block
            if event::poll(timeout)? {
                self.handle_events()?;
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
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

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for x in 0..area.width {
            for y in 0..area.height {
                let idx = ((y * area.width) + x) as usize;
                let b = self.ca.grid.at(x as i16, y as i16);
                match *b {
                    State::Alive => {
                        buf.content[idx].set_bg(Color::White);
                    }
                    State::Dead => {
                        buf.content[idx].set_bg(Color::Reset);
                    }
                }
            }
        }
    }
}
