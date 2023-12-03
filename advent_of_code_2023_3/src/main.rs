use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Number {
    start_index: usize,
    end_index: usize,
    value: u32,
}

#[derive(Debug, Clone)]
struct Gear {
    index: usize,
}

fn main() {
    // sliding column window, vector storing line above, current line and line below
    let mut previous_line: Vec<char>;
    let mut current_line: Vec<char> = Vec::new();
    let mut bottom_line: Vec<char> = Vec::new();
    let mut collapsed_special_chars: Vec<char> = Vec::new();
    let mut prev_numbers: Vec<Number>;
    let mut cur_numbers: Vec<Number> = Vec::new();
    let mut next_numbers: Vec<Number> = Vec::new();
    let mut cur_gears: Vec<Gear> = Vec::new();
    let mut sum: u32 = 0;
    let mut gears_sum: u32 = 0;

    // vector that stores collapsed special chars from above and below lines
    // get line by line

    // read the input file
    let path: &Path = Path::new("input.txt");
    let file: File = File::open(&path).unwrap();
    let reader: io::BufReader<File> = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                previous_line = current_line;
                current_line = bottom_line;
                bottom_line = line.chars().collect::<Vec<char>>();

                prev_numbers = cur_numbers;
                cur_numbers = next_numbers;
                // TODO need to interpret the numbers here
                next_numbers = interpret_numbers(&bottom_line);

                // special case for first line
                if current_line.len() == 0 {
                    continue;
                }

                sum += parse_line(
                    &mut cur_gears,
                    &mut collapsed_special_chars,
                    &mut cur_numbers,
                    &current_line,
                    &previous_line,
                    &bottom_line,
                );
                gears_sum += parse_gears_for_cur_line(
                    &mut cur_gears,
                    &mut prev_numbers,
                    &mut cur_numbers,
                    &mut next_numbers,
                );
            }
            Err(error) => {
                println!("Error reading line: {}", error);
            }
        }
    }

    // now we're at the last line, we need to check the last line with the algorithm
    previous_line = current_line.clone();
    current_line = bottom_line.clone();
    bottom_line = Vec::new();

    prev_numbers = cur_numbers;
    cur_numbers = next_numbers;
    next_numbers = Vec::new();

    sum += parse_line(
        &mut cur_gears,
        &mut collapsed_special_chars,
        &mut cur_numbers,
        &current_line,
        &previous_line,
        &bottom_line,
    );

    gears_sum += parse_gears_for_cur_line(
        &mut cur_gears,
        &mut prev_numbers,
        &mut cur_numbers,
        &mut next_numbers,
    );
    // println!("upperLine: {:?}", upperLine);
    // println!("currentLine: {:?}", currentLine);
    // println!("bottomLine: {:?}", bottomLine);

    // println!("prev_contiguous_digits: {:?}", prev_contiguous_digits);
    // println!("cur_contiguous_digits: {:?}", cur_contiguous_digits);
    // println!("next_contiguous_digits: {:?}", next_contiguous_digits);

    println!("sum: {}", sum);
    println!("sum_gears: {}", gears_sum);
}

fn interpret_numbers(line: &Vec<char>) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();
    let mut temp_digit: Vec<char> = Vec::new();
    let mut temp_digit_start_index: i32 = -1;

    for i in 0..line.len() {
        if line[i].is_digit(10) {
            if temp_digit_start_index == -1 {
                temp_digit_start_index = i as i32;
            }
            temp_digit.push(line[i]);
        } else {
            // if it's not a digit, time to unwrap the contiguous digit into a number and save its details
            temp_digit_to_contiguous_digit(
                &mut temp_digit,
                i,
                &mut numbers,
                &mut temp_digit_start_index,
            );
        }
    }

    // need to check if we have a contiguous digit at the end of the line
    temp_digit_to_contiguous_digit(
        &mut temp_digit,
        line.len(),
        &mut numbers,
        &mut temp_digit_start_index,
    );

    // println!("numbers: {:?}", numbers);
    return numbers;
}

