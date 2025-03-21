use std::{fmt::Display, io::{self, BufRead, BufReader}, thread};

#[derive(Clone)]
pub enum LogColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    HiBlack,
    HiRed,
    HiGreen,
    HiYellow,
    HiBlue,
    HiMagenta,
    HiCyan,
    HiWhite,
}

impl Display for LogColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogColor::Black => write!(f, "30m"),
            LogColor::Red => write!(f,"31m"),
            LogColor::Green => write!(f,"32m"),
            LogColor::Yellow => write!(f,"33m"),
            LogColor::Blue => write!(f,"34m"),
            LogColor::Magenta => write!(f,"35m"),
            LogColor::Cyan => write!(f,"36m"),
            LogColor::White => write!(f,"37m"),
            LogColor::HiBlack => write!(f,"90m"),
            LogColor::HiRed => write!(f,"91m"),
            LogColor::HiGreen => write!(f,"92m"),
            LogColor::HiYellow => write!(f,"93m"),
            LogColor::HiBlue=> write!(f,"94m"),
            LogColor::HiMagenta=> write!(f,"95m"),
            LogColor::HiCyan=> write!(f,"96m"),
            LogColor::HiWhite=> write!(f,"97m"),
        }
    }
}

pub struct Logger {
    name: String,
    color: LogColor,
}

impl Logger {
    pub fn new(name: String, color: LogColor) -> Self {
        Self { name, color }
    }

    pub fn stream(&self, out: impl io::Read + Send + 'static, color: Option<LogColor>) -> thread::JoinHandle<()> {
        let name = self.name.clone();
        let color = color.unwrap_or(self.color.clone());

        thread::spawn(move || {
            let reader = BufReader::new(out);
            for line in reader.lines() {
                let line = line.unwrap_or("error reading line".to_string());
                let print = format!("\x1b[{color}[{name}] {line}\x1b[0m");
                println!("{print}")
            }
        })
    }
}

