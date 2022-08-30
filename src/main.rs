use std::fs::File;
use std::io::{self, BufRead};
use std::thread;

extern crate num_cpus;

const WORD_LENGTH: usize = 5;

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

fn find_sols(depth: u32, me: u32, comparer: u32, words: &Vec<u32>, skip_table: &Vec<u16>, history: &mut [u32;WORD_LENGTH], solutions: &mut Vec<[u32;WORD_LENGTH]>) {
    if depth == WORD_LENGTH as u32 {
        solutions.push(*history);
        return;
    }
    for number in (me as usize + if depth != 0 { skip_table[me as usize] } else { 0 } as usize)..words.len() {
        if (comparer & words[number]) != 0 {
            continue;
        }
        history[depth as usize] = words[number];
        find_sols(depth + 1, number as u32, comparer | words[number], words, skip_table, history, solutions);
    }
    
}
fn main() {
    if WORD_LENGTH > 5 {
        println!("No solutions found ):");
    }
    use std::time::Instant;
    let now = Instant::now();
    let lines = io::BufReader::new(File::open("./src/words.txt").unwrap()).lines();
    let mut words = Vec::new();
    let mut words_complete = Vec::new();
    let mut letter_freq_map = vec![(0,0);26];
    let mut cpu_to_use = (num_cpus::get() - 1).max(1);

    for arg in std::env::args().skip(1) {
        let pair = arg.split("=").collect::<Vec<&str>>();
        let key = pair[0];
        let val = pair[1.min(pair.len() - 1)];

        match key {
            "numthreads" => {
                cpu_to_use = val.parse::<usize>().unwrap_or(cpu_to_use);
            }
            _ => {
            }
        }
    }
    for i in 0..26 {
        letter_freq_map[i].1 = i;
    }
    for line in lines {
        let l = line.unwrap();
        if l.len() != WORD_LENGTH { continue };
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

    println!("Computing solutions...\n");

    let mut handles = Vec::new();
    for thread_id in 0..cpu_to_use {
        let thread_words = words.clone();
        let thread_skip_table = skip_table.clone();
        let handle = thread::spawn(move || {
            let mut history = [0;WORD_LENGTH];
            let mut solutions = Vec::new();
            for num in 0..thread_words.len() {
                if num % cpu_to_use == thread_id {
                    history[0] = thread_words[num];
                    find_sols(1, num as u32, thread_words[num], &thread_words, &thread_skip_table, &mut history, &mut solutions);
                }
            }
            solutions
        });
        handles.push(handle);
    }
    let mut all_solutions = Vec::new();
    for h in handles {
        all_solutions.append(&mut h.join().unwrap());
    }
    for (i, solution) in all_solutions.clone().into_iter().enumerate() {
        println!("   --- SOLUTION {} ---\n", i + 1);
        let mut combined = 0;
        for i in 0..WORD_LENGTH {
            combined |= solution[i];
            println!("{}", num_to_str(solution[i], &words_complete));
        }
        println!("{}\n", num_to_str(combined, &words_complete));
    }
    if all_solutions.len() == 0 {
        println!("No solutions found ):");
    }
    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}
