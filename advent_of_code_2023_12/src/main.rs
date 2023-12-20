use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let mut result = 0;
    let mut result_v2 = 0;
    let mut lines: Vec<String> = Vec::new();

    // read the input file
    let path: &Path = Path::new("input.txt");
    let file: File = File::open(&path).unwrap();
    let reader: io::BufReader<File> = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                lines.push(line);
                // result_v2 += get_number_of_arrangements_v2(line.as_str());
            }
            Err(error) => {
                println!("Error reading line: {}", error);
            }
        }
    }

    let start = Instant::now();
    for line in &lines {
        result += get_number_of_arrangements(line.as_str());
    }
    let duration = start.elapsed();

    let start_v2 = Instant::now();
    for line in &lines {
        result_v2 += get_number_of_arrangements_v2(line.as_str());
    }
    let duration_v2 = start_v2.elapsed();

    println!("Result: {}", result);
    println!("Result v2: {}", result_v2);

    println!("Finished after {:?}", duration);
    println!("Finished_v2 after {:?}", duration_v2);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_get_arrangement_number_format() {
    //     assert_eq!(get_arrangement_number_format("#.#"), vec![1, 1]);
    //     assert_eq!(
    //         get_arrangement_number_format("#.#.#.#.#.#.#.#.#"),
    //         vec![1, 1, 1, 1, 1, 1, 1, 1, 1]
    //     );
    //     assert_eq!(get_arrangement_number_format("#.###.#"), vec![1, 3, 1]);
    //     assert_eq!(get_arrangement_number_format("#..##......#"), vec![1, 2, 1]);
    // }

    #[test]
    fn test_validate_char_arrangement_against_record_char_format() {
        assert_eq!(
            validate_char_arrangement_against_record_char_format("#.#", "#.#"),
            true
        );
        assert_eq!(
            validate_char_arrangement_against_record_char_format(
                "#.#.#.#.#.#.#.#.#",
                "#.#.??#.#.#.#.#.#"
            ),
            true
        );
        assert_eq!(
            validate_char_arrangement_against_record_char_format("#.###.#", "#.#??.#"),
            true
        );
        assert_eq!(
            validate_char_arrangement_against_record_char_format("#..##......#", "#..##..??..#"),
            true
        );
        assert_eq!(
            validate_char_arrangement_against_record_char_format("#..##......#", "#..?#......#"),
            true
        );
        assert_eq!(
            validate_char_arrangement_against_record_char_format("#..##......#", "#..##....???"),
            true
        );
        assert_eq!(
            validate_char_arrangement_against_record_char_format("#..##......#", "#..##??....#"),
            true
        );
        assert_eq!(
            validate_char_arrangement_against_record_char_format("#..##......#", "???##......#"),
            true
        );
        assert_eq!(
            validate_char_arrangement_against_record_char_format("#..##......#", "????????????"),
            true
        );
        assert_eq!(
            validate_char_arrangement_against_record_char_format("#..##......#", "#..##.??????"),
            true
        );
        assert_eq!(
            validate_char_arrangement_against_record_char_format("#..##......#", "??????.....#"),
            true
        );
    }
    // ???.### 1,1,3 - 1 arrangement
    // #[test]
    // fn test_1() {
    //     // ???.### 1,1,3 - 1 arrangement
    //     let condition_record = "???.### 1,1,3";
    //     let arrangement = "#.#.###";
    //     assert_eq!(is_valid_arrangement(arrangement, condition_record), true);
    // }

    // .??..??...?##. 1,1,3 - 4 arrangements
    // #[test]
    // fn test_2() {
    //     // .??..??...?##. 1,1,3 - 4 arrangements
    //     let condition_record = ".??..??...?##. 1,1,3";
    //     let arrangement = ".#...#....###.";
    //     assert_eq!(is_valid_arrangement(arrangement, condition_record), true);
    // }

    #[test]
    fn test_get_start_offsets() {
        assert_eq!(get_start_offsets(&vec![1, 1, 3]), vec![0, 2, 4]);
        assert_eq!(
            get_start_offsets(&vec![1, 1, 1, 1, 1, 1, 1, 1, 1]),
            vec![0, 2, 4, 6, 8, 10, 12, 14, 16]
        );
        assert_eq!(get_start_offsets(&vec![1, 3, 1]), vec![0, 2, 6]);
    }

    #[test]
    fn test_get_arrangement_from_offsets_and_number_format() {
        assert_eq!(
            get_arrangement_from_offsets_and_number_format(&vec![0, 2, 4], &vec![1, 1, 3], 7),
            "#.#.###"
        );
        assert_eq!(
            get_arrangement_from_offsets_and_number_format(&vec![0, 2, 6], &vec![1, 3, 1], 7),
            "#.###.#"
        );
    }

    // ???.### 1,1,3 - 1 arrangement
    #[test]
    fn test_number_of_arrangements_1() {
        let condition_record = "???.### 1,1,3";
        assert_eq!(get_number_of_arrangements(condition_record), 1);
    }

    // .??..??...?##. 1,1,3 - 4 arrangements
    #[test]
    fn test_number_of_arrangements_2() {
        let condition_record = ".??..??...?##. 1,1,3";
        assert_eq!(get_number_of_arrangements(condition_record), 4);
    }

    // ?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
    #[test]
    fn test_number_of_arrangements_3() {
        let condition_record = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(get_number_of_arrangements(condition_record), 1);
    }
    // ????.#...#... 4,1,1 - 1 arrangement
    #[test]
    fn test_number_of_arrangements_4() {
        let condition_record = "????.#...#... 4,1,1";
        assert_eq!(get_number_of_arrangements(condition_record), 1);
    }
    // ????.######..#####. 1,6,5 - 4 arrangements
    #[test]
    fn test_number_of_arrangements_5() {
        let condition_record = "????.######..#####. 1,6,5";
        assert_eq!(get_number_of_arrangements(condition_record), 4);
    }
    // ?###???????? 3,2,1 - 10 arrangements
    #[test]
    fn test_number_of_arrangements_6() {
        let condition_record = "?###???????? 3,2,1";
        assert_eq!(get_number_of_arrangements(condition_record), 10);
    }

    #[test]
    fn test_get_next_offsets() {
        // #.#. [0, 2] -> #..# [0, 3] | 1,1 length 4
        assert_eq!(get_next_offsets(&vec![0, 2], &vec![1, 1], 4), vec![0, 3]);
        // #..# [0, 3] -> .#.# [1, 3] | 1,1 length 4
        assert_eq!(get_next_offsets(&vec![0, 3], &vec![1, 1], 4), vec![1, 3]);
        // .#.# [1, 3] -> .#.# [1, 3] | 1,1 length 4
        assert_eq!(get_next_offsets(&vec![1, 3], &vec![1, 1], 4), vec![1, 3]);

        // ##.. [0] -> .##. [1] | 2 length 4
        assert_eq!(get_next_offsets(&vec![0], &vec![2], 4), vec![1]);
        // .##. [1] -> ..## [2] | 2 length 4
        assert_eq!(get_next_offsets(&vec![1], &vec![2], 4), vec![2]);
        // ..## [2] -> ..## [2] | 2 length 4
        assert_eq!(get_next_offsets(&vec![2], &vec![2], 4), vec![2]);

        // #.#.### [0, 2, 4] -> #.#.### [0, 2, 4] | 1,1,3 length 7
        assert_eq!(
            get_next_offsets(&vec![0, 2, 4], &vec![1, 1, 3], 7),
            vec![0, 2, 4]
        );
        // #...#.### [0,4,6] -> .#.#.###. [1,3,5] | 1,1,3 length 9
        assert_eq!(
            get_next_offsets(&vec![0, 4, 6], &vec![1, 1, 3], 9),
            vec![1, 3, 5]
        );
    }
}

