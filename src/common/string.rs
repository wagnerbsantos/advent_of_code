pub fn split_to_vec(text_block: String, splitter: &str) -> Vec<String> {
    let lines = text_block.split(splitter);

    return lines.map(|line| line.to_string()).collect();
}
