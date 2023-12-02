use std::fs;

const TEST_INPUT1: &str = "data/prob1_test_input.txt";
const INPUT1: &str = "data/prob1_text_input.txt";
const TEST_INPUT2: &str = "data/prob2_test_input.txt";
const INPUT2: &str = "data/prob2_text_input.txt";

const PROB1_INPUT: &str = INPUT1;
const PROB2_INPUT: &str = INPUT1;

fn prob1() {
    let puz_input = fs::read_to_string(PROB1_INPUT)
        .expect("Couldn't read text input.");

    let lines = puz_input.split("\n");

    let mut num = 0;
    let mut last_dig = 0;
    let mut first_found = false;
    let mut total = 0;

    for l in lines {
        for c in l.chars() {
            if c.is_ascii_digit() {
                if ! first_found {
                    num += 10 * c.to_digit(10).unwrap();
                    first_found = true;
                }
                num -= last_dig;
                last_dig = c.to_digit(10).unwrap();
                num += last_dig;
            }
        }
        total += num;
        num = 0;
        last_dig = 0;
        first_found = false;
    }

    println!("The sum is: {}", total);
}

fn prob2() {
    let puz_input = fs::read_to_string(PROB2_INPUT)
        .expect("Couldn't read text input.");

    let lines = puz_input.split("\n");

    let mut num = 0;
    let mut first_found = false;
    let mut total = 0;
    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut count = 0;
    let mut wtf = 0;

    for l in lines {
        wtf += 1;
        let mut first_pos = isize::MAX;
        let mut first_digit = 0;
        let mut last_pos = isize::MIN;
        let mut last_digit = 0;
        let mut last_digit_pos = 0;
        let mut last_dig = 0;
        count = 0;
        for text_dig in digits {
            count += 1;
            let f = l.find(text_dig);
            let e = l.rfind(text_dig);
            match f {
                Some(x) => {
                    if (x as isize) < first_pos {
                        first_pos = x as isize;
                        first_digit = count;
                    }
                },
                None => continue
            }
            match e {
                Some(x) => {
                    if (x as isize) > last_pos {
                        last_pos = x as isize;
                        last_digit = count;
                    }
                },
                None => continue
            }
        }
        count = 0;
        for c in l.chars() {
            if c.is_ascii_digit() {
                if (! first_found) && (count < first_pos) {
                    num += 10 * c.to_digit(10).unwrap();
                    first_found = true;
                }
                num -= last_dig;
                last_dig = c.to_digit(10).unwrap();
                num += last_dig;
                last_digit_pos = count;
            }
            count += 1;
        }
        if last_digit_pos < last_pos {
            num += last_digit as u32;
            num -= last_dig;
        }
        if ! first_found {
            num += 10 * first_digit as u32;
        }
        total += num;
        num = 0;
        first_found = false;
    }

    println!("The sum is: {}", total);
}

fn main() {
    prob2();
}
