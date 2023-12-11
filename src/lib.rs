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

pub struct Date {
    pub month: u32,
    pub year: u32,
}

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

const STATUS_LINE_CONTROLS: &str = "Quit - q | Today - t | Change month/year - Arrow keys |";

pub fn run(date: Option<Date>) -> crossterm::Result<()> {
    let mut ui = Ui::init(date)?;
    ui.run()?;
    Ok(())
}

struct Ui {
    calander: Calander,
}

impl Ui {
    fn init(date: Option<Date>) -> crossterm::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;
        let calander = match date {
            Some(c) => Calander::from(0, c.month, c.year),
            None => Calander::today(),
        };
        Ok(Self { calander })
    }

    fn get_today_as_text(&self) -> String {
        format!(
            "{} {} {}",
            self.calander.today.0,
            &MONTHS[self.calander.today.1 as usize - 1],
            self.calander.today.2,
        )
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
                queue!(
                    stdout,
                    Print(format!("[{}]", date.to_string().bold()))
                )?;
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
                    queue!(
                        stdout,
                        Print(format!("[{}]", date.to_string().bold()))
                    )?;
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
        queue!(
            stdout,
            cursor::MoveTo(
                (term_size.0
                    - (STATUS_LINE_CONTROLS.len() + self.get_today_as_text().len()) as u16)
                    / 2,
                term_size.1 - 1
            ),
            Print(format!(
                "{} {}",
                STATUS_LINE_CONTROLS.dark_grey(),
                self.get_today_as_text().red(),
            ))
        )?;
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
