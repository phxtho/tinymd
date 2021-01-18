use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_markdown_file(filename: &str) {
    println!("[INFO] Trying to parse {}...", filename);

    let input_filename = Path::new(filename);
    let file = match File::open(&input_filename) {
        Err(err) => panic!("Couldn't open file: {}", err),
        Ok(value) => value,
    };

    let mut _ptag = false;
    let mut _htag = false;
    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(file);
     for line in reader.lines() {
        let mut output_line = String::new();

         let line_content = line.unwrap();
         let first_char = line_content.chars().nth(0).unwrap();
         match first_char {
             '#' => {
                // Close open paragraph 
                if _ptag {
                     _ptag = false;
                     output_line.push_str("</p>\n");
                 }
                // Close open heading
                 if _htag {
                     _htag = false;
                     output_line.push_str("</h1>\n");
                 }
                
                 _htag = true;
                 output_line.push_str("<h1>");
                 output_line.push_str(&line_content[2..]);
             },
             _ => {
                 if !_ptag {
                     _ptag = true;
                     output_line.push_str("<p>")
                    }

                output_line.push_str(&line_content);
                 
             }
         };

         if _ptag {
            _ptag = false;
            output_line.push_str("</p>\n");
          }

          if _htag {
            _htag = false;
            output_line.push_str("</h1>\n");      
          } 
          
          if output_line != "<p></p>\n" {
            tokens.push(output_line);
          }

     }
}

fn main() {
    let args:Vec<String> = std::env::args().collect();
    match args.len() {
        1 => (), 
        2 => parse_markdown_file(&args[1]),
        _ => ()
    };
}
