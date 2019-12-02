pub fn solve() {
    let input = include_str!("day02.txt");
    let p1 = day2p1(input, 12, 2);
    let p2 = day2p2(input, 19690720, 100, 100);
    println!("Day 1:\n\tp1: {}\n\tp2: {}", p1, p2);
}

fn day2p1(input: &str, noun: i32, verb: i32) -> i32 {
    // let mut sum = 0 as i32;
    let mut values: Vec<i32> = input.split(",")
        .map(|x| x.parse::<i32>().expect("can't parse value"))
        .collect();
    values[1] = noun;
    values[2] = verb;
    let mut address = 0;
    loop {
        match values[address] {
            1 => {
                let out = values[address+3];
                values[out as usize] = values[values[address+1] as usize] + values[values[address+2] as usize];
            },
            2 => {
                let out = values[address+3];
                values[out as usize] = values[values[address+1] as usize] * values[values[address+2] as usize];
            },
            99 => return values[0],
            _ => return -1
        }
        address += 4
    };
}

fn day2p2(input: &str, desired_output: i32, n_range: i32, v_range: i32) -> i32 {
    for n in 0..n_range {
        for v in 0..v_range {
            let output = day2p1(input, n, v);
            if output == desired_output {
                return (100 * n) + v
            }
        }
    }
    return -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn part1() {
        assert_eq!(3500, day2p1("1,9,10,3,2,3,11,0,99,30,40,50", 0, 0));
    }
    
    #[test]
    fn part2() {
        assert_eq!(910, day2p2("1,9,10,3,2,3,11,0,99,30,40,50", 3500, 12, 12));
    }
    
    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| day2p1("1,9,10,3,2,3,11,0,99,30,40,50", 0, 0));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| day2p2("1,9,10,3,2,3,11,0,99,30,40,50", 3500, 12, 12));
    }
}