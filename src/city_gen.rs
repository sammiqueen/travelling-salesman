pub mod city_gen {
    use crate::AMOUNT;
    use rand::Rng;
    use crate::City;

    pub fn cities_spawn() -> [[f64; AMOUNT as usize]; AMOUNT as usize] {
        let mut rng = rand::rng();

        let cities: [City; AMOUNT as usize] = {
            let mut temp = [City {
                position_x: 0,
                position_y: 0,
            }; AMOUNT as usize];

            for i in 0..temp.len() {
                let mut is_duplicate = false;

                for y in 0..temp.len() {
                    match temp[i] {
                        val if val == temp[y] => is_duplicate = true,
                        City {
                            position_x: 0,
                            position_y: 0,
                        } => is_duplicate = true,
                        _ => is_duplicate = false,
                    }
                }

                if is_duplicate {
                    temp[i] = City {
                        position_x: rng.random_range(1..(AMOUNT * AMOUNT)),
                        position_y: rng.random_range(1..(AMOUNT * AMOUNT)),
                    };
                }
            }
            temp
        };

        let cities_distances: [[f64; AMOUNT as usize]; AMOUNT as usize] = {
            let mut temp: [[f64; AMOUNT as usize]; AMOUNT as usize] =
                [[0.0; AMOUNT as usize]; AMOUNT as usize];

            for i in 0..cities.len() {
                for y in 0..cities.len() {
                    temp[i][y] = distance(cities[i], cities[y]);
                }
            }

            temp
        };

        cities_distances
    }

    fn distance(city_a: City, city_b: City) -> f64 {
        let x_dif: f64 = (city_a.position_x - city_b.position_x).abs().into();
        let y_dif: f64 = (city_a.position_y - city_b.position_y).abs().into();

        let distance: f64 = (x_dif + y_dif).sqrt();

        distance
    }
}
