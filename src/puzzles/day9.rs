use std::path::Path;
use super::filehelper;

pub fn solve(file_path: impl AsRef<Path>) -> String {

    let content = filehelper::string_from_file(file_path);

    let disk_blocks = generate_disk_blocks(&content);
    let defragmented_disk_blocks = defragment_disk_blocks(&disk_blocks);
    let moved_disk_blocks = move_files_to_free_space(&disk_blocks);
    
    let checksum = calculate_checksum(&defragmented_disk_blocks);
    let moved_checksum = calculate_checksum(&moved_disk_blocks);

    return format!("checksum {}, moved checksum {}", checksum, moved_checksum);
}

fn generate_disk_blocks(content: &String) -> Vec<i32> {
    let mut disk_blocks: Vec<i32> = Vec::new();
    let mut is_file = true;
    let mut file_index = 0;

    for char in content.chars() {
        for _i in 0..char.to_digit(10).unwrap() {
            if is_file {
                disk_blocks.push(file_index);
            } else {
                disk_blocks.push(-1);
            }

        }
        is_file = !is_file;
        if is_file  {
            file_index += 1;
        }        
    }

    return disk_blocks;
}

fn defragment_disk_blocks(disk_blocks: &Vec<i32>) -> Vec<i32> {
    let mut defragmented_disk_blocks: Vec<i32> = disk_blocks.clone();

    let mut first_free_index = disk_blocks.iter().position(|&i| i == -1).unwrap();
    let mut last_file_index = disk_blocks.iter().rposition(|&i| i >= 0).unwrap();
    
    while first_free_index < last_file_index {
        defragmented_disk_blocks.swap(first_free_index, last_file_index);

        first_free_index = defragmented_disk_blocks.iter().position(|&c| c == -1).unwrap();
        last_file_index = defragmented_disk_blocks.iter().rposition(|&i| i >= 0).unwrap();
    }

    return defragmented_disk_blocks;
}

fn move_files_to_free_space(disk_blocks: &Vec<i32>) -> Vec<i32> {
    let mut moved_disk_blocks: Vec<i32> = disk_blocks.clone();
    let max_file_index = *disk_blocks.iter().max().unwrap();

    for i in (0..max_file_index+1).rev() {
        let file_size = disk_blocks.iter().filter(|&c| *c == i).count();
        let mut file_sized_windows = moved_disk_blocks.windows(file_size);
        let free_space_slice = file_sized_windows.position(|slice| slice.iter().all(|&c| c == -1));
        let file_index = moved_disk_blocks.iter().position(|&c| c == i).unwrap();

        if free_space_slice.is_some() {
            let free_space_index = free_space_slice.unwrap();
            if free_space_index > file_index {
                continue;
            }

            for j in 0..moved_disk_blocks.len() {
                if moved_disk_blocks[j] == i {
                    moved_disk_blocks[j] = -1;
                }
            }

            for j in 0..file_size {
                moved_disk_blocks[free_space_index + j] = i;
            }
        }
    }

    return moved_disk_blocks;
}

fn calculate_checksum(disk_blocks: &Vec<i32>) -> usize {
    let mut checksum = 0;
    for i in 0..disk_blocks.len() {
        if disk_blocks[i] == -1 {
            continue;
        }
        checksum += i * disk_blocks[i] as usize;
    }
    return checksum;
}