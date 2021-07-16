extern crate colored;

mod words;

use std::{fs, usize};
use words::{find_bonus_words, find_words};

use colored::*;
use rustyline::{error::ReadlineError, Editor};

fn print_words(words: Vec<(String, usize)>) {
  println!(
    "{}",
    words
      .iter()
      .map(|d| format!("{} {}", d.1, d.0))
      .collect::<Vec<String>>()
      .join("\n")
  );
}

const HISTFILE: &str = ".wwhistory";

fn main() {
  let file = fs::read_to_string("words.txt").expect("Error reading words.txt");
  let word_list = file.split("\n").collect::<Vec<&str>>();

  let mut rl = Editor::<()>::new();

  rl.load_history(HISTFILE).ok();

  loop {
    let cloned_words = word_list.to_vec();
    let readline = rl.readline(&format!(
      "{} {} ",
      "wordswith".bright_blue().bold(),
      ">".bold()
    ));
    match readline {
      Ok(line) => {
        rl.add_history_entry(line.as_str());
        let segments = line.split_ascii_whitespace().collect::<Vec<&str>>();
        if segments.len() == 0 {
          continue;
        }
        let words = if segments.len() == 1 {
          find_words(cloned_words, segments[0])
        } else {
          find_bonus_words(cloned_words, segments[1], segments[0])
        };

        print_words(words);
      }
      Err(ReadlineError::Interrupted) => {
        println!("CTRL-C");
        break;
      }
      Err(ReadlineError::Eof) => {
        println!("CTRL-D");
        break;
      }
      Err(err) => {
        println!("Error: {:?}", err);
        break;
      }
    }
  }
  rl.save_history(HISTFILE).unwrap();
}
