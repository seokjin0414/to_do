use std::fs::OpenOptions;
use chrono::{serde::ts_seconds, DateTime, Utc, Local};
use serde::{Deserialize, Serialize};
use std::io::{Result, Seek, SeekFrom};
use std::iter::Successors;
use std::path::PathBuf;
use std::ptr::read;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // open file
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks: Vec<Task> = match serde_json::from_reader(&file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    file.seek(SeekFrom::Start(0))?;
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

// fn function_1() -> Result(Success, Failure) {
//     match operation_that_might_fail() {
//         Ok(success) => success,
//         Err(failure) => return Err(failure),
//     }
// }
//  동일하다
// fn function_2() -> Result(Success, Failure) {
//     operation_that_might_fail()?
// }