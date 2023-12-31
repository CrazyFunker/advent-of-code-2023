use core::panic;
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
            for _i in 0..line.len() {
                self.v.push(String::new());
            }
        }

        for (i, c) in line.chars().enumerate() {
            self.v[i].push(c);
        }
    }

    fn find_symmetries(&mut self) {
        self.find_horizontal_symmetry();
        self.find_vertical_symmetry();
        // println!("Pattern: {:#?}", self.h);
        // println!("Horizontal symmetry: {:#?}", self.h_sym_index);
        // println!("Vertical symmetry: {:#?}", self.v_sym_index);
    }

    fn find_symmetries_v2(&mut self) {
        self.find_horizontal_symmetry_v2();
        self.find_vertical_symmetry_v2();

        if self.h_sym_index == None && self.v_sym_index == None {
            panic!("No symmetries found: {:#?}", self);
        }

        if self.h_sym_index != None && self.v_sym_index != None {
            panic!("Both h and v symmetries found: {:#?}", self);
        }
        // println!("Pattern: {:#?}", self.h);
        // println!("Horizontal symmetry: {:#?}", self.h_sym_index);
        // println!("Vertical symmetry: {:#?}", self.v_sym_index);
    }

    fn find_horizontal_symmetry_v2(&mut self) {
        let mut index: usize = 1; // start at second line
        let mut sym_index: Option<usize> = None;
        let mut next_sym_index: Option<usize> = None;
        let mut queue: Vec<String> = Vec::new();
        let mut discarded: Vec<String> = Vec::new();
        let mut used_smudge: bool = false;
        let mut next_sym_index_used_smudge: bool = false;
        self.h_sym_index = None;
        queue.push(self.h[0].clone());

        while index < self.h.len() {
            let diff = char_diff(&self.h[index], &self.h[index - 1]);
            // if sym_index is None, then we are looking for the first line that is a mirror of the previous line
            match sym_index {
                None => {
                    if diff == 0 || (used_smudge == false && diff == 1) {
                        if diff == 1 {
                            used_smudge = true;
                        }
                        sym_index = Some(index);
                        self.h_sym_index = Some(index);
                        discarded.push(queue.pop().unwrap());
                    } else {
                        queue.push(self.h[index].clone());
                    }
                }
                Some(_i) => {
                    if next_sym_index == None
                        && (diff == 0 || (next_sym_index_used_smudge == false && diff == 1))
                    {
                        if diff == 1 {
                            next_sym_index_used_smudge = true;
                        }
                        next_sym_index = Some(index);
                    }

                    let el = queue.pop();
                    match el {
                        Some(line) => {
                            // is the same as current line?
                            let diff2 = char_diff(&self.h[index], &line);
                            if line.eq(&self.h[index]) || (used_smudge == false && diff2 == 1) {
                                if diff2 == 1 {
                                    used_smudge = true;
                                }
                                discarded.push(line);
                            } else {
                                // reset to next sym_index

                                sym_index = next_sym_index;
                                self.h_sym_index = next_sym_index;

                                if next_sym_index_used_smudge == true {
                                    used_smudge = true;
                                    next_sym_index_used_smudge = false;
                                } else {
                                    used_smudge = false;
                                }

                                // flush discarded q
                                discarded.clear();

                                // take slice of queue from start to next_sym_index
                                if next_sym_index != None {
                                    queue =
                                        Vec::from(self.h[0..next_sym_index.unwrap() - 1].to_vec());
                                    // reset the index to next_sym_index
                                    index = next_sym_index.unwrap();
                                    next_sym_index = None;
                                } else {
                                    queue = Vec::from(self.h[0..index + 1].to_vec());
                                }
                            }
                        }
                        None => {
                            if used_smudge == true {
                                // we've found the mirror early
                                break;
                            } else {
                                // reset to next sym_index

                                sym_index = next_sym_index;
                                self.h_sym_index = next_sym_index;

                                if next_sym_index_used_smudge == true {
                                    used_smudge = true;
                                    next_sym_index_used_smudge = false;
                                } else {
                                    used_smudge = false;
                                }

                                // flush discarded q
                                discarded.clear();

                                // take slice of queue from start to next_sym_index
                                if next_sym_index != None {
                                    queue =
                                        Vec::from(self.h[0..next_sym_index.unwrap() - 1].to_vec());
                                    // reset the index to next_sym_index
                                    index = next_sym_index.unwrap();
                                    next_sym_index = None;
                                } else {
                                    queue = Vec::from(self.h[0..index + 1].to_vec());
                                }
                            }
                        }
                    }
                }
            }

            index += 1;
            // special case for the last line
            if index == self.h.len() && used_smudge == false && next_sym_index_used_smudge == true {
                // reset to next sym_index

                sym_index = next_sym_index;
                self.h_sym_index = next_sym_index;

                if next_sym_index_used_smudge == true {
                    used_smudge = true;
                    next_sym_index_used_smudge = false;
                } else {
                    used_smudge = false;
                }

                // flush discarded q
                discarded.clear();

                // take slice of queue from start to next_sym_index
                if next_sym_index != None {
                    queue = Vec::from(self.h[0..next_sym_index.unwrap() - 1].to_vec());
                    // reset the index to next_sym_index
                    index = next_sym_index.unwrap();
                    next_sym_index = None;
                } else {
                    queue = Vec::from(self.h[0..index + 1].to_vec());
                }
            }
        }

        // if used_smudge == false {
        //     println!("Pattern: {:#?}", self.h);
        //     panic!("No h smudge used!");
        // }
        if used_smudge == false {
            // println!("Didn't use smudge, setting h_sym_index to None");
            self.h_sym_index = None;
        }
    }

    fn find_vertical_symmetry_v2(&mut self) {
        let mut index: usize = 1; // start at second line
        let mut sym_index: Option<usize> = None;
        let mut next_sym_index: Option<usize> = None;
        let mut queue: Vec<String> = Vec::new();
        let mut discarded: Vec<String> = Vec::new();
        let mut used_smudge: bool = false;
        let mut next_sym_index_used_smudge: bool = false;
        self.v_sym_index = None;
        queue.push(self.v[0].clone());

        while index < self.v.len() {
            let diff = char_diff(&self.v[index], &self.v[index - 1]);
            // if sym_index is None, then we are looking for the first line that is a mirror of the previous line
            match sym_index {
                None => {
                    if diff == 0 || (used_smudge == false && diff == 1) {
                        if diff == 1 {
                            used_smudge = true;
                        }
                        sym_index = Some(index);
                        self.v_sym_index = Some(index);
                        discarded.push(queue.pop().unwrap());
                    } else {
                        queue.push(self.v[index].clone());
                    }
                }
                Some(_i) => {
                    if next_sym_index == None
                        && (diff == 0 || (next_sym_index_used_smudge == false && diff == 1))
                    {
                        if diff == 1 {
                            next_sym_index_used_smudge = true;
                        }
                        next_sym_index = Some(index);
                    }

                    let el = queue.pop();
                    match el {
                        Some(line) => {
                            // is the same as current line?
                            let diff2 = char_diff(&self.v[index], &line);
                            if line.eq(&self.v[index]) || (used_smudge == false && diff2 == 1) {
                                if diff2 == 1 {
                                    used_smudge = true;
                                }
                                discarded.push(line);
                            } else {
                                // reset to next sym_index

                                sym_index = next_sym_index;
                                self.v_sym_index = next_sym_index;

                                if next_sym_index_used_smudge == true {
                                    used_smudge = true;
                                    next_sym_index_used_smudge = false;
                                } else {
                                    used_smudge = false;
                                }

                                // flush discarded q
                                discarded.clear();

                                // take slice of queue from start to next_sym_index
                                if next_sym_index != None {
                                    queue =
                                        Vec::from(self.v[0..next_sym_index.unwrap() - 1].to_vec());
                                    // reset the index to next_sym_index
                                    index = next_sym_index.unwrap();
                                    next_sym_index = None;
                                } else {
                                    queue = Vec::from(self.v[0..index + 1].to_vec());
                                }
                            }
                        }
                        None => {
                            if used_smudge == true {
                                // we've found the mirror early
                                break;
                            } else {
                                // reset to next sym_index

                                sym_index = next_sym_index;
                                self.v_sym_index = next_sym_index;

                                if next_sym_index_used_smudge == true {
                                    used_smudge = true;
                                    next_sym_index_used_smudge = false;
                                } else {
                                    used_smudge = false;
                                }

                                // flush discarded q
                                discarded.clear();

                                // take slice of queue from start to next_sym_index
                                if next_sym_index != None {
                                    queue =
                                        Vec::from(self.v[0..next_sym_index.unwrap() - 1].to_vec());
                                    // reset the index to next_sym_index
                                    index = next_sym_index.unwrap();
                                    next_sym_index = None;
                                } else {
                                    queue = Vec::from(self.v[0..index + 1].to_vec());
                                }
                            }
                        }
                    }
                }
            }

            index += 1;
            // special case for the last line
            if index == self.v.len() && used_smudge == false && next_sym_index_used_smudge == true {
                // reset to next sym_index

                sym_index = next_sym_index;
                self.v_sym_index = next_sym_index;

                if next_sym_index_used_smudge == true {
                    used_smudge = true;
                    next_sym_index_used_smudge = false;
                } else {
                    used_smudge = false;
                }

                // flush discarded q
                discarded.clear();

                // take slice of queue from start to next_sym_index
                if next_sym_index != None {
                    queue = Vec::from(self.v[0..next_sym_index.unwrap() - 1].to_vec());
                    // reset the index to next_sym_index
                    index = next_sym_index.unwrap();
                    next_sym_index = None;
                } else {
                    queue = Vec::from(self.v[0..index + 1].to_vec());
                }
            }
        }

        // if used_smudge == false {
        //     println!("Pattern: {:#?}", self.h);
        //     panic!("No v smudge used!");
        // }
        if used_smudge == false {
            // println!("Didn't use smudge, setting v_sym_index to None");
            self.v_sym_index = None;
        }
    }

    fn find_horizontal_symmetry(&mut self) {
        let mut index: usize = 1; // start at second line
        let mut sym_index: Option<usize> = None;
        let mut next_sym_index: Option<usize> = None;
        let mut queue: Vec<String> = Vec::new();
        let mut discarded: Vec<String> = Vec::new();
        self.h_sym_index = None;
        queue.push(self.h[0].clone());

        while index < self.h.len() {
            // if sym_index is None, then we are looking for the first line that is a mirror of the previous line
            match sym_index {
                None => {
                    if self.h[index] == self.h[index - 1] {
                        sym_index = Some(index);
                        self.h_sym_index = Some(index);
                        discarded.push(queue.pop().unwrap());
                    } else {
                        queue.push(self.h[index].clone());
                    }
                }
                Some(_i) => {
                    if self.h[index] == self.h[index - 1] && next_sym_index == None {
                        next_sym_index = Some(index);
                    }

                    let el = queue.pop();
                    match el {
                        Some(line) => {
                            // is the same as current line?
                            if line.eq(&self.h[index]) {
                                discarded.push(line);
                            } else {
                                // reset to next sym_index

                                sym_index = next_sym_index;
                                self.h_sym_index = next_sym_index;

                                // flush discarded q
                                discarded.clear();

                                // take slice of queue from start to next_sym_index
                                if next_sym_index != None {
                                    queue =
                                        Vec::from(self.h[0..next_sym_index.unwrap() - 1].to_vec());
                                    // reset the index to next_sym_index
                                    index = next_sym_index.unwrap();
                                    next_sym_index = None;
                                } else {
                                    queue = Vec::from(self.h[0..index + 1].to_vec());
                                }
                            }
                        }
                        None => {
                            // we've found the mirror early
                            break;
                        }
                    }
                }
            }

            index += 1;
        }
    }

    fn find_vertical_symmetry(&mut self) {
        let mut index: usize = 1; // start at second line
        let mut sym_index: Option<usize> = None;
        let mut next_sym_index: Option<usize> = None;
        let mut queue: Vec<String> = Vec::new();
        let mut discarded: Vec<String> = Vec::new();
        self.v_sym_index = None;
        queue.push(self.v[0].clone());

        while index < self.v.len() {
            // if sym_index is None, then we are looking for the first line that is a mirror of the previous line
            match sym_index {
                None => {
                    if self.v[index] == self.v[index - 1] {
                        sym_index = Some(index);
                        self.v_sym_index = Some(index);
                        discarded.push(queue.pop().unwrap());
                    } else {
                        queue.push(self.v[index].clone());
                    }
                }
                Some(_i) => {
                    if self.v[index] == self.v[index - 1] && next_sym_index == None {
                        next_sym_index = Some(index);
                    }

                    let el = queue.pop();
                    match el {
                        Some(line) => {
                            // is the same as current line?
                            if line.eq(&self.v[index]) {
                                discarded.push(line);
                            } else {
                                // reset to next sym_index

                                sym_index = next_sym_index;
                                self.v_sym_index = next_sym_index;

                                // flush discarded q
                                discarded.clear();

                                // take slice of queue from start to next_sym_index
                                if next_sym_index != None {
                                    queue =
                                        Vec::from(self.v[0..next_sym_index.unwrap() - 1].to_vec());
                                    // reset the index to next_sym_index
                                    index = next_sym_index.unwrap();
                                    next_sym_index = None;
                                } else {
                                    queue = Vec::from(self.v[0..index + 1].to_vec());
                                }
                            }
                        }
                        None => {
                            // we've found the mirror early
                            break;
                        }
                    }
                }
            }

            index += 1;
        }
    }
}

