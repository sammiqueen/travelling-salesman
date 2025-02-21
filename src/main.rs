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
    println!("{}", cities_distances);

    return cities_distances
}


fn naive(cities_distances : &Array::<i32, Ix2>) {
    for city in cities_distances {
        println!("{:?}", city);
    }
}

fn repetitive_nearest_neighbour(cities_distances : &Array::<i32, Ix2>) {
    let mut shortest_distance : i32 = 10;

    for city in cities_distances {
        println!("{:?}", &city);
        
        println!("{}", &shortest_distance);

        if &city < &shortest_distance {
            println!("{} is shorter than {}", &city, &shortest_distance);
            shortest_distance = city;
        }
    }
    
}

/*fn christofide_serdyukov(cities_distances : &Array::<i32, Ix2>) {

}*/