use ndarray::Array;
use ndarray::Ix2;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

fn main() {
    let _cities_distances: Array::<i32, Ix2> = cities_spawn();
}

fn cities_spawn() -> Array::<i32, Ix2> {
    let cities_distances = Array::<i32, Ix2>::random((10, 10), Uniform::new(1, 10));
    println!("{}", cities_distances);

    return cities_distances
}

/*
fn naive(cities_distances : Array(10, 10)) {
    
}

fn nearest_neighbour(cities_distances : Array(10, 10)) {

}

fn christofide_serdyukov(cities_distances : Array(10, 10)) {

}
*/