fn get_next_offsets(offsets: &Vec<usize>, number_format: &Vec<usize>, length: usize) -> Vec<usize> {
    let mut new_offsets = offsets.clone();
    let mut last_boundary = length;
    // try to move the last offset
    for i in (0..number_format.len()).rev() {
        if offsets[i] + number_format[i] < last_boundary {
            new_offsets[i] += 1;
            return new_offsets;
        } else {
            // try moving it all the way to the left + 1
            if i == 0 {
                // special case for the first offset
                return new_offsets;
            }
            if offsets[i - 1] + number_format[i - 1] + 1 == offsets[i] {
                // need to stay in place, cannot jump more
                last_boundary = new_offsets[i] - 1;
                continue;
            }
            let old_offset = new_offsets[i];
            new_offsets[i] = offsets[i - 1] + number_format[i - 1] + 2;

            let diff = old_offset - new_offsets[i];
            // update all the offsets to the right by the difference
            for j in i + 1..number_format.len() {
                new_offsets[j] -= diff;
            }

            last_boundary = new_offsets[i];
        }
    }

    // if not possible, try to move the second last offset and move over the last offset
    return new_offsets;
}

fn get_number_of_arrangements_v2(condition_record: &str) -> usize {
    let condition_record_v2: String;
    let mut split = condition_record.split(" ");
    let record_char_format = split.next().unwrap();

    let repeated_record_char = std::iter::repeat(record_char_format)
        .take(5)
        .collect::<Vec<&str>>()
        .join("?");

    let number_format = split.next().unwrap();
    let repeated_number_format = std::iter::repeat(number_format)
        .take(5)
        .collect::<Vec<&str>>()
        .join(",");

    condition_record_v2 = repeated_record_char + " " + repeated_number_format.as_str();

    println!("{}", condition_record_v2.as_str());
    return get_number_of_arrangements(condition_record_v2.as_str());
}

