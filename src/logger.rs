use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::path::Path;
use chrono::prelude::*;
use chrono::{DateTime, Utc, Local};

const path: &str = "crash.log";

pub struct Logger {}

impl Logger {
    pub fn log(message: String) {
        let now = Local::now();
        let mut file = File::create(path).unwrap();
        writeln!(file, "{} - {}", now.naive_utc().format("%Y-%m-%d %H:%M:%S"), message).unwrap();
    }
}
