
fn main() {
    let file_name = "resources/input.txt";
    let file_content = std::fs::read_to_string(file_name)
        .expect("Could not read file");

    let sum: u32 = file_content
        .lines()
        .map(|line| get_compartments(line))
        .map(|comps| find_common_items(comps))
        .map(|items| get_priority_sum(items))
        .sum();
    println!("Result: {}", sum);

    let sum: u32 = file_content
        .lines()
        .enumerate()
        .zip(file_content.lines().skip(1))
        .zip(file_content.lines().skip(2))
        .filter(|(((idx, _a), _b), _c)| idx%3==0)
        .map(|(((_idx, a), b), c)| find_common_item_prio(a, b, c))
        .sum();
    println!("Result 2: {}", sum);
}

fn find_common_item_prio(a: &str, b: &str, c: &str) -> u32 {
    let mut a_vec: Vec<char> = a.chars().collect();
    let mut b_vec: Vec<char> = b.chars().collect();
    let mut c_vec: Vec<char> = c.chars().collect();

    a_vec.sort();
    a_vec.dedup();
    b_vec.sort();
    b_vec.dedup();
    c_vec.sort();
    c_vec.dedup();

    let item =  a_vec
        .into_iter()
        .find(|ai| {
            return 
                b_vec.iter().any(|bi| ai.eq(&bi)) &&
                c_vec.iter().any(|ci| ai.eq(&ci));
        })
        .unwrap();
    return get_priority(item);
}

fn get_compartments(line: &str) -> Vec<&str> {
    let comp1_str = &line[..(line.len()/2)];
    let comp2_str = &line[(line.len()/2)..];
//    println!("Line: {} {}", comp1_str, comp2_str);
    return vec![
        comp1_str,
        comp2_str,
    ];
}

fn find_common_items(compartments: Vec<&str>) -> Vec<char> {
    let str1: &str = compartments.get(0).unwrap();
    let mut comp1: Vec<char> = str1.chars().collect();
    comp1.sort();
    comp1.dedup();

    let str2: &str = compartments.get(1).unwrap();
    let mut comp2: Vec<char> = str2.chars().collect();
    comp2.sort();
    comp2.dedup();

    let result: Vec<char> = comp1
        .into_iter()
        .filter(|item1| {
            comp2
                .iter()
                .any(|item2| item1.eq(&item2))
        })
        .collect();
 //   println!("  {:?}", result);

    return result;
}

fn get_priority(item: char) -> u32 {
    let ascii = item as u32;
    if ascii <= ('Z' as u32) { 
        return ascii - ('A' as u32) + 27;
    } else {
        return ascii - ('a' as u32) + 1;
    }
}

fn get_priority_sum(items: Vec<char>) -> u32 {
    return items
        .into_iter()
        .map(|item| get_priority(item))
        .sum();
}
