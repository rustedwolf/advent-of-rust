use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Distance {
    point_a: String,
    point_b: String,
    value: u32
}
type Distances = Vec<Distance>;

struct Route {
    places: Places,
    distance: u32
}
type Routes = Vec<Route>;

type Places = Vec<String>;

trait RouteCalculations {
    fn get_distances(&mut self) -> (u32, u32);
}

impl RouteCalculations for Routes {
    fn get_distances(&mut self) -> (u32, u32) {
        self.sort_by(|a, b| a.distance.cmp(&b.distance));
        (self.first().unwrap().distance, self.last().unwrap().distance)
    }
}

fn main() {
    println!("--- Day 9: All in a Single Night ---");

    let distances = get_distances();
    let places = get_places(&distances);
    let mut routes: Routes = Vec::new();

    for place in &places {
        let route = Route {
            places: vec![place.clone()],
            distance: 0
        };
        routes.push(route);
    }

    for _ in 1..places.len() {
        routes = generate_routes(&routes, &places, &distances);
    }

    let (shortest_route, longest_route) = routes.get_distances();

    println!("The distance of the shortest route is {} and the longest is {}", shortest_route, longest_route);
}

fn get_distances() -> Distances {
    let path = Path::new("input/day_9");
    let input = match File::open(&path) {
        Ok(file) => file,
        Err(reason) => panic!("Could not open input file: {}", reason)
    };

    let reader = BufReader::new(&input);
    let mut distances: Distances = Vec::new();

    for line in reader.lines() {
        let info_string = line.unwrap();
        let info: Vec<&str> = info_string.split(" = ").collect();
        let places: Vec<&str> = info[0].rsplit(" to ").collect();
        let distance = Distance {
            point_a: places[0].to_string(),
            point_b: places[1].to_string(),
            value: info[1].parse::<u32>().unwrap()
        };
        distances.push(distance);
    }

    distances
}

fn get_places(distances: &Distances) -> Places {
    let mut places: Places = Vec::new();

    for distance in distances {
        if !places.contains(&distance.point_a) {
            places.push(distance.point_a.clone())
        }
        if !places.contains(&distance.point_b) {
            places.push(distance.point_b.clone())
        }
    }

    places
}

fn generate_routes(routes: &Routes, places: &Places, distances: &Distances) -> Routes {
    let mut updated_routes: Routes = Vec::new();
    for route in routes {
        for place in places {
            if !route.places.contains(&place) {
                let mut route_places = route.places.clone();
                route_places.push(place.clone());
                let new_route = Route {
                    places: route_places,
                    distance: route.distance + get_distance(route.places.last().unwrap(), &place, &distances),                    
                };
                updated_routes.push(new_route);
            }
        }
    }

    updated_routes
}

fn get_distance(place_a : &str, place_b : &str, distances: &Distances) -> u32 {
    let mut distance_value = 0u32;
    for distance in distances {
        if (distance.point_a == place_a && distance.point_b == place_b) || 
        (distance.point_a == place_b && distance.point_b == place_a) {
            distance_value = distance.value;
            break;
        }
    }

    distance_value
}