use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input_file = BufReader::new(File::open("input.txt").unwrap());
    let mut heap1: BinaryHeap<Reverse<u64>> = BinaryHeap::new();
    let mut heap2: BinaryHeap<Reverse<u64>> = BinaryHeap::new();

    for (line_num, line) in input_file.lines().enumerate() {
        let line = line.expect(&format!("Error reading line: {line_num}"));
        let mut number_strings = line.split_whitespace();
        heap1.push(Reverse(
            number_strings
                .next()
                .expect(&format!("No first element in line {line_num}"))
                .parse()
                .expect(&format!("Couldn't parse second number on line {line_num}")),
        ));
        heap2.push(Reverse(
            number_strings
                .next()
                .expect(&format!("No second element in line {line_num}"))
                .parse()
                .expect(&format!("Couldn't parse second number on line {line_num}")),
        ));
    }

    let mut sum_of_differences = 0;
    for (Reverse(n1), Reverse(n2)) in heap1.into_sorted_vec().into_iter().zip(heap2.into_sorted_vec().into_iter()) {
        sum_of_differences += n1.abs_diff(n2);
    }
    println!("Sum of differences was: {sum_of_differences}");
}