fn parse_line(
    cur_gears: &mut Vec<Gear>,
    collapsed_special_chars: &mut Vec<char>,
    cur_numbers: &mut Vec<Number>,
    cur_line: &Vec<char>,
    prev_line: &Vec<char>,
    next_line: &Vec<char>,
) -> u32 {
    // prepare collapsed special chars vector
    collapsed_special_chars.clear();

    // prepare gears on line vector
    cur_gears.clear();

    // keep track of stars and gears
    let mut stars: Vec<char> = Vec::new();

    for i in 0..cur_line.len() {
        if cur_line[i] == '*' {
            cur_gears.push(Gear { index: i });
        } else {
            stars.push('.');
        }

        // collapse special chars
        if i < cur_line.len() && cur_line[i] != '.' && !cur_line[i].is_digit(10) {
            collapsed_special_chars.push(cur_line[i]);
        } else if i < prev_line.len() && prev_line[i] != '.' && !prev_line[i].is_digit(10) {
            collapsed_special_chars.push(prev_line[i]);
        } else if i < next_line.len() && next_line[i] != '.' && !next_line[i].is_digit(10) {
            collapsed_special_chars.push(next_line[i]);
        } else {
            collapsed_special_chars.push('.');
        }
    }

    // check each found number and see if it is adjacent to a special char
    return sum_up_numbers_adjacent_to_special_chars(
        cur_numbers,
        cur_line,
        collapsed_special_chars,
    );
}

fn parse_gears_for_cur_line(
    cur_gears: &mut Vec<Gear>,
    prev_digits: &mut Vec<Number>,
    cur_digits: &mut Vec<Number>,
    next_digits: &mut Vec<Number>,
) -> u32 {
    // println!("cur_gear.len():\t{}", cur_gears.len());
    // println!("\tprev_digits.len():\t{}", prev_digits.len());
    // println!("\tcur_digits.len():\t{}", cur_digits.len());
    // println!("\tnext_digits.len():\t{}", next_digits.len());

    let mut sum_gear_ratio: u32 = 0;

    for gear in cur_gears {
        let mut gear_ratio: u32 = 0;
        // collect surrounding digits
        let mut surrounding_digits: Vec<Number> = Vec::new();

        // prev digits
        for prev_digit in prev_digits.iter() {
            if prev_digit.start_index.saturating_sub(1) <= gear.index
                && gear.index <= prev_digit.end_index.saturating_add(1)
            {
                surrounding_digits.push(prev_digit.clone());
            }
        }

        // cur digits
        for cur_digit in cur_digits.iter() {
            if cur_digit.start_index.saturating_sub(1) <= gear.index
                && gear.index <= cur_digit.end_index.saturating_add(1)
            {
                surrounding_digits.push(cur_digit.clone());
            }
        }

        // next digits
        for next_digit in next_digits.iter() {
            if next_digit.start_index.saturating_sub(1) <= gear.index
                && gear.index <= next_digit.end_index.saturating_add(1)
            {
                surrounding_digits.push(next_digit.clone());
            }
        }

        // println!(
        //     "\t\t{} surrounding_digits: {:?}",
        //     surrounding_digits.len(),
        //     surrounding_digits
        // );

        if surrounding_digits.len() == 2 {
            gear_ratio = surrounding_digits[0].value * surrounding_digits[1].value;
            // println!(
            //     "\t\t\tgear_ratio: {} = {} * {}",
            //     gear_ratio, surrounding_digits[0].value, surrounding_digits[1].value
            // );
        }

        sum_gear_ratio += gear_ratio;
    }

    // println!("\t\tgear_ratio: {}", sum_gear_ratio);
    return sum_gear_ratio;
}

fn temp_digit_to_contiguous_digit(
    temp_digit: &mut Vec<char>,
    i: usize,
    contiguous_digits: &mut Vec<Number>,
    temp_digit_start_index: &mut i32,
) {
    if temp_digit.len() > 0 {
        let temp_digit_end_index: i32 = i as i32 - 1;
        let temp_digit_value: u32 = temp_digit
            .iter()
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        contiguous_digits.push(Number {
            start_index: *temp_digit_start_index as usize,
            end_index: temp_digit_end_index as usize,
            value: temp_digit_value,
        });
        temp_digit.clear();
        *temp_digit_start_index = -1;
    }
}

fn sum_up_numbers_adjacent_to_special_chars(
    contiguous_digits: &mut Vec<Number>,
    current_line: &Vec<char>,
    collapsed_special_chars: &mut Vec<char>,
) -> u32 {
    let mut sum: u32 = 0;

    for number in contiguous_digits.iter() {
        // get start-1 and end+1 index of string
        let adjusted_start_index: usize = if number.start_index == 0 {
            number.start_index
        } else {
            number.start_index - 1
        };
        let adjusted_end_index: usize = if number.end_index == current_line.len() - 1 {
            number.end_index
        } else {
            number.end_index + 1
        };

        for i in adjusted_start_index..=adjusted_end_index {
            if collapsed_special_chars[i] != '.' {
                // println!("{:?}", number);
                sum += number.value;
                break;
            }
        }
    }

    // println!("=========================== line sum: {}", sum);
    return sum;
}