fn char_diff(s1: &str, s2: &str) -> usize {
    s1.chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_diff() {
        let s1: &str = "###";
        let s2: &str = "###";
        let s3: &str = "##.";
        let s4: &str = ".##";

        assert_eq!(char_diff(s1, s2), 0);
        assert_eq!(char_diff(s1, s3), 1);
        assert_eq!(char_diff(s3, s4), 2);
    }

    #[test]
    fn test_find_horizontal_symmetry_v2() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };

        p.add_horizontal(String::from("#.##..##."));
        p.add_horizontal(String::from("..#.##.#."));
        p.add_horizontal(String::from("##......#"));
        p.add_horizontal(String::from("##......#"));
        p.add_horizontal(String::from("..#.##.#."));
        p.add_horizontal(String::from("..##..##."));
        p.add_horizontal(String::from("#.#.##.#."));

        p.find_horizontal_symmetry_v2();

        assert_eq!(p.h_sym_index, Some(3));
    }

    #[test]
    fn test_find_horizontal_symmetry_v2_2() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };

        p.add_horizontal(String::from("#...##..#"));
        p.add_horizontal(String::from("#....#..#"));
        p.add_horizontal(String::from("..##..###"));
        p.add_horizontal(String::from("#####.##."));
        p.add_horizontal(String::from("#####.##."));
        p.add_horizontal(String::from("..##..###"));
        p.add_horizontal(String::from("#....#..#"));

        p.find_horizontal_symmetry_v2();

        assert_eq!(p.h_sym_index, Some(1));
    }

    #[test]
    fn test_find_horizontal_symmetry_v2_3() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };

        p.add_horizontal(String::from("..#..#..###"));
        p.add_horizontal(String::from("#.#.#...###"));
        p.add_horizontal(String::from("#.#.#...###"));
        p.add_horizontal(String::from("..#..#..###"));
        p.add_horizontal(String::from("#...#..##.."));
        p.add_horizontal(String::from("#..##.#...."));
        p.add_horizontal(String::from(".#.###.#.##"));
        p.add_horizontal(String::from("#.....#...."));
        p.add_horizontal(String::from("##.##.##..#"));
        p.add_horizontal(String::from("#..##.##..#"));
        p.add_horizontal(String::from("#.....#...."));

        p.find_horizontal_symmetry_v2();

        assert_eq!(p.h_sym_index, Some(9));
    }

    #[test]
    fn test_find_horizontal_symmetry_v2_4() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };

        p.add_horizontal(String::from("..#..#."));
        p.add_horizontal(String::from("#....#."));
        p.add_horizontal(String::from("...##.."));
        p.add_horizontal(String::from(".##.#.."));
        p.add_horizontal(String::from(".####.."));
        p.add_horizontal(String::from("..#.#.#"));
        p.add_horizontal(String::from("....###"));
        p.add_horizontal(String::from("#..#.#."));
        p.add_horizontal(String::from("#..#.#."));
        p.add_horizontal(String::from("....###"));
        p.add_horizontal(String::from("..#.#.#"));
        p.add_horizontal(String::from(".####.."));
        p.add_horizontal(String::from(".##.#.."));

        p.find_horizontal_symmetry_v2();

        assert_eq!(p.h_sym_index, Some(12));
    }

    #[test]
    fn test_find_vertical_symmetry_v2() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };

        p.add_horizontal(String::from("####..######..#"));
        p.add_horizontal(String::from(".###..#.##.#..#"));
        p.add_horizontal(String::from("....##..##..##."));
        p.add_horizontal(String::from("#.#....#..#...."));
        p.add_horizontal(String::from("..##..#.##.#..#"));
        p.add_horizontal(String::from("#.##..#.##.#..#"));
        p.add_horizontal(String::from("#...#..#..#..#."));
        p.add_horizontal(String::from("####..##..##..#"));
        p.add_horizontal(String::from(".###..######..#"));

        p.find_vertical_symmetry_v2();

        assert_eq!(p.v_sym_index, Some(13));
    }

    #[test]
    fn test_add_horizontal() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };

        p.add_horizontal(String::from("..."));
        p.add_horizontal(String::from("###"));
        p.add_horizontal(String::from("###"));
        p.add_horizontal(String::from("..."));

        assert_eq!(p.h, vec!["...", "###", "###", "..."]);
        assert_eq!(p.v, vec![".##.", ".##.", ".##."]);
    }

    #[test]
    fn test_day_1() {
        let mut patterns: Vec<Pattern> = Vec::new();
        let mut lines: Vec<String> = Vec::new();
        lines.push(String::from("#.##..##."));
        lines.push(String::from("..#.##.#."));
        lines.push(String::from("##......#"));
        lines.push(String::from("##......#"));
        lines.push(String::from("..#.##.#."));
        lines.push(String::from("..##..##."));
        lines.push(String::from("#.#.##.#."));
        lines.push(String::from(""));
        lines.push(String::from("#...##..#"));
        lines.push(String::from("#....#..#"));
        lines.push(String::from("..##..###"));
        lines.push(String::from("#####.##."));
        lines.push(String::from("#####.##."));
        lines.push(String::from("..##..###"));
        lines.push(String::from("#....#..#"));
        lines.push(String::from(""));
        lines.push(String::from(".#.##.#.#"));
        lines.push(String::from(".##..##.."));
        lines.push(String::from(".#.##.#.."));
        lines.push(String::from("#......##"));
        lines.push(String::from("#......##"));
        lines.push(String::from(".#.##.#.."));
        lines.push(String::from(".##..##.#"));
        lines.push(String::from(""));
        lines.push(String::from("#..#....#"));
        lines.push(String::from("###..##.."));
        lines.push(String::from(".##.#####"));
        lines.push(String::from(".##.#####"));
        lines.push(String::from("###..##.."));
        lines.push(String::from("#..#....#"));
        lines.push(String::from("#..##...#"));
        lines.push(String::from(""));
        lines.push(String::from("#.##..##."));
        lines.push(String::from("..#.##.#."));
        lines.push(String::from("##..#...#"));
        lines.push(String::from("##...#..#"));
        lines.push(String::from("..#.##.#."));
        lines.push(String::from("..##..##."));
        lines.push(String::from("#.#.##.#."));

        let result = get_part_1_result(&lines, &mut patterns);
        // Part one answer: 709
        // Part two answer: 1400
        assert_eq!(patterns.len(), 5);
        assert_eq!(result, 709);
    }

    #[test]
    fn test_day_1_2() {
        let mut patterns: Vec<Pattern> = Vec::new();
        let mut lines: Vec<String> = Vec::new();

        lines.push(String::from("###.##.##"));
        lines.push(String::from("##.####.#"));
        lines.push(String::from("##.#..#.#"));
        lines.push(String::from("####..###"));
        lines.push(String::from("....##..."));
        lines.push(String::from("##.#..#.#"));
        lines.push(String::from("...#..#.."));
        lines.push(String::from("##..###.#"));
        lines.push(String::from("##......#"));
        lines.push(String::from("##......#"));
        lines.push(String::from("..#.##.#."));
        lines.push(String::from("...#..#.."));
        lines.push(String::from("##.####.#"));
        lines.push(String::from("....##..."));
        lines.push(String::from("...####.."));
        lines.push(String::from("....##..."));
        lines.push(String::from("##.####.#"));
        lines.push(String::from(""));
        lines.push(String::from(".##.##...##...##."));
        lines.push(String::from("#####..##..##..##"));
        lines.push(String::from(".....##..##..##.."));
        lines.push(String::from(".##.#.#.####.#.#."));
        lines.push(String::from(".##...#.#..#.#..."));
        lines.push(String::from("....#..........#."));
        lines.push(String::from("#..#..#......#..#"));
        lines.push(String::from("....###.....####."));
        lines.push(String::from(".##...#.#..#.#..."));
        lines.push(String::from(".....#..####..#.."));
        lines.push(String::from("#..#...##..##...#"));
        lines.push(String::from("....#...#..#...#."));
        lines.push(String::from("#..#.##########.#"));
        lines.push(String::from("#..##...####...##"));

        let result = get_part_1_result(&lines, &mut patterns);

        assert_eq!(patterns.len(), 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_find_symmetries_0() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };

        p.add_horizontal(String::from(".....#."));
        p.add_horizontal(String::from("#.##..."));
        p.add_horizontal(String::from("#.##..."));
        p.add_horizontal(String::from("..#..#."));
        p.add_horizontal(String::from("..#..#."));
        p.add_horizontal(String::from("#.##..."));
        p.add_horizontal(String::from("#.##..."));
        p.add_horizontal(String::from(".....#."));
        p.add_horizontal(String::from("##..#.#"));

        p.find_symmetries();

        assert_eq!(p.h_sym_index, Some(4));
        assert_eq!(p.v_sym_index, None);
    }

    // #..#.####
    // .#.#.###.
    // .###.###.
    // #..#.####
    // #...#...#
    // #...#...#
    // #..#.####
    #[test]
    fn test_find_symmetries() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };

        p.add_horizontal(String::from("#..#.####"));
        p.add_horizontal(String::from(".#.#.###."));
        p.add_horizontal(String::from(".###.###."));
        p.add_horizontal(String::from("#..#.####"));
        p.add_horizontal(String::from("#...#...#"));
        p.add_horizontal(String::from("#...#...#"));
        p.add_horizontal(String::from("#..#.####"));

        p.find_symmetries();

        assert_eq!(p.h_sym_index, Some(5));
        assert_eq!(p.v_sym_index, None);
    }

    // #....#.
    // #....##
    // .####..
    // .####..
    // #....##
    // #.##.#.
    // .####.#
    #[test]
    fn test_find_symmetries_2() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };

        p.add_horizontal(String::from("#....#."));
        p.add_horizontal(String::from("#....##"));
        p.add_horizontal(String::from(".####.."));
        p.add_horizontal(String::from(".####.."));
        p.add_horizontal(String::from("#....##"));
        p.add_horizontal(String::from("#.##.#."));
        p.add_horizontal(String::from(".####.#"));

        p.find_symmetries();

        assert_eq!(p.h_sym_index, None);
        assert_eq!(p.v_sym_index, Some(3));
    }

    // .#...####.##..#
    // ..#.###...#.##.
    // ..#.###...#.##.
    // .#...####.##..#
    // .####..##.##..#
    // .####.#.###.##.
    // ####..#.#######
    // #.......####..#
    // ####.#.#.......
    // ..#....##.#####
    // .##.##...##.###
    // ..#.#.#.##.....
    // .....##.#..####
    // ##..#.###..####
    // .##..#...######
    #[test]
    fn test_find_symmetries_3() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };

        p.add_horizontal(String::from(".#...####.##..#"));
        p.add_horizontal(String::from("..#.###...#.##."));
        p.add_horizontal(String::from("..#.###...#.##."));
        p.add_horizontal(String::from(".#...####.##..#"));
        p.add_horizontal(String::from(".####..##.##..#"));
        p.add_horizontal(String::from(".####.#.###.##."));
        p.add_horizontal(String::from("####..#.#######"));
        p.add_horizontal(String::from("#.......####..#"));
        p.add_horizontal(String::from("####.#.#......."));
        p.add_horizontal(String::from("..#....##.#####"));
        p.add_horizontal(String::from(".##.##...##.###"));
        p.add_horizontal(String::from("..#.#.#.##....."));
        p.add_horizontal(String::from(".....##.#..####"));
        p.add_horizontal(String::from("##..#.###..####"));
        p.add_horizontal(String::from(".##..#...######"));

        p.find_symmetries();

        assert_eq!(p.h_sym_index, Some(2));
        assert_eq!(p.v_sym_index, None);
    }

    // ....
    // ....
    // #.#.
    #[test]
    fn test_find_symmetries_4() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };

        p.add_horizontal(String::from("...."));
        p.add_horizontal(String::from("...."));
        p.add_horizontal(String::from("#.#."));

        p.find_symmetries();

        assert_eq!(p.h_sym_index, Some(1));
        assert_eq!(p.v_sym_index, None);
    }

    // #.#.
    // ....
    // ....
    #[test]
    fn test_find_symmetries_5() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };

        p.add_horizontal(String::from("#.#."));
        p.add_horizontal(String::from("...."));
        p.add_horizontal(String::from("...."));

        p.find_symmetries();

        assert_eq!(p.h_sym_index, Some(2));
        assert_eq!(p.v_sym_index, None);
    }

    // .#..
    // #...
    // #...
    // ##..
    #[test]
    fn test_find_symmetries_6() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };
        p.add_horizontal(String::from(".#.."));
        p.add_horizontal(String::from("#..."));
        p.add_horizontal(String::from("#..."));
        p.add_horizontal(String::from("##.."));

        p.find_symmetries();

        assert_eq!(p.h_sym_index, None);
        assert_eq!(p.v_sym_index, Some(3));
    }

    // ...#
    // ..#.
    // ..#.
    // ..##
    #[test]
    fn test_find_symmetries_7() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };
        p.add_horizontal(String::from("...#"));
        p.add_horizontal(String::from("..#."));
        p.add_horizontal(String::from("..#."));
        p.add_horizontal(String::from("..##"));

        p.find_symmetries();

        assert_eq!(p.h_sym_index, None);
        assert_eq!(p.v_sym_index, Some(1));
    }

    #[test]
    fn test_find_vertical_symmetry() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };

        p.add_horizontal(String::from("..."));
        p.add_horizontal(String::from("###"));
        p.add_horizontal(String::from("###"));
        p.add_horizontal(String::from("..."));

        p.find_vertical_symmetry();

        assert_eq!(p.v_sym_index, Some(1));
    }

    #[test]
    fn test_find_horizontal_symmetry() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };

        p.add_horizontal(String::from("..."));
        p.add_horizontal(String::from("###"));
        p.add_horizontal(String::from("###"));
        p.add_horizontal(String::from("..."));

        p.find_horizontal_symmetry();

        assert_eq!(p.h_sym_index, Some(2));
    }

    #[test]
    fn test_find_horizontal_symmetry_2() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };

        p.add_horizontal(String::from("..."));
        p.add_horizontal(String::from("###"));
        p.add_horizontal(String::from("###"));
        p.add_horizontal(String::from("..."));
        p.add_horizontal(String::from("###"));

        p.find_horizontal_symmetry();

        assert_eq!(p.h_sym_index, Some(2));
    }

    #[test]
    fn test_find_horizontal_symmetry_3() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };

        p.add_horizontal(String::from("###"));
        p.add_horizontal(String::from("..."));
        p.add_horizontal(String::from("###"));
        p.add_horizontal(String::from("###"));
        p.add_horizontal(String::from("..."));

        p.find_horizontal_symmetry();

        assert_eq!(p.h_sym_index, Some(3));
    }

    #[test]
    fn test_find_horizontal_symmetry_4() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };

        p.add_horizontal(String::from("..."));
        p.add_horizontal(String::from("###"));
        p.add_horizontal(String::from("###"));
        p.add_horizontal(String::from("..."));
        p.add_horizontal(String::from("###"));
        p.add_horizontal(String::from("###"));

        p.find_horizontal_symmetry();

        assert_eq!(p.h_sym_index, Some(2));
    }

    #[test]
    fn test_find_horizontal_symmetry_5() {
        let mut p: Pattern = Pattern {
            h: Vec::new(),
            v: Vec::new(),
            h_sym_index: None,
            v_sym_index: None,
        };

        p.add_horizontal(String::from("..."));
        p.add_horizontal(String::from("..."));

        p.find_horizontal_symmetry();

        assert_eq!(p.h_sym_index, Some(1));
    }
}

