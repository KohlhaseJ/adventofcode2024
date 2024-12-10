use std::path::Path;

use super::filehelper;

pub fn solve(file_path: impl AsRef<Path>) -> String {

    let contents = filehelper::lines_from_file(file_path);
    let (rules, updates) = get_rules_and_updates(&contents);

    let mut page_numbers = 0;
    let mut corrected_page_numbers = 0;
    
    for update in updates {
        let (in_correct_order, corrected) = is_in_correct_order(&update, &rules); 
        if in_correct_order {
            page_numbers += update[update.len()/2];
        } else {
            corrected_page_numbers += corrected[corrected.len()/2];
        }
    }

    return format!("sum of page numbers {}, sum of corrected page numbers {}", page_numbers, corrected_page_numbers);
}

fn get_rules_and_updates(contents : &Vec<String>) -> (Vec<(usize, usize)>, Vec<Vec<usize>>){
    
    let mut rules: Vec<(usize, usize)> = Vec::new(); 
    let mut updates: Vec<Vec<usize>> = Vec::new();

    let mut in_update_section = false;
    for line in contents {
        if line.trim().is_empty() {
            in_update_section = true;
            continue;
        }

        if in_update_section {
            updates.push(line.split(",").map(|s| s.parse().unwrap()).collect());
        }
        else {
            let pipe_separated : Vec<&str> = line.split("|").collect();
            assert!(pipe_separated.len() == 2);
            rules.push((pipe_separated[0].parse().unwrap(), pipe_separated[1].parse().unwrap()));
        }
    }
    return (rules, updates);
}

fn is_in_correct_order(update: &Vec<usize>, rules: &Vec<(usize, usize)>) -> (bool, Vec<usize>) {
    let mut in_correct_order = true;
    let mut corrected = update.clone();
    let mut i : usize = 0;

    while i < update.len() {
        let mut in_correct_place = true;
        let page_number = corrected[i];
        let left = (0..i).map(|li| corrected[li]).collect::<Vec<usize>>();
        let right = (i+1..corrected.len()).map(|ri| corrected[ri]).collect::<Vec<usize>>();

        for rule in rules {
            if !(corrected.contains(&rule.0) && corrected.contains(&rule.1)) {
                continue;
            }
            
            if page_number == rule.0 && left.contains(&rule.1) {
                in_correct_order = false;
                in_correct_place = false;
                corrected.remove(corrected.iter().position(|el| el == &page_number).unwrap());
                corrected.insert(corrected.iter().position(|el| el == &rule.1).unwrap(), page_number);
                break;
            }

            if page_number == rule.1 && right.contains(&rule.0) {
                in_correct_order = false;
                in_correct_place = false;
                corrected.remove(corrected.iter().position(|el| el == &page_number).unwrap());
                corrected.insert(corrected.iter().position(|el| el == &rule.0).unwrap() + 1, page_number);
                break;
            }
        }
        if in_correct_place {
            i += 1;
        }
    }
    return (in_correct_order, corrected);
}
