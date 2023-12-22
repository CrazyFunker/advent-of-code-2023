use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Remove,
}

#[derive(Debug, Clone)]
struct Box {
    lenses: Vec<Lens>,
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    label_hash: u8,
    focal_length: u8,
}

#[derive(Debug, Clone)]
struct LensOperation {
    operation: Operation,
    step_value: String,
    lens: Lens,
    destination_box_label: u8,
}

fn get_part_2(steps: &Vec<&str>) -> u64 {
    let mut result: u64 = 0;
    let mut boxes: Vec<Box> = Vec::new();
    let mut operations: Vec<LensOperation> = Vec::new();
    // init boxes 0 -> 255
    for i in 0..256 {
        let mut lenses: Vec<Lens> = Vec::new();
        let mut one_box: Box = Box { lenses: lenses };
        boxes.push(one_box);
    }

    // prepare LensOperations
    for step in steps {
        let lens_operation: LensOperation;
        if step.contains('-') {
            let label = &step[0..&step.len() - 1]; // removing last '-'
            let label_hash: u8 = hash(label);
            let lens = Lens {
                label: label.to_string(),
                label_hash,
                focal_length: 0,
            };
            lens_operation = LensOperation {
                operation: Operation::Remove,
                step_value: step.to_string(),
                lens,
                destination_box_label: label_hash,
            };
        } else {
            let split: Vec<&str> = step.split('=').collect();
            let label_hash: u8 = hash(split[0]);
            let lens: Lens = Lens {
                label: split[0].to_string(),
                label_hash,
                focal_length: split[1].parse::<u8>().unwrap(),
            };
            lens_operation = LensOperation {
                operation: Operation::Add,
                step_value: step.to_string(),
                lens,
                destination_box_label: label_hash,
            };
        }
        operations.push(lens_operation);
    }

    // println!("operations: {:#?}", &operations);

    // execute LensOperations
    for operation in operations {
        match operation.operation {
            Operation::Add => {
                add_lens_to_box(&mut boxes, &operation.lens, operation.destination_box_label);
            }
            Operation::Remove => {
                remove_lens_from_box(&mut boxes, operation.lens, operation.destination_box_label);
            }
        }
        // println!("step: {}", operation.step_value);
        // for (i, one_box) in boxes.iter().enumerate() {
        //     if one_box.lenses.len() > 0 {
        //         println!("box {}: {:#?}", i, &one_box);
        //     }
        // }
    }

    // count focusing power of all lenses in all boxes
    //To confirm that all of the lenses are installed correctly, add up the focusing power of all of the lenses. The focusing power of a single lens is the result of multiplying together:
    // One plus the box number of the lens in question.
    // The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
    // The focal length of the lens.
    // At the end of the above example, the focusing power of each lens is as follows:

    // rn: 1 (box 0) * 1 (first slot) * 1 (focal length) = 1
    // cm: 1 (box 0) * 2 (second slot) * 2 (focal length) = 4
    // ot: 4 (box 3) * 1 (first slot) * 7 (focal length) = 28
    // ab: 4 (box 3) * 2 (second slot) * 5 (focal length) = 40
    // pc: 4 (box 3) * 3 (third slot) * 6 (focal length) = 72
    // So, the above example ends up with a total focusing power of 145.

    for (box_i, one_box) in boxes.iter().enumerate() {
        for (slot_i, lens) in one_box.lenses.iter().enumerate() {
            let focusing_power: u64 =
                (box_i as u64 + 1) * (slot_i as u64 + 1) * lens.focal_length as u64;
            result += focusing_power;
        }
    }

    result
}

fn add_lens_to_box(boxes: &mut Vec<Box>, lens: &Lens, destination_box_label: u8) {
    // println!("{}", destination_box_label);
    // find box with label
    let mut one_box = boxes.get_mut(destination_box_label as usize).unwrap();
    let mut found_lens = false;
    // add lens to box
    for box_lens in one_box.lenses.iter_mut() {
        if box_lens.label == lens.label {
            // lens already exists
            // update focal length
            box_lens.focal_length = lens.focal_length;
            found_lens = true;
        }
    }

    if !found_lens {
        one_box.lenses.push(lens.clone());
    }
}

fn remove_lens_from_box(boxes: &mut Vec<Box>, lens: Lens, destination_box_label: u8) {
    // find box with label
    let mut found_box = boxes.get_mut(destination_box_label as usize).unwrap();

    found_box
        .lenses
        .retain(|box_lens| box_lens.label != lens.label);
}

fn get_part_1(steps: &Vec<&str>) -> u64 {
    let mut result: u64 = 0;

    for step in steps {
        result += hash(step) as u64;
    }

    result
}

// The HASH algorithm is a way to turn any string of characters into a single number in the range 0 to 255. To run the HASH algorithm on a string, start with a current value of 0. Then, for each character in the string starting from the beginning:
//
// Determine the ASCII code for the current character of the string.
// Increase the current value by the ASCII code you just determined.
// Set the current value to itself multiplied by 17.
// Set the current value to the remainder of dividing itself by 256.
// After following these steps for each character in the string in order, the current value is the output of the HASH algorithm.

fn hash(s: &str) -> u8 {
    let mut current_value: u16 = 0;
    for c in s.chars() {
        let ascii_code = c as u16;
        current_value += ascii_code;
        current_value *= 17;
        current_value %= 256;
    }
    current_value as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_test() {
        let s = "HASH";
        let expected: u8 = 52;

        assert_eq!(hash(s), expected);
    }

    #[test]
    fn get_part_2_test() {
        let steps: Vec<&str> = vec![
            "rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6", "ot=7",
        ];
        // After "rn=1":
        // Box 0: [rn 1]

        // After "cm-":
        // Box 0: [rn 1]

        // After "qp=3":
        // Box 0: [rn 1]
        // Box 1: [qp 3]

        // After "cm=2":
        // Box 0: [rn 1] [cm 2]
        // Box 1: [qp 3]

        // After "qp-":
        // Box 0: [rn 1] [cm 2]

        // After "pc=4":
        // Box 0: [rn 1] [cm 2]
        // Box 3: [pc 4]

        // After "ot=9":
        // Box 0: [rn 1] [cm 2]
        // Box 3: [pc 4] [ot 9]

        // After "ab=5":
        // Box 0: [rn 1] [cm 2]
        // Box 3: [pc 4] [ot 9] [ab 5]

        // After "pc-":
        // Box 0: [rn 1] [cm 2]
        // Box 3: [ot 9] [ab 5]

        // After "pc=6":
        // Box 0: [rn 1] [cm 2]
        // Box 3: [ot 9] [ab 5] [pc 6]

        // After "ot=7":
        // Box 0: [rn 1] [cm 2]
        // Box 3: [ot 7] [ab 5] [pc 6]

        let result = get_part_2(&steps);
        assert_eq!(result, 145);
    }
}

fn main() {
    let mut all_steps: String = String::new();

    // read the input file
    let path: &Path = Path::new("input.txt");
    let file: File = File::open(&path).unwrap();
    let reader: io::BufReader<File> = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                all_steps.push_str(&line);
                break;
            }
            Err(error) => {
                println!("Error reading line: {}", error);
            }
        }
    }

    let steps: Vec<&str> = all_steps.split(",").collect();

    let start = Instant::now();
    let result = get_part_1(&steps);
    println!("Time: {:?}", start.elapsed());
    println!("Part 1: {}", result);

    let start = Instant::now();
    let result_v2 = get_part_2(&steps);
    println!("Time: {:?}", start.elapsed());
    println!("Part 2: {}", result_v2);
}