struct DamagedSpringsGroup {
    number: usize,
    placement: usize,
}

fn get_number_of_arrangements(condition_record: &str) -> usize {
    let (record_char_format, record_number_format_vec) =
        interpret_condition_record(condition_record);
    let mut number_of_arrangements = 0;
    let mut groups: Vec<DamagedSpringsGroup> = Vec::new();
    let mut i: usize = 0;
    for number in record_number_format_vec {
        // let's find a place to put this group
        // check if we can place at current i
        let mut count = 0;
        for j in i..record_char_format.len() {
            let c = record_char_format.chars().nth(j).unwrap();
            if c != '?' && c != '#' {
                // cannot place here, try next
                count = 0;
                i = j + 1;
                continue;
            } else {
                count += 1;
            }
            if count == number {
                break;
            }
        }

        groups.push(DamagedSpringsGroup {
            number: number,
            placement: i,
        });

        i += 2; // next group needs to be separated by at least one dot
    }

    number_of_arrangements = 1;

    let mut moved = true;

    while moved {
        moved = false;

        // create next arrangement
        // try moving a group to the right, if possible, starting from last group till first group
        let mut last_boundary = record_char_format.len();
        let mut moved_group_index = 0;
        for (group_index, group) in groups.iter_mut().rev().enumerate() {
            if moved {
                break;
            }

            if group.placement + group.number < last_boundary {
                let mut count = 0;
                let mut j: usize = group.placement + 1;
                for i in group.placement + 1..last_boundary {
                    let c = record_char_format.chars().nth(i).unwrap();
                    if c == '#' || c == '?' {
                        count += 1;
                    } else {
                        j = i;
                        continue;
                    }

                    if count == group.number {
                        moved = true;
                        number_of_arrangements += 1;
                        moved_group_index = group_index;
                        group.placement = j;
                        break;
                    }
                }
                
                
                // break;
            } else {
                if group.placement == 0 {
                    break;
                }
                last_boundary = group.placement - 1;
            }
        }
        
        if moved && moved_group_index < groups.len() - 1 {
            // now move all the groups to the right next to this moved group (reset them)
            let mut count = 0;
            let mut g_i = moved_group_index + 1;
            let mut c_j: usize = groups[g_i].placement + 1;
            for c_i in groups[moved_group_index].placement + groups[moved_group_index].number + 1..record_char_format.len() {
                let c = record_char_format.chars().nth(c_i).unwrap();
                if c == '#' || c == '?' {
                    count += 1;
                    if count == groups[g_i].number {
                        groups[g_i].placement = c_j;
                        c_j = c_i + 2;
                        g_i += 1;
                        if g_i == groups.len() {
                            break;
                        }
                    }
                } else {
                    c_j = c_i + 1;
                    count = 0;
                }
            }
        }
    }

    // old solution below ---------------------------------------------

    // let mut offsets: Vec<usize> = get_start_offsets(&record_number_format_vec);
    // let mut last_offsets: Vec<usize> = Vec::new();

    // while offsets.ne(&last_offsets) {
    //     let arrangement = get_arrangement_from_offsets_and_number_format(
    //         &offsets,
    //         &record_number_format_vec,
    //         record_char_format.len(),
    //     );
    //     if validate_char_arrangement_against_record_char_format(
    //         arrangement.as_str(),
    //         record_char_format,
    //     ) {
    //         number_of_arrangements += 1;
    //     }

    //     last_offsets = offsets.clone();
    //     // get next offsets
    //     offsets = get_next_offsets(
    //         &offsets,
    //         &record_number_format_vec,
    //         record_char_format.len(),
    //     );
    // }

    return number_of_arrangements;
}

