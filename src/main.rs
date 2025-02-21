use rand::Rng;

use ndarray::Array;
use ndarray::Ix2;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

fn main() {
    let cities_distances: Array::<i32, Ix2> = cities_spawn();

    naive(&cities_distances);

    repetitive_nearest_neighbour(&cities_distances);

    //christofide_serdyukov(&cities_distances);
}

fn cities_spawn() -> Array::<i32, Ix2> {
    let cities_distances = Array::<i32, Ix2>::random((10, 10), Uniform::new(1, 10));
    
    struct City {
        position_x: i32,
        position_y: i32,
    }

    let rng = rand::rng();

    let cities : Vec<City> = Vec::new();

    for i in 0..10 {
        cities.push(City {
            position_x: rng.random(),
            position_y: rng.random(),
        }
    );
        println!("{:?}", cities[i]);mu
    }

    //println!("{}", cities_distances);

    return cities_distances
}


fn naive(cities_distances : &Array::<i32, Ix2>) {

    println!("Hello from naive solution");
}

fn repetitive_nearest_neighbour(cities_distances : &Array::<i32, Ix2>) {
    let mut shortest_distance : i32 = 10;

    for city in cities_distances {
        println!("{:?} c", city);
        
        println!("{:?} s", shortest_distance);

        if *city < shortest_distance {
            println!("{} is shorter than {}", city, shortest_distance);
            shortest_distance = *city;
        }
    }
    
}

/*fn christofide_serdyukov(cities_distances : &Array::<i32, Ix2>) {

}*/