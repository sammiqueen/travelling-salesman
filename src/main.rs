use rand::Rng;

use ndarray::Array;
use ndarray::Ix2;

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
struct City {
    position_x: i32,
    position_y: i32,
}

fn main() {
    let cities_distances: Array::<f64, Ix2> = cities_spawn();

    naive(&cities_distances);

    //repetitive_nearest_neighbour(&cities_distances);

    //christofide_serdyukov(&cities_distances);
}

fn cities_spawn() -> Array::<f64, Ix2> {

    let mut rng = rand::rng();

    let cities : [City; 10] = {
        let mut temp = [City {
            position_x: 0,
            position_y: 0
        }; 10];

        for i in 0..10 {

            let mut is_duplicate = false;

            for y in 0..10 {
                match temp[i] {
                     val if val == temp[y] => is_duplicate = true,
                    City {
                        position_x: 0,
                        position_y: 0
                    } => is_duplicate = true,
                    _ => is_duplicate = false
                }
            }
            
            if is_duplicate {
                temp[i] = City {
                    position_x: rng.random_range(1..100),
                    position_y: rng.random_range(1..100)
                };
            }
        }
        temp
    };

    println!("{:?}", cities);

    let cities_distances : Array::<f64, Ix2> = {
        let mut temp : Array::<f64, Ix2> = Array::<f64, Ix2>::zeros((10, 10));

        for i in 0..10 {
            for y in 0..10 {
                temp[[i, y]] = distance(cities[i], cities[y]);
            }
        }

        temp
    };
     
    cities_distances
}

fn distance(city_a : City, city_b : City) -> f64 {
    let x_dif : f64 = (city_a.position_x - city_b.position_x).abs().into();
    let y_dif : f64 = (city_a.position_y - city_b.position_y).abs().into();

    let distance : f64 = (x_dif + y_dif).sqrt();

    distance
}


fn naive(cities_distances : &Array::<f64, Ix2>) -> f64 {
    println!("Hello from naive solution");

    let mut route_length : f64 = 0;

    route_length
}

/*fn repetitive_nearest_neighbour(cities_distances : &Array::<f64, Ix2>) {
    let mut shortest_distance : i32 = 10;

    for city in cities_distances {
        println!("{:?} c", city);
        
        println!("{:?} s", shortest_distance);

        if *city < shortest_distance {
            println!("{} is shorter than {}", city, shortest_distance);
            shortest_distance = *city;
        }
    }
    
}*/

/*fn christofide_serdyukov(cities_distances : &Array::<f64, Ix2>) {

}*/