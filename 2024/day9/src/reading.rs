use std::io::stdin;

static ZERO: u8 = '0' as u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiskSegment {
    DiskFile { id: usize, space: u8 },
    FreeSpace(u8)
}

pub fn read() -> Vec<Vec<DiskSegment>> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().as_bytes();
    let mut buffer: Vec<Vec<DiskSegment>> = vec![];
    parse(input, 0, true, &mut buffer);
    buffer
}

pub fn parse(input: &[u8], id: usize, is_file: bool, buffer: &mut Vec<Vec<DiskSegment>>) {
    if input.is_empty() {
        return;
    }
    let space = input[0] - ZERO;
    let tail = &input[1..];

    if is_file {
        buffer.push(vec![DiskSegment::DiskFile { id, space }]);
        parse(tail, id + 1, false, buffer);
    } else {
        buffer.push(vec![DiskSegment::FreeSpace(space)]);
        parse(tail, id, true, buffer);
    }
}
