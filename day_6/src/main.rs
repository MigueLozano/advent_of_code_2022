
fn main() {
    let path = "input.txt";
    let file_content = std::fs::read_to_string(path)
        .expect("Could not read file");

    let result = get_result(file_content);
    println!("{}", result);
}

fn get_result(content: String) -> u32 {
    let mut result = 14;
    loop {
        let substr = &content[(result-14)..result];
        if !contains_duplicates(substr) {
            break;
        }
        result += 1;
    }
    return result as u32;
}

fn contains_duplicates(str: &str) -> bool {
    let mut c_arr: Vec<char> = str.chars().collect();
    c_arr.sort();
    c_arr.dedup();
    return str.len() != c_arr.len();
}
