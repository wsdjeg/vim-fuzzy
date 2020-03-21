pub fn fuzzy_match(files: Vec<String>, query: &str) -> Vec<String> {
    let mut rst: Vec<String> = Vec::new();
    for file in files {
        if file.contains(query) {
            rst.push(file);
        }
    }
    rst
}
