use std::fs;

const INPUT1: &str = "data/input1.txt";

fn prob1(){
    let games = fs::read_to_string(INPUT1)
        .expect("Couldn't read input");
    let mut cond: [u32; 115] = [0; 115];
    cond[98] = 14;
    cond[103] = 13;
    cond[114] = 12;

    let mut result = 0;

    for game in games.lines() {
        let mut elems = game.split_ascii_whitespace();
        elems.next();
        let gameid: usize = match elems.next() {
            Some(x) => {
                let mut y = x.to_owned();
                y.pop();
                y.parse().unwrap()
            },
            None => break
        };

        let mut possible = true;

        while possible {
            let number: u32 = match elems.next() {
                Some(x) => x.parse().unwrap(),
                None => break
            };
            let category = elems.next().unwrap();
            let hash = category.as_bytes()[0];

            if number > cond[hash as usize] {
                possible = false;
                break;
            }
        }
        if possible {
            result += gameid;
        }
    }

    println!("The result is {}", result);
}

fn prob2(){
    let games = fs::read_to_string(INPUT1)
        .expect("Couldn't read input");
    let mut cond: [u32; 115] = [0; 115];
    cond[98] = <u32>::MIN;
    cond[103] = <u32>::MIN;
    cond[114] = <u32>::MIN;

    let mut result = 0;

    for game in games.lines() {
        let mut elems = game.split_ascii_whitespace();
        elems.next();
        let gameid: usize = match elems.next() {
            Some(x) => {
                let mut y = x.to_owned();
                y.pop();
                y.parse().unwrap()
            },
            None => break
        };

        let possible = true;

        while possible {
            let number: u32 = match elems.next() {
                Some(x) => x.parse().unwrap(),
                None => break
            };
            let category = elems.next().unwrap();
            let hash = category.as_bytes()[0];

            cond[hash as usize] = cond[hash as usize].max(number);
        }
        result += cond[98]*cond[103]*cond[114];
        cond[98] = <u32>::MIN;
        cond[103] = <u32>::MIN;
        cond[114] = <u32>::MIN;
    }

    println!("The result is {}", result);
}

fn main() {
    prob2();
}
