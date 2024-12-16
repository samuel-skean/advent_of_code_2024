use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input_file = BufReader::new(File::open("input.txt").unwrap());
    let mut list1: Vec<u64> = Vec::new();
    // Map from numbers in the list to the number of their occurences:
    let mut list2_counter: HashMap<u64, u64> = HashMap::new();

    for (line_num, line) in input_file.lines().enumerate() {
        let line = line.expect(&format!("Error reading line: {line_num}"));
        let mut number_strings = line.split_whitespace();
        let list1_num = number_strings
            .next()
            .expect(&format!("No first element in line {line_num}"))
            .parse()
            .expect(&format!("Couldn't parse second number on line {line_num}"));
        let list2_num = number_strings
            .next()
            .expect(&format!("No second element in line {line_num}"))
            .parse()
            .expect(&format!("Couldn't parse second number on line {line_num}"));

        list1.push(list1_num);
        *list2_counter.entry(list2_num).or_insert(0) += 1;
    }

    let mut similarity_score = 0;
    for list1_num in list1 {
        similarity_score += list1_num * list2_counter.get(&list1_num).unwrap_or(&0);
    }
    println!("Similarity Score: {similarity_score}");
}
