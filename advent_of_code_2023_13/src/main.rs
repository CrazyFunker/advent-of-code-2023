use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Pattern {
    h: Vec<String>,
    v: Vec<String>,
    h_sym_index: Option<usize>,
    v_sym_index: Option<usize>,
}

impl Pattern {
    fn add_horizontal(&mut self, line: String) {
        self.h.push(line.clone());

        if self.h.len() == 1 {
            // first line, need to initialise the vertical Strings
            for i in 0..line.len() {
                self.v.push(String::new());
            }
        } else { // start checking for h symmetry
            match self.h_sym_index {
                Some(i) => {}
                None => {
                    // check if current line is equal to previous line
                    if self.h[self.h.len() - 1] == self.h[self.h.len() - 2] {
                        self.h_sym_index = Some(self.h.len() - 1);
                    }
                }
            }
        }

        for (i, c) in line.chars().enumerate() {
            self.v[i].push(c);
        }
        if self.v.len() > 1 {
            // check v symmetry
            match self.v_sym_index {
                Some(i) => {}
                None => {
                    // check if current line is equal to previous line
                    if self.v[self.v.len() - 1] == self.v[self.v.len() - 2] {
                        self.v_sym_index = Some(self.v.len() - 1);
                    }
                }
            }
        }
    }
}

fn main() {
    let mut patterns: Vec<Pattern> = Vec::new();
    let mut cur_pattern: Pattern = Pattern {
        h: Vec::new(),
        v: Vec::new(),
        h_sym_index: None,
        v_sym_index: None,
    };

    // read the input file
    let path: &Path = Path::new("input.txt");
    let file: File = File::open(&path).unwrap();
    let reader: io::BufReader<File> = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line == "" {
                    println!("{:#?}", cur_pattern);
                    patterns.push(cur_pattern);

                    cur_pattern = Pattern {
                        h: Vec::new(),
                        v: Vec::new(),
                        h_sym_index: None,
                        v_sym_index: None,
                    };
                    break;
                    // continue;
                }

                cur_pattern.add_horizontal(line);
            }
            Err(error) => {
                println!("Error reading line: {}", error);
            }
        }
    }

    // println!("{:#?}", cur_pattern);
    // patterns.push(cur_pattern);

    // cur_pattern = Pattern {
    //     h: Vec::new(),
    //     v: Vec::new(),
    //     h_sym_index: None,
    //     v_sym_index: None,
    // };

    // find vertical reflection
    // find horizontal reflection
}
