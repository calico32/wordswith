use rand::{seq::IteratorRandom, thread_rng};
use regex::Regex;
use std::{collections::HashSet, usize};

fn unique_chars(string: &str) -> usize {
  let mut set: HashSet<char> = HashSet::new();
  for char in string.chars() {
    set.insert(char);
  }
  return set.len();
}

const WORD_COUNT: usize = 20;

fn find_all_matches<'a>(word_list: Vec<&'a str>, prompt: &str) -> Vec<&'a str> {
  let re = Regex::new(&prompt.to_lowercase()).unwrap();

  let matches = word_list
    .iter()
    .filter(|word| re.is_match(word))
    .map(|&word| word)
    .collect::<Vec<&str>>();

  return matches;
}

pub(crate) fn find_words(
  word_list: Vec<&str>,
  prompt: &str,
) -> Vec<(String, usize)> {
  let matches = find_all_matches(word_list, prompt);

  let mut rng = thread_rng();

  let mut sample = matches
    .iter()
    .choose_multiple(&mut rng, WORD_COUNT)
    .iter()
    .map(|&s| (s.to_string(), unique_chars(s)))
    .collect::<Vec<(String, usize)>>();

  sample.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
  sample.sort_unstable_by(|a, b| a.1.cmp(&b.1));

  return sample;
}

pub(crate) fn find_bonus_words(
  word_list: Vec<&str>,
  prompt: &str,
  bonus_letters: &str,
) -> Vec<(String, usize)> {
  let matches = find_all_matches(word_list, prompt);

  let mut rng = thread_rng();

  let mut best_matches: Vec<(String, usize)> = vec![];
  let mut other_matches: Vec<&str> = vec![];

  let bonus_re = bonus_letters
    .chars()
    .map(|c| Regex::new(&c.to_string()).unwrap())
    .collect::<Vec<Regex>>();

  for word in matches {
    let re = bonus_re.to_vec();
    let score = re.iter().map(|re| re.is_match(word)).filter(|&b| b).count();
    if score > 0 {
      best_matches.push((word.to_string(), score));
    } else {
      other_matches.push(word);
    }
  }

  let mut selection: Vec<(String, usize)> = vec![];

  best_matches.sort_by(|a, b| a.0.len().cmp(&b.0.len()));
  best_matches.sort_by(|a, b| b.1.cmp(&a.1));

  if best_matches.len() < WORD_COUNT {
    selection.append(&mut best_matches);
    selection.append(
      &mut other_matches
        .iter()
        .choose_multiple(&mut rng, WORD_COUNT - best_matches.len())
        .iter()
        .map(|&&word| (word.to_string(), 0usize))
        .collect(),
    )
  } else {
    selection.append(&mut best_matches[0..WORD_COUNT].to_vec());
  }

  selection.reverse();

  return selection;
}
