use std::fs::{File, OpenOptions};
use chrono::{serde::ts_seconds, DateTime, Utc, Local};
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
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

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;

    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    Ok(tasks)
}


pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // open file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks: Vec<Task> = collect_tasks(&file)?;
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
// 동일하다
// fn function_2() -> Result(Success, Failure) {
//     operation_that_might_fail()?
// }

pub fn complete_task(journal_path: PathBuf, task_position: usize)-> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks =collect_tasks(&file)?;

    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid task ID"));
    }

    tasks.remove(task_position - 1);
    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}















