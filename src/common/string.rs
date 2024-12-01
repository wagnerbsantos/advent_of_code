pub fn split_to_vec(text_block: String, splitter: &str) -> Vec<String> {
    let lines = text_block.split(splitter);

    lines.map(|line| line.to_string()).collect()
}

pub fn get_number(chars: &[char], start: usize, end: usize) -> u32 {
    let mut sum = 0;
    for pos in start..end {
        let multi = end - pos -1;
        sum += (chars[pos] as u32 -48) * (10_u32).pow(multi as u32);
    }
    sum
}