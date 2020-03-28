use std::cmp::max;
#[allow(dead_code)]
pub fn fuzzy_match(files: Vec<String>, query: &str) -> Vec<String> {
    sort_word_by_distance(files, query.to_string())
}

struct Words {
    distance: i32,
    word: String,
}

impl Words {
    fn new(word: String, query: String) -> Self {
        Words {
            distance: get_distance(word.clone(), query.clone()),
            word: word.clone(),
        }
    }
}

fn sort_word_by_distance(words: Vec<String>, query: String) -> Vec<String> {
    let mut w: Vec<Words> = words
        .iter()
        .map(|x| Words::new(x.to_string(), query.clone())).collect::<Vec<_>>();
    w.sort_by(|a, b| a.distance.cmp(&b.distance));
    w.iter().map(|x| x.word.clone() ).collect::<Vec<String>>()
}

// ref: http://www.voidcn.com/article/p-xgnydwfz-bqy.html

// 编辑距离算法
// https://www.dreamxu.com/books/dsa/dp/edit-distance.html

fn get_distance(s1: String, s2: String) -> i32 {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    // 当 len 为 0 时：
    if len1 == 0 {
        return len2 as i32;
    }else if len2 == 0{
        return len1 as i32;
    }
    let chars1 = s1.chars().collect::<Vec<char>>();
    let chars2 = s2.chars().collect::<Vec<char>>();
    let mut d = vec![vec![0i32; len2 + 1]; len1 + 1];
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

#[allow(dead_code)]
fn similarity(s1: String, s2: String) -> f32 {
    let long = max(s1.chars().count(), s2.chars().count()) as f32;
    let distance = get_distance(s1, s2) as f32;
    1 as f32 - distance / long
}

#[allow(dead_code)]
pub fn main() {
    println!("{}", get_distance("he".to_string(), "hello".to_string()));
}
