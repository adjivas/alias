// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/alias/LICENCE.
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate macrolib;

fn main() {
  let files = vec!["./data/example.macro".to_string()];
  let dictionary = macrolib::_macro::Dictionary::from_files(&files);
  let mut line:String = String::new();
  loop {
    match std::io::stdin().read_line(&mut line) {
      Ok(_) => {
        let line:String = line.chars().take_while(|x|
          *x != '\n'
        ).collect();
        let sentence = dictionary.get_alias(&line);

        println!("{:?} ", sentence);
      },
      Err(_) => break ,
    }
    line.clear();
  }
}
