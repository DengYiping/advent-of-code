use std::fs::File;

use advent_of_code_2024_day_9::reading::{read, DiskSegment};

fn main() {
    let mut input = read();
    compact(&mut input);
    let result: Vec<&DiskSegment> = input.iter().flat_map(|x| x.iter())
        .filter(|&x| match x {
            DiskSegment::DiskFile { id: _, space } => *space > 0,
            DiskSegment::FreeSpace(_) => false
        })
        .collect();
    let checksum = result.iter()
        .fold((0 as usize, 0 as usize), |(idx, acc), &x| {
            match x {
                DiskSegment::DiskFile { id, space } => {
                    let end_idx = idx + (*space as usize);
                    let mut acc = acc;
                    for i in idx..end_idx {
                        acc += i * (*id);
                    }
                    (end_idx, acc)
                },
                DiskSegment::FreeSpace(_) => panic!("Do not expect freespace here")
            }
        })
        .1;
    println!("Checksum: {}", checksum);
}


fn compact(input: &mut Vec<Vec<DiskSegment>>) {
    let mut freespace_start = 0;
    for slot_idx in (0..input.len()).rev() {
        if input[slot_idx].len() == 1 {
            if let DiskSegment::DiskFile { id, space } = input[slot_idx][0] {
                let mut space_to_pack = space;
                while space_to_pack > 0 && freespace_start < slot_idx {
                    freespace_start = find_free_space(&input, freespace_start, slot_idx);
                    if freespace_start < slot_idx {
                        // Free space found
                        if let DiskSegment::FreeSpace(free_space) = input[freespace_start][0] {
                            if free_space < space_to_pack {
                                input[freespace_start][0] = DiskSegment::FreeSpace(0);
                                input[freespace_start].push(DiskSegment::DiskFile { id, space: free_space});
                                space_to_pack -= free_space;
                            } else {
                                input[freespace_start][0] = DiskSegment::FreeSpace(free_space - space_to_pack);
                                input[freespace_start].push(DiskSegment::DiskFile { id, space: space_to_pack});
                                space_to_pack = 0;
                            }
                        }
                    }
                }
                input[slot_idx][0] = DiskSegment::DiskFile { id, space: space_to_pack };
            }
        }
    }
}

fn find_free_space(input: &Vec<Vec<DiskSegment>>, start: usize, end: usize) -> usize {
    let mut freespace_srart = start;
    while freespace_srart < end {
        match input[freespace_srart][0] {
            DiskSegment::FreeSpace(free_space) if free_space > 0 => return freespace_srart,
            _ => {
                freespace_srart += 1;
            }
        }
    }
    return end;
}
