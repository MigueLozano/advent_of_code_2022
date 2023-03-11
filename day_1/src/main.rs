
fn get_calorie_vectors(content: String) -> Vec<Vec<u32>> {
    let mut result: Vec<Vec<u32>> = vec![];
    let mut vector: Vec<u32> = vec![];
    for line in content.lines() {
        if line.eq("") {
            result.push(vector.to_vec());
            vector.clear();
        } else {
            vector.push(line.parse()
                        .expect("Line could not be parsed"));
        }
    }
    return result;
}

fn main() {
    let file_name = "resources/calories.txt";
    let file_content = std::fs::read_to_string(file_name)
        .expect("Could not read file");
    let vectors = get_calorie_vectors(file_content);
    let max: u32 = vectors
        .iter()
        .map(|cals| cals
             .iter()
             .sum())
        .max()
        .unwrap();

    println!("Max calories: {}", max);
    println!("done.");
}
