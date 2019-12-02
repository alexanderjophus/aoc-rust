pub fn solve() {
    let input = include_str!("day01.txt");
    let p1 = day1p1(input);
    let p2 = day1p2(input);
    println!("Day 1:\n\tp1: {}\n\tp2: {}", p1, p2);
}

fn day1p1(input: &str) -> i32 {
    // let mut sum = 0 as i32;
    input.lines()
        .map(|x| {
            let val = x.parse::<i32>().expect("can't parse value");
            val / 3 - 2
        })
        .sum()
}

fn day1p2(input: &str) -> i32 {
    input.lines()
        .map(|x| {
            let mut sum = 0 as i32;
            let mut val = x.parse::<i32>().expect("can't parse value");
            while val > 8{
                val = val / 3 - 2;
                sum += val;
            }
            sum
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn part1() {
        assert_eq!(2, day1p1("12"));
        assert_eq!(2, day1p1("14"));
        assert_eq!(654, day1p1("1969"));
        assert_eq!(33583, day1p1("100756"));
        assert_eq!(4, day1p1("12\n14"));
        assert_eq!(658, day1p1("12\n14\n1969"));
    }
    
    #[test]
    fn part2() {
        assert_eq!(2, day1p2("12"));
        assert_eq!(2, day1p2("14"));
        assert_eq!(966, day1p2("1969"));
        assert_eq!(50346, day1p2("100756"));
        assert_eq!(4, day1p2("12\n14"));
        assert_eq!(970, day1p2("12\n14\n1969"));
    }
    
    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| day1p1("12"));
        b.iter(|| day1p1("14"));
        b.iter(|| day1p1("1969"));
        b.iter(|| day1p1("100756"));
        b.iter(|| day1p1("12\n14"));
        b.iter(|| day1p1("12\n14\n1969"));
    }
    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| day1p2("12"));
        b.iter(|| day1p2("14"));
        b.iter(|| day1p2("1969"));
        b.iter(|| day1p2("100756"));
        b.iter(|| day1p2("12\n14"));
        b.iter(|| day1p2("12\n14\n1969"));
    }
}