fn main() {
    let mut patterns: Vec<Pattern> = Vec::new();

    // read the input file
    let path: &Path = Path::new("input.txt");
    let file: File = File::open(&path).unwrap();
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                lines.push(line);
            }
            Err(error) => {
                println!("Error reading line: {}", error);
            }
        }
    }

    let start = std::time::Instant::now();
    let result = get_part_1_result(&lines, &mut patterns);
    println!("Result: {}", result);
    println!("Time taken: {:?}", start.elapsed());
    println!();

    patterns.clear();
    let start = std::time::Instant::now();
    let result_v2 = get_part_2_result(&lines, &mut patterns);
    println!("Result_v2: {}", result_v2);
    println!("Time taken: {:?}", start.elapsed());
}

fn get_part_2_result(input_lines: &Vec<String>, patterns: &mut Vec<Pattern>) -> usize {
    let mut result_h: usize = 0;
    let mut result_v: usize = 0;

    let mut cur_pattern: Pattern = Pattern {
        h: Vec::new(),
        v: Vec::new(),
        h_sym_index: None,
        v_sym_index: None,
    };

    for line in input_lines {
        if line == "" {
            cur_pattern.find_symmetries_v2();
            // println!("{:#?}", cur_pattern);
            if cur_pattern.h_sym_index != None {
                result_h += cur_pattern.h_sym_index.unwrap();
            }
            if cur_pattern.v_sym_index != None {
                result_v += cur_pattern.v_sym_index.unwrap();
            }

            patterns.push(cur_pattern);

            cur_pattern = Pattern {
                h: Vec::new(),
                v: Vec::new(),
                h_sym_index: None,
                v_sym_index: None,
            };
            // break;
            continue;
        }

        cur_pattern.add_horizontal(line.clone());
    }

    cur_pattern.find_symmetries_v2();

    if cur_pattern.h_sym_index != None && cur_pattern.v_sym_index != None {
        panic!("Both symmetries found: {:#?}", cur_pattern);
    }

    if cur_pattern.h_sym_index == None && cur_pattern.v_sym_index == None {
        panic!("No symmetries found: {:#?}", cur_pattern);
    }

    if cur_pattern.h_sym_index != None {
        result_h += cur_pattern.h_sym_index.unwrap();
    }
    if cur_pattern.v_sym_index != None {
        result_v += cur_pattern.v_sym_index.unwrap();
    }
    patterns.push(cur_pattern);

    println!("Horizontal result: {}", result_h);
    println!("Vertical result: {}", result_v);

    let result = 100 * result_h + result_v;
    result
}

