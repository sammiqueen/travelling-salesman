use std::time::Instant;

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
struct City {
    position_x: i32,
    position_y: i32,
}

#[derive(Debug)]
#[derive(Clone)]
struct Path {
    length: f64,
    route: Vec<i32>
}

mod algorithms;
mod city_gen;

const AMOUNT: i32 = 8;
const ITERATIONS: i32 = 1000;

fn main() {
    let mut tot_naive_time: u128 = 0;
    let mut tot_rnn_time: u128 = 0;
    let mut tot_naive_length: f64 = 0.0;
    let mut tot_rnn_length: f64 = 0.0;
    let mut tot_percentual_increase: f64 = 0.0;

    for _i in 0..ITERATIONS {
        let cities_distances: [[f64; AMOUNT as usize]; AMOUNT as usize] =
            city_gen::city_gen::cities_spawn();
        //println!("Spawning {} cities took {:?}", AMOUNT, now.elapsed());
        //println!("Distances included:  \n{:#?}", cities_distances);

        let now = Instant::now();
        let path_naive = algorithms::algorithms::naive(&cities_distances);
        //println!("Solving Naïve solution took {:?}", now.elapsed());
        //println!("Naïve solution: {:?}", path_naive.route);
        //println!("Solving Naïve solution took {:?}", naive_time);
        let naive_time = now.elapsed().as_nanos();

        let now = Instant::now();
        let path_rnn = algorithms::algorithms::repetitive_nearest_neighbour(&cities_distances);
        //println!("Solving RNN solution took {:?}", now.elapsed());
        //println!("RNN solution: {:?}", path_rnn.route);
        //println!("Solving RNN solution took {:?}", rnn_time);
        let rnn_time = now.elapsed().as_nanos();

        tot_naive_time = tot_naive_time + naive_time;
        tot_rnn_time = tot_rnn_time + rnn_time;

        tot_rnn_length = tot_rnn_length.clone() + path_rnn.length;
        tot_naive_length = tot_naive_length.clone() + path_naive.length;

        let mut percentual_increase = (path_rnn.length - path_naive.length) / path_naive.length;
        if percentual_increase < 0.0 {
            percentual_increase = 0.0;
        }
        tot_percentual_increase = tot_percentual_increase.clone() + percentual_increase;
        //println!("percentual difference = {}", percentual_increase);
    }

    let avg_naive_time = tot_naive_time / ITERATIONS as u128;
    let avg_rnn_time = tot_rnn_time / ITERATIONS as u128;

    let avg_rnn_length = tot_rnn_length / ITERATIONS as f64;
    let avg_naive_length = tot_naive_length / ITERATIONS as f64;

    let avg_percentual_increase = tot_percentual_increase / ITERATIONS as f64;

    println!(
        "AVG Naïve time: {}ns \n AVG RNN time: {}ns",
        avg_naive_time, avg_rnn_time
    );
    println!(
        "AVG Naive path length: {} \n AVG RNN path length {}",
        avg_naive_length, avg_rnn_length
    );
    println!(
        "AVG percentual increase from naive length to rnn length: {}",
        avg_percentual_increase * 100.0
    );
}
