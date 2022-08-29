use std::fs::File;
use std::io::{self, BufRead, stdout, Write};

fn num_to_str(n: u32, all_words: &Vec<(String, u32)>) -> String {
    let mut res = String::default();
    for bit in 0..=25 {
        if (n >> bit) & 1 == 1 {
            res.push(char::from_u32('A' as u32 + bit as u32).unwrap());
        } else {
            res.push('-');
        } 
    }
    let mut low = 0;
    let mut high = all_words.len() - 1;
    while low < high {
        let mid = low + (high - low) / 2;
        if all_words[mid].1 >= n {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    res.push_str("  ");
    let mut flag = false;
    while all_words[low].1 == n {
        if flag {
            res.push_str(" | ");
        }
        res.push_str(&(all_words[low].0.clone()));
        low += 1;
        flag = true;
    }
    res
}
fn str_to_num(s: String) -> u32 {
    let mut n = 0;
    for(_i, c) in s.chars().enumerate() {
        if 1 == n >> (c as u32 - 'a' as u32) & 1 {
            return 0;
        }
        n |= 1 << (c as u32 - 'a' as u32);
    }
    n
}

fn score_number(word: u32) -> u32 {
    let mut score = 0;
    let score_str = "etaoinshrdlucwmfygpbvkxjqz";
    let mut score_map = vec![0;26];
    for i in 0..26 {
        score_map[(score_str.chars().nth(i).unwrap() as u32 - 'a' as u32) as usize] = 1 << (26-i);
    }
    for i in 0..=26 {
        let n = (word >> i) & 1;
        if n != 0 {
            score += score_map[i];
        }
    }
    score
}

fn gen_skip_table(words: Vec<u32>) -> Vec<u16> {
    let mut res = Vec::new();
    for i in 0..words.len() {
        let mut skippable = 0;
        for j in (i + 1)..words.len() {
            if words[i] & words[j] == 0 {
                skippable = j;
                break;
            }
        }
        res.push((skippable - i) as u16);
    }
    res
}

fn find_sols(depth: u32, me: u32, comparer: u32, words: &Vec<u32>, all_words: &Vec<(String, u32)>, skip_table: &Vec<u16>, history: &mut[u32;5]) {
    if depth == 1 {
        print!("\rchecking starting word {}/{}\r", me + 1, words.len());
        stdout().flush().unwrap();
    }
    if depth == 5 {
        let mut combined = 0;
        for i in 0..5 {
            combined |= history[i];
            println!("{}", num_to_str(history[i], all_words));
        }
        println!("{}\n", num_to_str(combined, all_words));
        return;
    }
    for number in (me as usize + if depth != 0 { skip_table[me as usize] } else { 0 } as usize)..words.len() {
        if (comparer & words[number as usize]) != 0 {
            continue;
        }
        history[depth as usize] = words[number];
        find_sols(depth + 1, number as u32, comparer | words[number], words, all_words, skip_table, history);
    }
    
}
fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let lines = io::BufReader::new(File::open("./src/words.txt").unwrap()).lines();
    let mut words = Vec::new();
    let mut words_complete = Vec::new();
    let mut history = [0;5];
    for line in lines {
        let l = line.unwrap();
        let n = str_to_num(l.clone());
        if n != 0 && words.contains(&n) == false {
            words.push(n);
        }
        words_complete.push((l, n));
    }
    words.sort_by(|a, b| score_number(*a).partial_cmp(&score_number(*b)).unwrap());
    words_complete.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let skip_table = gen_skip_table(words.clone());
    find_sols(0, 0, 0, &words, &words_complete, &skip_table, &mut history);
    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}