fn get_arrangement_from_offsets_and_number_format(
    offsets: &Vec<usize>,
    number_format: &Vec<usize>,
    length: usize,
) -> String {
    let mut arrangement = ".".repeat(length);
    for (offset_index, offset) in offsets.iter().enumerate() {
        let from = offset;
        let to = offset + number_format[offset_index];
        arrangement.replace_range(from..&to, "#".repeat(to - from).as_str());
    }

    return arrangement;
}

fn get_start_offsets(record_number_format_vec: &Vec<usize>) -> Vec<usize> {
    let mut offsets: Vec<usize> = Vec::new();
    let mut index_sum: usize = 0;
    for i in 0..record_number_format_vec.len() {
        offsets.push(index_sum);
        index_sum += record_number_format_vec[i] + 1; // +1 for the dot so they don't touch
    }

    return offsets;
}

// fn is_valid_arrangement(arrangement: &str, condition_record: &str) -> bool {
//     let (record_char_format, record_number_format_vec) =
//         interpret_condition_record(condition_record);
//     let arrangement_number_format = get_arrangement_number_format(arrangement);
//     return validate_char_arrangement_against_record_char_format(arrangement, record_char_format);
// }

fn validate_char_arrangement_against_record_char_format(
    arrangement: &str,
    record_char_format: &str,
) -> bool {
    if arrangement.len() != record_char_format.len() {
        return false;
    }

    for (i, c) in arrangement.chars().enumerate() {
        if record_char_format.chars().nth(i).unwrap() == '?' {
            continue;
        }

        if c != record_char_format.chars().nth(i).unwrap() {
            return false;
        }
    }

    return true;
}

// get the number format of the arrangement
// e.g. #.# -> 1,1
// e.g. #.#.#.#.#.#.#.# -> 1,1,1,1,1,1,1,1
// e.g. #.###.# -> 1,3,1
// fn get_arrangement_number_format(arrangement: &str) -> Vec<usize> {
//     let mut number_format: Vec<usize> = Vec::new();
//     let mut count = 0;
//     for (i, c) in arrangement.chars().enumerate() {
//         if c == '#' {
//             count += 1;
//         } else {
//             if count > 0 {
//                 number_format.push(count);
//                 count = 0;
//             }
//         }
//     }

//     if count > 0 {
//         number_format.push(count);
//     }

//     return number_format;
// }

fn interpret_condition_record(condition_record: &str) -> (&str, Vec<usize>) {
    let mut split = condition_record.split(" ");
    let record_char_format = split.next().unwrap();
    let record_number_format = split.next().unwrap();
    let record_number_format_vec: Vec<usize> = record_number_format
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    return (record_char_format, record_number_format_vec);
}
