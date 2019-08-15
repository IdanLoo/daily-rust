use crate::question::Question;
use std::env::current_dir;
use std::fs::{File, OpenOptions};
use std::io::{Result, Write};
use std::path::PathBuf;

pub fn insert_mod(name: &str) -> Result<()> {
  let cwd = current_dir().unwrap();
  let path = {
    let mut tmp = PathBuf::from(cwd);
    tmp.push("src/solutions/mod.rs");
    tmp
  };
  let mut file = OpenOptions::new().append(true).open(path)?;
  write!(file, "pub mod {};\n", name)
}

pub fn write_mod_file(question: &Question) -> Result<()> {
  let cwd = current_dir().unwrap();
  let path = {
    let mut tmp = PathBuf::from(cwd);
    tmp.push("src/solutions");
    tmp.push(format!("{}.rs", question.snake));
    tmp
  };

  let mut file = File::create(path)?;
  write!(
    file,
    "/**
 * {id}. {title}
 * URL: {url}
 * Author: Idan Loo <im@siwei.lu>
 * Date: {date}
 */
pub struct Solution {{}}
    ",
    id = question.id,
    title = question.title,
    url = question.url,
    date = now_date()
  )
}

fn now_date() -> String {
  let now = time::now();
  time::strftime("%Y-%m-%d", &now).unwrap()
}
