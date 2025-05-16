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

const AMOUNT: i32 = 5;

fn main() {
        let now = Instant::now();
        let cities_distances: [[f64; AMOUNT as usize]; AMOUNT as usize] =
        city_gen::city_gen::cities_spawn();
        //println!("Spawning {} cities took {:?}", AMOUNT, now.elapsed());
        //println!("Distances included:  \n{:#?}", cities_distances);

        let now = Instant::now();
        let path_naive = algorithms::algorithms::naive(&cities_distances);
        println!("Solving Naïve solution took {:?}", now.elapsed());
        println!("Naïve solution: {:?}", path_naive.route);
        let naive_time = now.elapsed().as_nanos();
        tot_naive_time = tot_naive_time + naive_time;
        //println!("Solving Naïve solution took {:?}", naive_time);

    let now = Instant::now();
    let path_repetitive = algorithms::algorithms::repetitive_nearest_neighbour(&cities_distances);
    println!("Solving RNN solution took {:?}", now.elapsed());
    println!("RNN solution: {:#?}", path_repetitive);

    //christofide_serdyukov(&cities_distances);
}

/*fn christofide_serdyukov(cities_distances : &[[f64; 10];10]) {

}*/
