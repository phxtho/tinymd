use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader,Write};

fn parse_markdown_file(filename: &str) -> Vec<String> {
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

     tokens
}

fn save_html_file(tokens: &Vec<String>, filename: &str) {
    let mut output_filename = String::from(&filename[..filename.len()-3]); // remove ".md" from filename
    output_filename.push_str(".html");
    let mut outfile = File::create(output_filename).expect("[ERROR] Could not create ouput file");
    for line in tokens {
        outfile.write_all(line.as_bytes()).expect("[ERROR] Could not write output to file");
    }
}

fn main() {
    let args:Vec<String> = std::env::args().collect();
    match args.len() {
        1 => (), 
        2 => {
            let filename = &args[1];
            let tokens = parse_markdown_file(filename);
            save_html_file(&tokens, filename);
        },
        _ => ()
    };
}
