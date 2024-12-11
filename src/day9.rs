use std::fs;

const RADIX: u32 = 10;

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).unwrap();
}

fn parse_problem(pb: &str) -> Vec<(u32, u32)> {
    let mut vec: Vec<(u32, u32)> = Vec::new();
    let mut file_size: u32 = 0;
    for (i, digit) in pb.chars().enumerate() {
        let num = digit.to_digit(RADIX).unwrap();
        if i%2 == 0 {
            // even
            file_size = num;
        } else {
            let free_blocks = num;
            vec.push((file_size, free_blocks));
        }
    }
    if pb.len() % 2 == 1 {
        vec.push((file_size, 0))
    }
    return vec;
}

fn show_structure(file: &[(u32, u32)]) -> String {
    let mut buffer: Vec<char> = Vec::new();
    for (_id, (file_size, free_blocks)) in file.into_iter().enumerate() {
        for _ in 0..*file_size {
            //let to_print = char::from_digit(id.try_into().unwrap(), RADIX).unwrap();
            let to_print = '#';
            buffer.push(to_print);
        }
        for _ in 0..*free_blocks {
            buffer.push('.');
        }
    }
    return buffer.into_iter().collect();
}

fn print_fs(filesystem: &[Option<u32>]) -> String {
    let mut buffer: Vec<char> = Vec::new();
    for i in filesystem {
        if i.is_none() {
            buffer.push('.');
        } else {
            buffer.push('#');
        }
    }
    return buffer.into_iter().collect();
}

fn deduce_fs(file: &[(u32, u32)]) -> Vec<Option<u32>> {
    let mut buffer: Vec<Option<u32>> = Vec::new();
    for (id, (file_size, free_blocks)) in file.into_iter().enumerate() {
        for _ in 0..*file_size {
            buffer.push(Some(id.try_into().unwrap()));
        }
        for _ in 0..*free_blocks {
            buffer.push(None);
        }
    }
    return buffer;
}

fn compact_fs(filesystem: &mut Vec<Option<u32>>) {
    let len = filesystem.len();
    let mut end_pos = len - 1;
    while filesystem.get(end_pos).unwrap().is_none() {
        end_pos -= 1;
    }
    for pos in 0..len {
        let file_block = filesystem.get(pos).unwrap();
        if file_block.is_none() {
            filesystem.swap(pos, end_pos);
            end_pos -= 1;
        }
        while filesystem.get(end_pos).unwrap().is_none() {
            end_pos -= 1;
        }
        if end_pos <= pos {
            break;
        }
    }
}

fn swap_blocks(filesystem: &mut Vec<Option<u32>>, start: usize, end: usize, file_size: usize) {
    for i in 0..file_size {
        filesystem.swap(start+i, end-file_size+i+1);
    }
}

// O(n^2) non-optimal!
fn compact_fs_p2(filesystem: &mut Vec<Option<u32>>, files: &[(u32, u32)]) {
    let mut end_pos = filesystem.len() - 1;
    let mut moved_files = vec![false; files.len()];
    while end_pos > 0 {
        let mut current_node = filesystem.get(end_pos).unwrap();
        while current_node.is_none() {
          end_pos -= 1;
          current_node = filesystem.get(end_pos).unwrap();
        }
        if let Some(id) = current_node {
            let (file_size, _) = files.get(*id as usize).unwrap(); 
            if moved_files[*id as usize] {
                // already processed
                let shift_by = *file_size as usize;
                end_pos -= shift_by;
                continue;
            }
            //println!("Processing file ID {} at {} with size {}", *id, end_pos, file_size);
            // look from the beginning:
            let mut i = 0;
            while i < end_pos {
                if let Some(occupied) = filesystem.get(i).unwrap() {
                    let (skip_by, _) = files.get(*occupied as usize).unwrap();
                    i += *skip_by as usize;
                } else {
                  // found empty slot
                    let mut empty_size = 1;
                    while filesystem.get(i+empty_size).unwrap().is_none() {
                        empty_size += 1;
                    }
                    //println!("Found empty slot at i: {} with size {}", i, empty_size);
                    if empty_size >= (*file_size).try_into().unwrap() {
                        //println!("{:?}", filesystem);
                        //println!("Swapping ID {} at {} with {} (file_size {})", *id, i, end_pos, *file_size);
                        moved_files[*id as usize] = true;
                        // swap blocks [i+file_size) and (end_pos-file_size; end_pos]
                        swap_blocks(filesystem, i, end_pos, *file_size as usize);
                        //println!("{:?}", filesystem);
                        //end_pos -= *file_size as usize;
                        break;
                    } else {
                        // look for next empty spot
                        //println!("Empty size not big enough for size {}", *file_size);
                        i += empty_size;
                    }
                }
                //i += 1;
            }
            //end_pos -= 1;
            if (end_pos as i32 - *file_size as i32) < 0 {
                break;
            } else {
                end_pos -= *file_size as usize;
            }
        }
    }
}

fn compute_checksum(filesystem: &[Option<u32>]) -> u64 {
    let mut checksum: u64 = 0;
    for (pos, block) in filesystem.into_iter().enumerate() {
        if let Some(id) = block {
            checksum += pos as u64 * (*id as u64);
        }
    }
    return checksum;
}

pub fn day9() {
    //let text = r"12345";
    /*let text = r"
2333133121414131402";*/
    //let text = read_file("inputs/evil.txt"); // 97898222299196
    let text = read_file("inputs/day9.txt");
    //let text = read_file("inputs/more_evil.txt"); // 5799706413896802
    let files = parse_problem(&text.trim());
    let mut filesystem = deduce_fs(&files);
    //println!("{:?}", filesystem);
    compact_fs(&mut filesystem);
    //println!("compact fs: {}", print_fs(&filesystem));
    //println!("{:?}", filesystem);
    let mut checksum = compute_checksum(&filesystem);

    println!("Solution to day 9 part 1: {}", checksum); // 6390180901651
    filesystem = deduce_fs(&files);
    //println!("{:?}", filesystem);
    compact_fs_p2(&mut filesystem, &files);
    //println!("{:?}", filesystem);
    //println!("compact fs: {}", print_fs(&filesystem));
    checksum = compute_checksum(&filesystem);
    println!("Solution to day 9 part 2: {}", checksum); // 6412390114238 
}
