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
    while low < all_words.len() && all_words[low].1 == n {
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
    for c in s.chars() {
        if 1 == n >> (c as u32 - 'a' as u32) & 1 {
            return 0;
        }
        n |= 1 << (c as u32 - 'a' as u32);
    }
    n
}

fn score_number(word: u32, letter_score: &[u32;26]) -> u32 {
    let mut score = 0;
    for i in 0..=26 {
        if 0 != (word >> i) & 1 {
            score += letter_score[i];
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

fn find_sols(depth: u32, me: u32, comparer: u32, words: &Vec<u32>, all_words: &Vec<(String, u32)>, skip_table: &Vec<u16>, history: &mut[u32;5], total_sols: &mut u32) {
    if depth == 1 {
        print!("\rstarting word {}/{}\r", me + 1, words.len());
        stdout().flush().unwrap();
    }
    if depth == 5 {
        *total_sols += 1;
        println!("   --- SOLUTION {} ---\n", total_sols);
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
        find_sols(depth + 1, number as u32, comparer | words[number], words, all_words, skip_table, history, total_sols);
    }
    
}
fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let lines = io::BufReader::new(File::open("./src/words.txt").unwrap()).lines();
    let mut words = Vec::new();
    let mut words_complete = Vec::new();
    let mut history = [0;5];
    let mut letter_freq_map = vec![(0,0);26];
    for i in 0..26 {
        letter_freq_map[i].1 = i;
    }
    for line in lines {
        let l = line.unwrap();
        if l.len() != 5 { continue };
        let n = str_to_num(l.clone());
        if n != 0 && words.contains(&n) == false {
            words.push(n);
            for i in 0..=25 {
                if (n >> i) & 1 == 1 {
                    letter_freq_map[i].0 += 1 as u32;
                }
            }
        }
        words_complete.push((l, n));
    }

    letter_freq_map.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut letter_score = [0;26];
    for i in 0..26 {
        letter_score[letter_freq_map[i].1] = (1 << i) as u32;
    }

    words.sort_by(|a, b| score_number(*a, &letter_score).partial_cmp(&score_number(*b, &letter_score)).unwrap());
    words_complete.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    
    let skip_table = gen_skip_table(words.clone());
    let mut total_solutions = 0;
    find_sols(0, 0, 0, &words, &words_complete, &skip_table, &mut history, &mut total_solutions);
    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
    println!("{} solutions were found", total_solutions);
}
