// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/alias/LICENCE.
//
// This file may not be copied, modified, or distributed
// except according to those terms.

pub mod alias {
  use std::collections::HashMap;
  use std::fs::File;
  use std::io::BufReader;
  use std::io::BufRead;
  use std::path::PathBuf;

  /// The `dictionary` structure is database of synonym.
  pub struct Dictionary {
    hash: HashMap<String, String>,
  }

  impl Dictionary {
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
        hash: dict,
      }
    }

    /// The `get_word` function returns the synonym's word.
    pub fn get_word (
      &self,
      keys: &str
    ) -> Result<String, String> {
      let word:String = keys.chars().flat_map(|c| c.to_lowercase()).collect();
      match self.hash.get(&word) {
        Some(value) => Ok(value.clone()),
        None => Err(word),
      }
    }
  }

  /// The `parse` function returns the keys and value from a line.
  fn parse (
    line: &str
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

