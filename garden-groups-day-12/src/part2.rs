use std::collections::HashSet;

use crate::input::INPUT;

type Coordinates = (i32, i32);

const SIDE_OFFSETS: [Coordinates; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[derive(Debug, Clone)]
struct Region {
    plant_coordinates: HashSet<Coordinates>,
    plant: char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

impl From<Coordinates> for Side {
    fn from(value: Coordinates) -> Self {
        match value {
            (0, -1) => Side::Top,
            (0, 1) => Side::Bottom,
            (-1, 0) => Side::Left,
            (1, 0) => Side::Right,
            _ => panic!("Wrong side coordinates"),
        }
    }
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

    pub fn sides(&self) -> usize {
        let mut fences: HashSet<(Coordinates, Side)> = HashSet::new();

        for (x, y) in self.plant_coordinates.iter() {
            for (x_offset, y_offset) in SIDE_OFFSETS {
                let side: Side = (x_offset, y_offset).into();
                let (x_with_offset, y_with_offset) = (x + x_offset, y + y_offset);

                if !self
                    .plant_coordinates
                    .contains(&(x_with_offset, y_with_offset))
                {
                    fences.insert(((x_with_offset, y_with_offset), side));
                }
            }
        }

        let mut already_counted: HashSet<(Coordinates, Side)> = HashSet::new();
        let mut sides = 0;

        fences.iter().for_each(|((x, y), side)| {
            let is_already_counted = already_counted.contains(&((*x, *y), *side));

            if is_already_counted {
                return;
            } else {
                already_counted.insert(((*x, *y), *side));
                sides += 1;
            }

            let move_offset = match side {
                Side::Top | Side::Bottom => (1, 0),
                Side::Left | Side::Right => (0, 1),
            };

            let mut neighbor_coords = (*x, *y);
            loop {
                neighbor_coords = (
                    neighbor_coords.0 + move_offset.0,
                    neighbor_coords.1 + move_offset.1,
                );

                let is_valid = fences.contains(&(neighbor_coords, *side));

                if !is_valid {
                    break;
                }

                already_counted.insert((neighbor_coords, *side));
            }

            neighbor_coords = (*x, *y);

            loop {
                neighbor_coords = (
                    neighbor_coords.0 - move_offset.0,
                    neighbor_coords.1 - move_offset.1,
                );

                let is_valid = fences.contains(&(neighbor_coords, *side));

                if !is_valid {
                    break;
                }

                already_counted.insert((neighbor_coords, *side));
            }
        });

        sides
    }

    pub fn add_plant(&mut self, coords: Coordinates) {
        self.plant_coordinates.insert(coords);
    }
}

pub fn part2() {
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
        .map(|region| region.area() * region.sides())
        .sum();

    println!("Part 2: total fencing price is {}", total_fencing_price);
}
