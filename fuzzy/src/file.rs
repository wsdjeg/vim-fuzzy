#[allow(dead_code)]
pub fn fuzzy_match(files: Vec<String>, query: &str) -> Vec<String> {
    let mut rst: Vec<String> = Vec::new();
    let mut with_one_char: Vec<String> = Vec::new();
    for file in files {
        if match_file_head(file.clone(), query) {
            rst.push(file);
        } else if match_one_char(file.clone(), query) {
            with_one_char.push(file);
        }
    }
    rst
}

// #[cfg(not(windows))]
#[allow(dead_code, unused_variables)]
fn match_file_head(file: String, query: &str) -> bool {
    let mut fuck: Vec<&str> = file.split("\\").collect::<Vec<_>>();
    fuck.pop().unwrap().starts_with(query)
}

#[allow(dead_code, unused_variables)]
fn match_one_char(file: String, query: &str) -> bool {
    let mut fuck: Vec<&str> = file.split("\\").collect::<Vec<_>>();
    fuck.pop().unwrap().starts_with(query)
}

// #[cfg(windows)]
// fn match_file_head(file: String, query: &str) -> bool {
// false
// }