fn get_part_1_result(input_lines: &Vec<String>, patterns: &mut Vec<Pattern>) -> usize {
    let mut result_h: usize = 0;
    let mut result_v: usize = 0;

    let mut cur_pattern: Pattern = Pattern {
        h: Vec::new(),
        v: Vec::new(),
        h_sym_index: None,
        v_sym_index: None,
    };

    for line in input_lines {
        if line == "" {
            cur_pattern.find_symmetries();
            // println!("{:#?}", cur_pattern);
            if cur_pattern.h_sym_index != None {
                result_h += cur_pattern.h_sym_index.unwrap();
            }
            if cur_pattern.v_sym_index != None {
                result_v += cur_pattern.v_sym_index.unwrap();
            }

            patterns.push(cur_pattern);

            cur_pattern = Pattern {
                h: Vec::new(),
                v: Vec::new(),
                h_sym_index: None,
                v_sym_index: None,
            };
            // break;
            continue;
        }

        cur_pattern.add_horizontal(line.clone());
    }

    cur_pattern.find_symmetries();

    if cur_pattern.h_sym_index != None && cur_pattern.v_sym_index != None {
        println!("Both symmetries found: {:#?}", cur_pattern);
    }

    if cur_pattern.h_sym_index != None {
        result_h += cur_pattern.h_sym_index.unwrap();
    }
    if cur_pattern.v_sym_index != None {
        result_v += cur_pattern.v_sym_index.unwrap();
    }
    patterns.push(cur_pattern);

    println!("Horizontal result: {}", result_h);
    println!("Vertical result: {}", result_v);

    let result = 100 * result_h + result_v;
    result
}
