use rayon::prelude::*;
use std::thread;

const CHANNEL_CAPACITY: usize = 10;

const MATRIX_SIZE: usize = 4096;
type Element = u8;
type Matrix = Vec<Vec<Element>>;

fn generate_matrix() -> Matrix {
    (0..MATRIX_SIZE)
        .map(|_| (0..MATRIX_SIZE).map(|_| rand::random()).collect())
        .collect()
}

type Sum = u64;
fn calc_matrix_sum(matrix: &Matrix) -> Sum {
    matrix.par_iter().flatten().map(|elem| *elem as Sum).sum()
}

fn main() {
    let (producer, consumer) = crossbeam_channel::bounded(CHANNEL_CAPACITY);

    thread::scope(|scope| {
        scope.spawn(|| loop {
            let matrix = generate_matrix();
            if producer.send(matrix).is_err() {
                break;
            }
        });

        for _ in 0..2 {
            scope.spawn(|| loop {
                let matrix = consumer.recv();
                match matrix {
                    Ok(matrix) => {
                        let sum = calc_matrix_sum(&matrix);
                        println!("sum: {}", sum);
                    }
                    Err(_) => break,
                }
            });
        }
    });
}
