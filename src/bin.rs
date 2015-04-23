// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/macro/LICENCE.
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate macrolib;

fn main() {
  let files = vec!["./data/example.macro".to_string()];
  let mut _macro = macrolib::_macro::Dictionary::from_files(&files);
  let mut line:String = String::new();

  loop {
    match std::io::stdin().read_line(&mut line) {
      Ok(_) => {
        let arg:String = line.chars().take_while(|x|
          *x != '\n'
        ).collect();
        let result = _macro.interpreter(&arg);

        println!("{:?}", result);
      },
      Err(_) => break ,
    }
    line.clear();
  }
}
