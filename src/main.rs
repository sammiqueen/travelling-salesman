use core::panic;
use std::time::Instant;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct City {
    position_x: i32,
    position_y: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TreeSpecifics {
    possible_destinations: [Vec<i32>; AMOUNT as usize],
    current_level: usize,
}

#[derive(Debug, Clone)]
pub struct Path {
    length: f64,
    route: Vec<i32>,
}

mod algorithms;
mod city_gen;

const AMOUNT: i32 = 5;
const ITERATIONS: i32 = 1;

fn main() {
    let mut tot_naive_time: u128 = 0;
    let mut tot_rnn_time: u128 = 0;
    let mut tot_difference: f64 = 0.0;

    let cities_distances: [[f64; AMOUNT as usize]; AMOUNT as usize] = {
        [
            [
                0.0,
                3.0,
                5.0990195135927845,
                1.4142135623730951,
                4.242640687119285,
            ],
            [
                3.0,
                0.0,
                4.58257569495584,
                2.6457513110645907,
                4.58257569495584,
            ],
            [
                5.0990195135927845,
                4.58257569495584,
                0.0,
                4.898979485566356,
                4.242640687119285,
            ],
            [
                1.4142135623730951,
                2.6457513110645907,
                4.898979485566356,
                0.0,
                4.47213595499958,
            ],
            [
                4.242640687119285,
                4.58257569495584,
                4.242640687119285,
                4.47213595499958,
                0.0,
            ],
        ]
    };

    for _i in 0..ITERATIONS {
        //let cities_distances: [[f64; AMOUNT as usize]; AMOUNT as usize] = city_gen::city_gen::cities_spawn();
        //println!("Spawning {} cities took {:?}", AMOUNT, now.elapsed());
        println!("Distances included:  \n{:#?}", cities_distances);

        let now = Instant::now();
        let path_naive = algorithms::algorithms::naive(&cities_distances);
        println!("Solving Naïve solution took {:?}", now.elapsed());
        println!("Naïve solution: {:#?}", path_naive);
        let naive_time = now.elapsed().as_nanos();
        tot_naive_time = tot_naive_time + naive_time;
        //println!("Solving Naïve solution took {:?}", naive_time);
        //println!("Naïve solution: {:#?}", path_naive);

        let now = Instant::now();
        let path_rnn = algorithms::algorithms::repetitive_nearest_neighbour(&cities_distances);
        println!("Solving RNN solution took {:?}", now.elapsed());
        println!("RNN solution: {:#?}", path_rnn);
        let rnn_time = now.elapsed().as_nanos();
        tot_rnn_time = tot_rnn_time + rnn_time;
        //println!("Solving RNN solution took {:?}", rnn_time);
        //println!("RNN solution: {:#?}", path_rnn);

        if rnn_time < naive_time {
            panic!()
        }

        let difference_naive_rnn = path_rnn.length - path_naive.length;
        tot_difference = tot_difference.clone() + difference_naive_rnn;
    }

    let avg_naive_time = tot_naive_time / AMOUNT as u128;
    let avg_rnn_time = tot_rnn_time / AMOUNT as u128;
    let avg_difference = tot_difference / AMOUNT as f64;

    println!(
        "AVG Naïve time: {} \n AVG RNN time: {}",
        avg_naive_time, avg_rnn_time
    );
    println!("AVG rnn.length - naive.length: {}", avg_difference);

    //christofide_serdyukov(&cities_distances);
}

/*fn christofide_serdyukov(cities_distances : &[[f64; 10];10]) {

}*/
