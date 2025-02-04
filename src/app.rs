use std::cmp;
use std::io;
use std::time::{Duration, Instant};

use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget, DefaultTerminal, Frame};

use crate::{
    automata::Automata, grid::Grid, rulesets::conways_game_of_life, states::BasicCellState,
};

#[derive(Debug)]
pub struct TerminalApp {
    ca: Automata<BasicCellState>,
    offset_x: u16,
    offset_y: u16,
    max_offset_x: u16,
    max_offset_y: u16,
    tick_rate: Duration,
    is_paused: bool,
    exit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl TerminalApp {
    pub fn new(grid: Grid<BasicCellState>, size: Size) -> Self {
        let ca = Automata::new(grid);
        assert!(
            size.width <= ca.grid.width as u16,
            "Grid width must be at least as big as viewport"
        );
        assert!(
            size.height <= ca.grid.height as u16,
            "Grid height be at least as big as viewport"
        );
        let max_offset_x = (ca.grid.width as u16) - size.width;
        let max_offset_y = (ca.grid.height as u16) - size.height;
        let tick_rate = Duration::from_millis(50);
        Self {
            ca,
            tick_rate,
            offset_x: 0,
            offset_y: 0,
            max_offset_x: if max_offset_x == 0 {
                0
            } else {
                max_offset_x - 1
            },
            max_offset_y: if max_offset_y == 0 {
                0
            } else {
                max_offset_y - 1
            },
            is_paused: false,
            exit: false,
        }
    }

    fn on_tick(&mut self) {
        if !self.is_paused {
            self.ca.advance(conways_game_of_life);
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        let mut last_tick = Instant::now();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = self.tick_rate.saturating_sub(last_tick.elapsed());

            // Poll to ensure the read won't block
            if event::poll(timeout)? {
                self.handle_events()?;
            }

            if last_tick.elapsed() >= self.tick_rate {
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

    fn play_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }

    fn offset(&mut self, direction: Direction, multiplier: u16) {
        match direction {
            Direction::Left => {
                if self.offset_x > multiplier {
                    self.offset_x -= multiplier;
                }
            }
            Direction::Right => {
                self.offset_x = cmp::min(self.offset_x + multiplier, self.max_offset_x)
            }
            Direction::Up => {
                self.offset_y = cmp::min(self.offset_y + multiplier, self.max_offset_y)
            }
            Direction::Down => {
                if self.offset_y > multiplier {
                    self.offset_y -= multiplier;
                }
            }
        }
    }

    fn tick_rate_sub(&mut self, amount: u64) {
        let rhs = Duration::from_millis(amount);
        match self.tick_rate.checked_sub(rhs) {
            Some(newval) => {
                self.tick_rate = newval;
            }
            None => {}
        }
    }

    fn tick_rate_add(&mut self, amount: u64) {
        let rhs = Duration::from_millis(amount);
        match self.tick_rate.checked_add(rhs) {
            Some(newval) => {
                self.tick_rate = newval;
            }
            None => {}
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char(' ') => self.play_pause(),
            KeyCode::Enter => self.play_pause(),
            KeyCode::Char('r') => self.ca.reset(),

            // support non-vim users... I'm judging you F.Y.I.
            KeyCode::Left => self.offset(Direction::Left, 2),
            KeyCode::Up => self.offset(Direction::Up, 1),
            KeyCode::Down => self.offset(Direction::Down, 1),
            KeyCode::Right => self.offset(Direction::Right, 2),

            KeyCode::Char('h') => self.offset(Direction::Left, 2),
            KeyCode::Char('j') => self.offset(Direction::Up, 1),
            KeyCode::Char('k') => self.offset(Direction::Down, 1),
            KeyCode::Char('l') => self.offset(Direction::Right, 2),

            KeyCode::Char('H') => self.offset(Direction::Left, 4),
            KeyCode::Char('J') => self.offset(Direction::Up, 2),
            KeyCode::Char('K') => self.offset(Direction::Down, 2),
            KeyCode::Char('L') => self.offset(Direction::Right, 4),

            KeyCode::Char('-') => self.tick_rate_add(5),
            KeyCode::Char('+') => self.tick_rate_sub(5),
            _ => {}
        }
    }
}

impl Widget for &TerminalApp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for x in 0..area.width {
            for y in 0..area.height {
                let x_off = (x + self.offset_x) as usize;
                let y_off = (y + self.offset_y) as usize;
                let idx = ((y * area.width) + x) as usize;
                let cell_idx = self.ca.grid.idx(x_off, y_off);
                let cell = self.ca.grid.data[cell_idx];
                match cell {
                    BasicCellState::Alive => {
                        buf.content[idx].set_bg(Color::White);
                    }
                    BasicCellState::Dead => {
                        buf.content[idx].set_bg(Color::Reset);
                    }
                }
            }
        }
    }
}
