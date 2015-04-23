// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/macro/LICENCE.
//
// This file may not be copied, modified, or distributed
// except according to those terms.

pub mod _macro {
 pub mod _macro {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::BufRead;
    use std::path::PathBuf;

    /// The `Alias` structure is database of synonym.
    pub struct Alias {
      alias: HashMap<String, String>,
    }

    impl Alias {

      /// The `from_files` constructor function returns a new Alias's Alias.
      pub fn from_files (
        files: &Vec<String>
      ) -> Alias {
        let lines = open(&files);
        let mut dict = HashMap::new();

        for mut content in lines {
          let mut line = String::new();
      
          while content.read_line(&mut line).is_ok()
          && !line.is_empty() {
            let (key, value) = parse(&line);

            dict.insert(key, value);
            line.clear();
          }
        }
        Alias {
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

      /// The `replace` function replaces all key's alias word
      /// by value's alias word.
      pub fn replace (
        &self,
        line: &String,
      ) -> String {
        let mut replaced:String = line.clone();

        for (key, val) in self.alias.iter() {
          replaced = replaced.replace(key, val);
        }
        replaced
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
      let mut split = line.split(" -> ");
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

  fn command (
    line: &String,
    parser: &interpreterlib::interpreter::Parser,
    _macro: &_macro::Alias,
  ) -> String { 
    match parser.exec(line) {
      Ok(display) => display,
      Err(why) => why,
    }
  }

  fn learn (
    line: &String,
    alias: &mut _macro::Alias,
  ) -> String {
    let learn:String = line.chars().take_while(|x|
      !x.is_whitespace()
    ).skip(1).collect();
    let arg:&String = &line.chars().skip_while(|x|
      !x.is_whitespace()
    ).skip(1).collect();
    match learn.as_ref() {
      "alias" => alias.interpreter(arg),
      "env" => env::interpreter(arg),
      _ => "bad expression".to_string(),
    }
  }

  pub fn interpreter (
    line: &String,
    alias: &mut _macro::Alias,
    parser: &interpreterlib::interpreter::Parser,
  ) -> String {
    match line.as_bytes()[0] as char {
      c if c.is_alphanumeric() => {
        let mut replaced:String = line.clone();

        replaced = env::replace(&replaced);
        replaced = alias.replace(&replaced);
        command(&replaced, &parser, &alias)
      },
      c if c == '!' => learn(line, alias),
      _ => "".to_string(),
    }
  }
}
