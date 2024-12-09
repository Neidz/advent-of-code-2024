use crate::input::INPUT;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileChunk {
    size: u32,
    id: usize,
    attempted_to_fragment: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Chunk {
    File(FileChunk),
    Free(u32),
}

#[derive(Debug, Clone, Copy)]
enum DiskItem {
    Num(usize),
    Free,
}

pub fn part2() {
    let raw_disk_map: Vec<char> = INPUT.chars().collect();
    let mut disk_map: Vec<Chunk> = Vec::with_capacity(raw_disk_map.len());

    for (id, chunk) in raw_disk_map.chunks(2).enumerate() {
        let (file, free_space) = if chunk.len() == 2 {
            (
                chunk[0].to_digit(10).unwrap(),
                Some(chunk[1].to_digit(10).unwrap()),
            )
        } else {
            (chunk[0].to_digit(10).unwrap(), None)
        };

        disk_map.push(Chunk::File(FileChunk {
            size: file,
            id,
            attempted_to_fragment: false,
        }));

        if let Some(free_space) = free_space {
            disk_map.push(Chunk::Free(free_space));
        }
    }

    loop {
        let unfragmented_chunk_index = match disk_map
            .iter()
            .rev()
            .position(|item| match item {
                Chunk::File(FileChunk {
                    attempted_to_fragment,
                    ..
                }) => !attempted_to_fragment,
                _ => false,
            })
            .map(|index| disk_map.len() - 1 - index)
        {
            Some(index) => index,
            None => break,
        };
        let mut unfragmented_chunk = match disk_map[unfragmented_chunk_index] {
            Chunk::File(file) => file,
            _ => panic!("Matched wrong chunk"),
        };

        let free_chunk_index = match disk_map.iter().position(|item| match item {
            Chunk::Free(size) => *size >= unfragmented_chunk.size,
            _ => false,
        }) {
            Some(index) => index,
            None => {
                unfragmented_chunk.attempted_to_fragment = true;
                disk_map[unfragmented_chunk_index] = Chunk::File(unfragmented_chunk);

                continue;
            }
        };
        let free_chunk_size = match disk_map[free_chunk_index] {
            Chunk::Free(size) => size,
            _ => panic!("Matched wrong chunk"),
        };

        if unfragmented_chunk_index < free_chunk_index {
            unfragmented_chunk.attempted_to_fragment = true;
            disk_map[unfragmented_chunk_index] = Chunk::File(unfragmented_chunk);

            continue;
        }

        disk_map[unfragmented_chunk_index] = Chunk::Free(unfragmented_chunk.size);

        let free_chunk_size_left = free_chunk_size - unfragmented_chunk.size;
        if free_chunk_size_left > 0 {
            disk_map[free_chunk_index] = Chunk::Free(free_chunk_size_left)
        } else {
            disk_map.remove(free_chunk_index);
        }

        unfragmented_chunk.attempted_to_fragment = true;
        disk_map.insert(free_chunk_index, Chunk::File(unfragmented_chunk));
    }

    let mut expanded_disk_map: Vec<DiskItem> = Vec::with_capacity(disk_map.len());

    for chunk in disk_map {
        match chunk {
            Chunk::File(FileChunk { size, id, .. }) => {
                for _ in 0..size {
                    expanded_disk_map.push(DiskItem::Num(id));
                }
            }
            Chunk::Free(size) => {
                for _ in 0..size {
                    expanded_disk_map.push(DiskItem::Free);
                }
            }
        }
    }

    let mut checksum = 0usize;

    for (index, disk_item) in expanded_disk_map.iter().enumerate() {
        match disk_item {
            DiskItem::Free => {}
            DiskItem::Num(val) => checksum += val * index,
        }
    }

    println!("Part 2: checksum is {}", checksum);
}
