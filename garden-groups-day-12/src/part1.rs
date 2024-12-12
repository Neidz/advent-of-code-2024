use std::collections::HashSet;

use crate::input::INPUT;

type Coordinates = (i32, i32);

const SIDE_OFFSETS: [Coordinates; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[derive(Debug, Clone)]
struct Region {
    plant_coordinates: HashSet<Coordinates>,
    plant: char,
}

impl Region {
    pub fn new(plant_coords: Coordinates, plant: char) -> Self {
        let mut plant_coordinates = HashSet::new();
        plant_coordinates.insert(plant_coords);

        Region {
            plant_coordinates,
            plant,
        }
    }

    pub fn can_merge(&self, other_region: &Region) -> bool {
        if self.plant != other_region.plant {
            return false;
        }

        for (x, y) in self.plant_coordinates.iter() {
            for (x_offset, y_offset) in SIDE_OFFSETS {
                let (x_with_offset, y_with_offset) = (x + x_offset, y + y_offset);

                if other_region
                    .plant_coordinates
                    .contains(&(x_with_offset, y_with_offset))
                {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn merge(&mut self, other_region_coords: &mut HashSet<Coordinates>) {
        self.plant_coordinates.extend(other_region_coords.drain());
    }

    pub fn belongs_to_region(&self, coords: Coordinates, plant: char) -> bool {
        if self.plant != plant {
            return false;
        }

        for (x_offset, y_offset) in SIDE_OFFSETS {
            let (x, y) = coords;

            let (x_with_offset, y_with_offset) = (x + x_offset, y + y_offset);

            if self
                .plant_coordinates
                .contains(&(x_with_offset, y_with_offset))
            {
                return true;
            }
        }

        return false;
    }

    pub fn area(&self) -> usize {
        self.plant_coordinates.len()
    }

    pub fn perimeter(&self) -> usize {
        let mut fences = 0;

        for (x, y) in self.plant_coordinates.iter() {
            for (x_offset, y_offset) in SIDE_OFFSETS {
                let (x_with_offset, y_with_offset) = (x + x_offset, y + y_offset);

                if !self
                    .plant_coordinates
                    .contains(&(x_with_offset, y_with_offset))
                {
                    fences += 1;
                }
            }
        }

        fences
    }

    pub fn add_plant(&mut self, coords: Coordinates) {
        self.plant_coordinates.insert(coords);
    }
}

pub fn part1() {
    let mut regions: Vec<Region> = vec![];

    for (y, line) in INPUT.lines().enumerate() {
        for (x, plant) in line.chars().enumerate() {
            let x: i32 = x.try_into().unwrap();
            let y: i32 = y.try_into().unwrap();

            let existing_region = regions
                .iter_mut()
                .find(|region| region.belongs_to_region((x, y), plant));

            if let Some(region) = existing_region {
                region.add_plant((x, y));
            } else {
                let new_region = Region::new((x, y), plant);

                regions.push(new_region);
            }
        }
    }

    let mut first_region_index = 0;

    while first_region_index < regions.len() {
        let mut first_region = regions.remove(first_region_index);

        let other_region = regions
            .iter_mut()
            .find(|region| region.can_merge(&first_region));

        if let Some(other_region) = other_region {
            other_region.merge(&mut first_region.plant_coordinates);
        } else {
            regions.insert(first_region_index, first_region);
            first_region_index += 1;
        }
    }

    let total_fencing_price: usize = regions
        .iter()
        .map(|region| region.area() * region.perimeter())
        .sum();

    println!("Part 1: total fencing price is {}", total_fencing_price);
}
