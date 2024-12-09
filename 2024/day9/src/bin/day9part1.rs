use std::fs::File;

use advent_of_code_2024_day_9::reading::{read, DiskSegment};

fn main() {
    let mut input = read();
    compact(&mut input);
    let result: Vec<&DiskSegment> = input.iter().flat_map(|x| x.iter()).collect();
    let checksum = result
        .iter()
        .fold((0 as usize, 0 as usize), |(idx, acc), &x| match x {
            DiskSegment::DiskFile { id, space } => {
                let end_idx = idx + (*space as usize);
                let mut acc = acc;
                for i in idx..end_idx {
                    acc += i * (*id);
                }
                (end_idx, acc)
            }
            DiskSegment::FreeSpace(space) => (idx + (*space as usize), acc),
        })
        .1;
    println!("Checksum: {}", checksum);
}

fn compact(input: &mut Vec<Vec<DiskSegment>>) {
    for slot_idx in (0..input.len()).rev() {
        if input[slot_idx].len() == 1 {
            if let DiskSegment::DiskFile { id, space } = input[slot_idx][0] {
                let freespace_start = find_free_space(&input, slot_idx, space);
                if freespace_start < slot_idx {
                    // Free space found
                    if let DiskSegment::FreeSpace(free_space) =
                        input[freespace_start].last().unwrap().clone()
                    {
                        *(input[freespace_start].last_mut().unwrap()) =
                            DiskSegment::DiskFile { id, space };
                        if free_space > space {
                            input[freespace_start].push(DiskSegment::FreeSpace(free_space - space));
                        }
                    }
                    input[slot_idx] = vec![DiskSegment::FreeSpace(space)];
                }
            }
        }
    }
}

fn find_free_space(input: &Vec<Vec<DiskSegment>>, end: usize, size: u8) -> usize {
    for freespace_srart in 0..end {
        match input[freespace_srart].last().unwrap() {
            DiskSegment::FreeSpace(free_space) if *free_space >= size => return freespace_srart,
            _ => {}
        }
    }
    return end;
}
