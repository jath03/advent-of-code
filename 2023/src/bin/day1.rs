const NUMBERS: &'static [&'static str] = &[
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn num_map(s: &str) -> u32 {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => unreachable!()
    }
}

fn main() {
    let lines = aoc2023::read_lines("input/input1.txt").unwrap();
    let mut sum = 0;
    let mut first: Option<_> = None;
    let mut last = 0;

    for line in lines  {
        if let Ok(s) = line {
            for (i, c) in s.char_indices() {
                if c.is_digit(10) {
                    if first.is_none() {
                        first = Some(c.to_digit(10).unwrap());
                    }
                    last = c.to_digit(10).unwrap();
               }
               for num in NUMBERS {
                   if s.chars().skip(i).take(num.len()).collect::<String>() == *num {
                       if first.is_none() {
                           first = Some(num_map(num));
                       }
                       last = num_map(num);
                   }
               }
            }
        }
        sum += first.unwrap_or(0) * 10 + last;
        first = None;
    }
    println!("{sum}");
}
