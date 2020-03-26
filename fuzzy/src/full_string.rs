use std::cmp::max;
#[allow(dead_code)]
pub fn fuzzy_match(_files: Vec<String>, _query: &str) -> Vec<String> {
    let mut rst: Vec<String> = Vec::new();
    let mut with_one_char: Vec<String> = Vec::new();
    rst.append(&mut with_one_char);
    rst
}

struct Words {
    distance: i32,
    word: String,
}

impl Words {
    fn new(word: String, query: String) -> Self {
        Words {
            distance: get_distance(word, query),
            word,
        }
    }
}

fn sort_word_by_distance(words: Vec<String>, query: String) -> Vec<String> {
    words
        .iter()
        .map(|x| Words::new(x.to_string(), query))
        .sort_by_key(|k| k.distance)
}

// ref: http://www.voidcn.com/article/p-xgnydwfz-bqy.html

// 编辑距离算法

fn get_distance(s1: String, s2: String) -> i32 {
    let chars1 = s1.chars().collect::<Vec<char>>();
    let chars2 = s2.chars().collect::<Vec<char>>();
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let mut d = vec![vec![0i32; len1 + 1]; len2 + 1];
    for i in 0..len1 {
        d[i][0] = i as i32;
    }
    for i in 0..len2 {
        d[0][i] = i as i32;
    }
    for i in 1..len1 + 1 {
        for j in 1..len2 + 1 {
            let mut cost = 1;
            if chars1[i - 1] == chars2[j - 1] {
                cost = 0;
            }
            let delete = d[i - 1][j] + 1;
            let insert = d[i][j - 1] + 1;
            let substitution = d[i - 1][j - 1] + cost;
            d[i][j] = min(delete, insert, substitution);
        }
    }
    d[len1][len2]
}
fn min(d: i32, i: i32, s: i32) -> i32 {
    let temp = if d > i { i } else { d };
    if s < temp {
        s
    } else {
        temp
    }
}

fn similarity(s1: String, s2: String) -> f32 {
    let long = max(s1.chars().count(), s2.chars().count()) as f32;
    let distance = get_distance(s1, s2) as f32;
    1 as f32 - distance / long
}

pub fn main() {
    println!(
        "{}",
        get_distance("wsdjeg".to_string(), "wdsjgh".to_string())
    );
    println!("{}", similarity("wsdjeg".to_string(), "wsdjeg".to_string()));
}
