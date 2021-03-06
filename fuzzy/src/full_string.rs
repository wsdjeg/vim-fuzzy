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
            distance: fuzzy_distance(word.clone(), query.clone()),
            word: word.clone(),
        }
    }
}

fn sort_word_by_distance(words: Vec<String>, query: String) -> Vec<String> {
    let mut w: Vec<Words> = words
        .iter()
        .map(|x| Words::new(x.to_string(), query.clone()))
        .collect::<Vec<_>>();
    w.sort_by(|a, b| a.distance.cmp(&b.distance));
    w.iter().map(|x| x.word.clone()).collect::<Vec<String>>()
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
    } else if len2 == 0 {
        return len1 as i32;
    }
    let chars1 = s1.chars().collect::<Vec<char>>();
    let chars2 = s2.chars().collect::<Vec<char>>();
    let mut d = vec![vec![0i32; len2 + 1]; len1 + 1];
    for i in 0..=len1 {
        d[i][0] = i as i32;
    }
    for i in 0..=len2 {
        d[0][i] = i as i32;
    }
    for i in 1..=len1 {
        for j in 1..=len2 {
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
    // println!("{}", get_distance("ec".to_string(), "he".to_string()));
    // let mut words: Vec<String> = Vec::new();
    // words.push("heslo".to_string());
    // words.push("helab".to_string());
    // words.push("helac".to_string());
    // words.push("hel".to_string());
    // println!("{:?}", fuzzy_match(words, "hel"));
    println!("{}", fuzzy_distance("abcdefg".to_string(), "ce".to_string()));
}

fn fuzzy_distance(s1: String, s2: String) -> i32 {
    let mut begin: i32 = 0;
    #[allow(unused_assignments)]
    let mut end: i32 = 0;
    let mut long: i32 = 0;
    let chars1 = s1.chars().collect::<Vec<char>>();
    let chars2 = s2.chars().collect::<Vec<char>>();
    let char1_len = chars1.len() as i32;
    let char2_len = chars2.len() as i32;
    let mut idx = 0;
    for c in chars1 {
        begin += 1;
        if idx == chars2.len() {
            break;
        } else if c == chars2[idx] {
            idx += 1;
        }else{
            long += 1;
        }
    }
    end = char1_len - begin;
    begin = begin - char2_len - 1;
    long = long + char2_len;
    // println!("{:?}", (begin, end, long));
    (begin + end) / 2 + long
}
