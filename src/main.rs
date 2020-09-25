use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    ex1(vec![1, 2, 3, 4]);
}

/*
Given a vector of ints return the mean, median and mode
*/
fn ex1(vector: &Vec<i32>) -> HashMap<String, f64> {
    let mut results = HashMap::new();

    results.insert(String::from("mean"), mean(&vector));
    results.insert(String::from("median"), median(&vector));
    results.insert(String::from("mode"), mode(&vector));
    results
}

fn mean(vector: &Vec<i32>) -> f64 {
    let vector_sum = vector.iter().sum::<i32>();
    f64::from(vector_sum) / (vector.len() as f64)
}

fn median(vector: &Vec<i32>) -> f64 {
    let mut vec_copy: Vec<i32> = vector.to_vec();
    vec_copy.sort();
    let med = vec_copy.len() / 2;
    if vec_copy.len() % 2 == 0 {
        return ((&vec_copy[med] + &vec_copy[med - 1]) as f64) / (2 as f64);
    } else {
        return *(&vec_copy[med]) as f64;
    }
}

fn mode(vector: &Vec<i32>) -> f64 {
    let mut count_map = HashMap::new();

    for val in vector {
        let element = count_map.entry(val).or_insert(0);
        *element += 1;
    }
    let max_value = count_map.values().max();
    if let None = max_value {
        panic!("Error !!");
    }

    for (num, val) in &count_map {
        if max_value == Some(val) {
            return **num as f64;
        }
    }
    panic!("No match !!");
}

fn ex2(word: &String) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let first = word.chars().nth(0).unwrap();

    if vowels.contains(&first) {
        return format!("{}-{}", word, "hay");
    } else {
        let mut copy = String::from(word);
        copy.remove(0);
        return format!("{}-{}{}", copy, first, "ay");
    }
}

fn ex3(sentence: &String) -> HashMap<String, Vec<String>> {
    let mut list: HashMap<String, Vec<String>> = HashMap::new();
    let iter = sentence.split_whitespace();

    let mut name: String = String::new();
    let mut department: String = String::new();

    for (i, word) in iter.enumerate() {
        if i == 0 && word != "Add" {
            panic!("Do not support the command");
        } else if i == 1 {
            name = String::from(word);
        } else if i == 3 {
            department = String::from(word);
        }
    }
    if let Some(department_people) = list.get_mut(&department) {
        department_people.push(name);
    }
    list
}
