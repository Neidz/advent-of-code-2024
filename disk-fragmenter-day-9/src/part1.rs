use crate::input::INPUT;

#[derive(Debug, Clone, Copy)]
enum DiskItem {
    Num(usize),
    Free,
}

pub fn part1() {
    let disk_map: Vec<char> = INPUT.chars().collect();
    let mut expanded_disk_map: Vec<DiskItem> = Vec::with_capacity(disk_map.len());

    for (id, chunk) in disk_map.chunks(2).enumerate() {
        let (file, free_space) = if chunk.len() == 2 {
            (
                chunk[0].to_digit(10).unwrap(),
                Some(chunk[1].to_digit(10).unwrap()),
            )
        } else {
            (chunk[0].to_digit(10).unwrap(), None)
        };

        for _ in 0..file {
            expanded_disk_map.push(DiskItem::Num(id));
        }

        if let Some(free_space) = free_space {
            for _ in 0..free_space {
                expanded_disk_map.push(DiskItem::Free);
            }
        }
    }

    let expanded_disk_map_len = expanded_disk_map.len();

    loop {
        let first_free_space_index = expanded_disk_map
            .iter()
            .position(|item| matches!(item, DiskItem::Free))
            .unwrap();

        let last_num_index = expanded_disk_map_len
            - 1
            - expanded_disk_map
                .iter()
                .rev()
                .position(|item| matches!(item, DiskItem::Num(_)))
                .unwrap();

        if first_free_space_index > last_num_index {
            break;
        }

        let last_num = expanded_disk_map[last_num_index];

        expanded_disk_map[first_free_space_index] = last_num;
        expanded_disk_map[last_num_index] = DiskItem::Free;
    }

    let mut checksum = 0usize;

    for (index, disk_item) in expanded_disk_map.iter().enumerate() {
        match disk_item {
            DiskItem::Free => break,
            DiskItem::Num(val) => checksum += val * index,
        }
    }

    println!("Part 1: checksum is {}", checksum);
}
