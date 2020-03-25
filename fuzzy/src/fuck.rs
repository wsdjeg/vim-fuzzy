pub fn main() {
    println!("{}", "abc".to_string().chars().collect::<Vec<char>>().get(1).unwrap());
}
