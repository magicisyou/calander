use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{Print, Stylize},
    terminal,
};
use std::io::{self, Write};

mod calander;

use crate::calander::Calander;

const WEEKS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "Jully",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub fn run(config: Option<Config>) -> crossterm::Result<()> {
    let mut ui = Ui::init(config)?;
    ui.run()?;
    Ok(())
}

struct Ui {
    calander: Calander,
}

impl Ui {
    fn init(config: Option<Config>) -> crossterm::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;
        let calander = match config {
            Some(c) => Calander::from(0, c.month, c.year),
            None => Calander::today(),
        };
        Ok(Self { calander })
    }

    fn update_screen(&self) -> crossterm::Result<()> {
        let term_size = terminal::size()?;
        let mut stdout = io::stdout();
        let mut line: u16 = 0;
        let position_x = |x: &u16| 12 + (term_size.0 - 12) / 14 + (term_size.0 - 12) / 7 * x;
        let position_y = |x: &u16| term_size.1 / 14 + term_size.1 / 7 * x;
        queue!(stdout, terminal::Clear(terminal::ClearType::All),)?;
        for (index, week) in WEEKS.iter().enumerate() {
            queue!(
                stdout,
                cursor::MoveTo(position_x(&(index as u16)), position_y(&line)),
            )?;
            if index == 0 {
                queue!(stdout, Print(week.bold().red()))?;
            } else {
                queue!(stdout, Print(week.bold().yellow()))?;
            }
        }
        line += 1;
        let mut date = 1;
        for week in self.calander.odd_days()..7 {
            queue!(
                stdout,
                cursor::MoveTo(position_x(&(week as u16)), position_y(&line)),
            )?;
            if date == self.calander.day {
                queue!(stdout, Print(date.to_string().blue().bold()))?;
            } else if week == 0 {
                queue!(stdout, Print(date.to_string().red()))?;
            } else {
                queue!(stdout, Print(date))?;
            }
            date += 1;
        }
        line += 1;
        while date <= self.calander.max_days_in_month(self.calander.month) {
            let mut week = 0;
            while date <= self.calander.max_days_in_month(self.calander.month) && week < 7 {
                queue!(
                    stdout,
                    cursor::MoveTo(position_x(&(week as u16)), position_y(&line)),
                )?;
                if date == self.calander.day {
                    queue!(stdout, Print(date.to_string().blue().bold()))?;
                } else if week == 0 {
                    queue!(stdout, Print(date.to_string().red()))?;
                } else {
                    queue!(stdout, Print(date))?;
                }
                date += 1;
                week += 1;
            }
            line += 1;
        }
        queue!(
            stdout,
            cursor::MoveTo(1, (term_size.1 - 14) / 2),
            Print(self.calander.year.to_string().bold().yellow())
        )?;
        for (index, month) in MONTHS.iter().enumerate() {
            queue!(
                stdout,
                cursor::MoveTo(1, 2 + (term_size.1 - 14) / 2 + index as u16)
            )?;
            if index == (self.calander.month - 1) as usize {
                queue!(stdout, Print(month.green().bold()))?;
            } else {
                queue!(stdout, Print(month.dark_grey()))?;
            }
        }
        stdout.flush()?;
        Ok(())
    }

    fn run(&mut self) -> crossterm::Result<()> {
        self.update_screen()?;
        loop {
            if let Event::Key(event) = event::read()? {
                match event {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => break,
                    KeyEvent {
                        code: KeyCode::Char('t'),
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => self.calander.go_to_today(),
                    KeyEvent {
                        code: KeyCode::Up,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => self.calander.previous_month(),
                    KeyEvent {
                        code: KeyCode::Down,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => self.calander.next_month(),
                    KeyEvent {
                        code: KeyCode::Right,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => self.calander.next_year(),
                    KeyEvent {
                        code: KeyCode::Left,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => self.calander.previous_year(),
                    _ => continue,
                }
            }
            self.update_screen()?;
        }
        Ok(())
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Failed to disable raw mode");
        execute!(io::stdout(), cursor::Show, terminal::LeaveAlternateScreen,)
            .expect("Failed to execute cleanup commands");
    }
}

pub struct Config {
    month: u32,
    year: u32,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Option<Self>, &'static str> {
        args.next();
        let month = match args.next() {
            Some(m) => m,
            None => return Ok(None),
        };
        let month = match month.parse::<u32>() {
            Ok(m) => m,
            Err(_) => return Err("Month is expected as integer"),
        };
        if !(1..=12).contains(&month) {
            return Err("Month should be in range 1 to 12");
        }
        let year = match args.next() {
            Some(y) => y,
            None => return Err("Year not entered"),
        };
        let year = match year.parse::<u32>() {
            Ok(y) => y,
            Err(_) => return Err("Year is expected as positive integer"),
        };
        Ok(Some(Self { month, year }))
    }
}
