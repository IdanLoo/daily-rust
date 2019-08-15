use std::str::FromStr;
use url::Url;

#[derive(Debug)]
pub struct Question {
  pub id: i32,
  pub name: String,
  pub url: String,
  pub snake: String,
  pub title: String
}

impl Question {
  pub fn new(id: i32, url: &str) -> Question {
    let name = name_of(url);
    let snake = snake_name_of(&name);
    let title = upper_first_letter(&name);

    Question {
      id,
      name,
      snake,
      title,
      url: String::from(url),
    }
  }
}

fn name_of(url_string: &str) -> String {
  let url = Url::from_str(url_string).unwrap();
  let name = url.path_segments().unwrap().nth(1).unwrap();
  name.to_string()
}

fn make_ascii_titlecase(s: &mut str) {
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
}

fn split_names_of(camel: &str) -> Vec<&str> {
  camel.split("-").collect()
}

fn snake_name_of(camel: &str) -> String {
  split_names_of(camel).join("_")
}

fn upper_first_letter(camel: &str) -> String {
  let mut s: String = split_names_of(camel).join(" ");
  make_ascii_titlecase(&mut s);
  s
}