use crate::util::find_start_at;

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
    rst.append(&mut with_one_char);
    rst
}

// #[cfg(not(windows))]
#[allow(dead_code, unused_variables)]
fn match_file_head(file: String, query: &str) -> bool {
    file.split("\\").last().unwrap().starts_with(query)
}

#[allow(dead_code, unused_variables)]
fn match_one_char(file: String, query: &str) -> bool {
    let f_name = file.split("\\").last().unwrap();
    let mut m = true;
    // let f_name = fuck.pop().unwrap();
    let mut idx = 0;
    for c in query.chars() {

         let o = find_start_at(f_name, idx, c);
         m = o.is_some();
         if m {
             idx = o.unwrap();
         }else{
             break
         }
    }
    m
}



// #[cfg(windows)]
// fn match_file_head(file: String, query: &str) -> bool {
// false
// }
