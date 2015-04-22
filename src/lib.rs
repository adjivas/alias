// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/alias/LICENCE.
//
// This file may not be copied, modified, or distributed
// except according to those terms.

pub mod _macro {
  use std::collections::HashMap;
  use std::fs::File;
  use std::io::BufReader;
  use std::io::BufRead;
  use std::path::PathBuf;

  /// The `Dictionary` structure is database of synonym.
  pub struct Dictionary {
    alias: HashMap<String, String>,
  }

  impl Dictionary {

    /// The `from_files` constructor function returns a new Dictionary's Dictionary.
    pub fn from_files (
      files: &Vec<String>
    ) -> Dictionary {
      let lines = open(&files);
      let mut dict = HashMap::new();

      for mut content in lines {
        let mut line = String::new();
    
        while content.read_line(&mut line).is_ok() && !line.is_empty() {
          let (key, value) = parse(&line);

          dict.insert(key, value);
          line.clear();
        }
      }
      Dictionary {
        alias: dict,
      }
    }

    /// The `interpreter` function returns the action.
    pub fn interpreter (
      &mut self,
      line: &String,
    ) -> String {
      let mut args = line.split(" -> ");
      match args.next() {
        Some(key) => {
          match args.next() {
            Some(value) => self.set_alias(key.to_string(), value.to_string()),
            None => self.get_alias(&key),
          }
        },
        None => panic!("alias interpreter bad split"),
      }
    }

    /// The `get_word` function returns the value.
    fn get_alias (
      &self,
      value: &str,
    ) -> String {
      match self.alias.get(value) {
        Some(value) => value.clone(),
        None => value.to_string(),
      }
    }

    /// The `set_word` function returns the result of addeds value for the key.
    fn set_alias (
      &mut self,
      key: String,
      value: String,
    ) -> String {
      match self.alias.insert(key, value.clone()) {
        Some(value) => value,
        None => value,
      }
    }
  }

  /// The `parse` function returns the keys and value from a line.
  fn parse (
    line: &str,
  ) -> (String, String) {
    let lines:String = line.chars().take_while(|x| *x != '\n').collect();
    let mut split = lines.split(" -> ");
    let key:String = match split.next() {
      Some(last) => last.to_string(),
      None => panic!("parse error key"),
    };
    let value:String = match split.next() {
        Some(new) => new.to_string(),
        None => panic!("parse error value")
    };
    return (key, value);
  }

  /// The `open` function returns a file's opened list for a file's list.
  fn open (
    files: &Vec<String>
  ) -> Vec<BufReader<File>> {
    let mut lines: Vec<BufReader<File>> = Vec::with_capacity(files.capacity());
    for file in files {
      let path = PathBuf::from(file);

      match File::open(&path) {
        Err(why) => panic!("Could not open {}: {:?}", path.display(), why),
        Ok(file) => {
          lines.push(BufReader::new(file))
        },
      }
    }
    lines
  }
}
