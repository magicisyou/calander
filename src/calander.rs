use chrono::{Datelike, Utc};

pub struct Calander {
    pub day: u32,
    pub month: u32,
    pub year: u32,
    pub today: (u32, u32, u32),
}

impl Calander {
    pub fn from(day: u32, month: u32, year: u32) -> Self {
        let now = Utc::now();
        Self {
            day,
            month,
            year,
            today: (now.day(), now.month(), now.year() as u32),
        }
    }

    pub fn today() -> Self {
        let now = Utc::now();
        Self {
            day: now.day(),
            month: now.month(),
            year: now.year() as u32,
            today: (now.day(), now.month(), now.year() as u32),
        }
    }

    pub fn go_to_today(&mut self) {
        (self.day, self.month, self.year) = self.today;
    }

    pub fn max_days_in_month(&self, month: u32) -> u32 {
        match month {
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => 31,
        }
    }

    fn leap_year(&self) -> bool {
        self.year % 400 == 0 || self.year % 100 != 0 && self.year % 4 == 0
    }

    pub fn odd_days(&self) -> u32 {
        let mut odd_days = if self.year % 400 == 0 {
            5
        } else {
            (self.year % 400 - 1) + (self.year % 400 - 1) / 4 - (self.year % 400 - 1) / 100
        };
        for month in 1..self.month {
            odd_days += self.max_days_in_month(month);
        }
        odd_days += 1;
        odd_days % 7
    }

    pub fn next_month(&mut self) {
        if self.month == 12 {
            self.month = 1;
            self.year += 1;
        } else {
            self.month += 1;
        }
        if self.month == self.today.1 && self.year == self.today.2 {
            self.day = self.today.0;
        } else {
            self.day = 0;
        }
    }

    pub fn previous_month(&mut self) {
        if self.month == 1 {
            if self.year > 0 {
                self.month = 12;
                self.year -= 1;
            }
        } else {
            self.month -= 1;
        }
        if self.month == self.today.1 && self.year == self.today.2 {
            self.day = self.today.0;
        } else {
            self.day = 0;
        }
    }

    pub fn next_year(&mut self) {
        self.year = self.year.saturating_add(1);
        if self.month == self.today.1 && self.year == self.today.2 {
            self.day = self.today.0;
        } else {
            self.day = 0;
        }
    }

    pub fn previous_year(&mut self) {
        if self.year != 0 {
            self.year -= 1;
        }
        if self.month == self.today.1 && self.year == self.today.2 {
            self.day = self.today.0;
        } else {
            self.day = 0;
        }
    }
}
