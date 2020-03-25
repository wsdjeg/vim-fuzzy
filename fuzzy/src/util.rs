pub fn find_start_at(slice: &str, at: usize, pat: char) -> Option<usize> {
    slice[at..].find(pat).map(|i| at + i)
}
