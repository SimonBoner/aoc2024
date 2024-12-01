use std::fs;
use std::num;

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    println!("Matrix:");
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            print!("{} ", matrix[i][j]);
        }
        println!();
    }
    println!("-----------------");
}

fn transpose(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut transposed = vec![vec![0; matrix.len()]; matrix[0].len()];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            transposed[j][i] = matrix[i][j];
        }
    }
    transposed
}

fn sort_columns(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // print_matrix(&matrix);
    let mut transposed = transpose(&matrix);
    // print_matrix(&transposed);
    for row in &mut transposed {
        row.sort();
    }
    transpose(&transposed)
}

fn count_occurrences<T: PartialEq>(vec: &Vec<T>, item: T) -> usize {
    vec.iter().filter(|x| **x == item).count()
}

fn main() {
    let filepath = "input.txt";

    let raw_input = fs::read_to_string(filepath);
    
    let parsed_input = raw_input
        .unwrap()
        .split("\n")
        .map(|line| 
            line.split_ascii_whitespace()
            .map(|num| num.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        ).collect::<Vec<Vec<i32>>>();
        

    let sorted_matrix = sort_columns(&parsed_input);

    let sum_of_diff_per_row = sorted_matrix.iter().fold(0, |acc, row| {
        acc + (row[0] - row[1]).abs()
    });

    println!("Result: {}", sum_of_diff_per_row);

    let transposed_input = transpose(&parsed_input);

    let row1 = transposed_input.get(0).unwrap();
    let row2 = transposed_input.get(1).unwrap();
    
    let sum_of_similarty_scores = row1.iter().fold(0, |acc, &cell| {
        let similarity_score = count_occurrences(row2, cell);

        acc + (cell * similarity_score as i32)
    });

    println!("Result: {}", sum_of_similarty_scores);
}
