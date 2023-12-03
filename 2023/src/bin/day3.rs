use std::cmp::min;

fn is_symbol(chars: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if let Some(row) = chars.get(y) {
        if let Some(char) = row.get(x) {
            return !char.is_numeric() && *char != '.';
        }
    }
    return false;
}

fn extract_number(chars: &Vec<Vec<char>>, start_cache: &mut Vec<usize>, x: usize, y: usize) -> anyhow::Result<u64> {
    let mut start = x;
    let mut num_len = 1;
    while start > 0 && chars[y][start - 1].is_digit(10) {
        start -= 1;
    }
    if start_cache.contains(&(start + 1000 * y)) {
        return Ok(0);
    } else {
        start_cache.push(start + 1000 * y);
    }
    while start + num_len < chars[0].len() && chars[y][start + num_len].is_digit(10) {
        num_len += 1;
    }
    // println!("Found number {} {}", chars[y][start..start+num_len].iter().collect::<String>(), chars[y][x]);


    Ok(chars[y][start..start+num_len].iter().collect::<String>().parse::<u64>()?)
}

fn main() -> anyhow::Result<()>{
    let lines = aoc2023::read_lines("input/input3.txt").unwrap();
    let chars: Vec<Vec<char>> = lines.map(|s| s.unwrap().chars().collect()).collect();
    let mut sum = 0;

    for i in 0..chars.len() {
        let mut j = 0;
        while j < chars[0].len() {
            if chars[i][j].is_digit(10) {
                let mut num_len = 1;
                while j + num_len < chars[0].len() && chars[i][j + num_len].is_digit(10) {
                    num_len += 1;
                }
                // println!("Found number {}", chars[i][j..j+num_len].iter().collect::<String>());
                if is_symbol(&chars, j.saturating_sub(1), i) || is_symbol(&chars, j + num_len, i) {
                    sum += chars[i][j..j+num_len].iter().collect::<String>().parse::<u32>()?;
                    // println!("Matched");
                } else {
                    for x in j.saturating_sub(1)..j+num_len+1 {
                        if is_symbol(&chars, x, i.saturating_sub(1)) || is_symbol(&chars, x, i + 1) {
                            sum += chars[i][j..j+num_len].iter().collect::<String>().parse::<u32>()?;
                            // println!("Matched");
                            break;
                        }
                    }
                }
                j += num_len - 1;
            }
            j += 1;
        }
    }
    println!("Sum of part numbers: {sum}");

    let mut ratios: u64 = 0;

    for i in 0..chars.len() {
        for j in 0..chars[0].len() {
            let mut adjacent = 0;
            let mut ratio = 1;
            let mut start_cache = vec![];
            if chars[i][j] == '*' {
                if chars[i][j.saturating_sub(1)].is_digit(10) {
                    let n = extract_number(&chars, &mut start_cache, j.saturating_sub(1), i)?;
                    if n != 0 {
                        adjacent += 1;
                        ratio *= n;
                    }
                }
                if j + 1 < chars[0].len() && chars[i][j + 1].is_digit(10) {
                    let n = extract_number(&chars, &mut start_cache, j + 1, i)?;
                    if n != 0 {
                        adjacent += 1;
                        ratio *= n;
                    }
                }
                for k in j.saturating_sub(1)..j+2 {
                    if k >= chars[0].len() {
                        break;
                    }
                    if chars[i.saturating_sub(1)][k].is_digit(10) {
                        let n = extract_number(&chars, &mut start_cache, k, i.saturating_sub(1))?;
                        if n != 0 {
                            adjacent += 1;
                            ratio *= n;
                        }
                    }
                    if i + 1 < chars.len() && chars[i + 1][k].is_digit(10) {
                        let n = extract_number(&chars, &mut start_cache, k, i + 1)?;
                        if n != 0 {
                            adjacent += 1;
                            ratio *= n;
                        }
                    }
                }
                if adjacent == 2 {
                    // println!("Adding {ratio} for {j}x{i}");
                    ratios += ratio;
                }
            }
        }
    }
    println!("Gear ratios: {ratios}");
    Ok(())
}
