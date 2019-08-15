extern crate time;
extern crate url;

mod helper;
mod question;
mod solutions;

use question::Question;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let id: i32 = args[1].parse().unwrap();
  let url = &args[2];

  let question = Question::new(id, url);
  helper::insert_mod(&question.snake).unwrap();
  helper::write_mod_file(&question).unwrap();
}
