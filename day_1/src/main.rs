
fn print_top_n_calories(calories_vector: Vec<Vec<u32>>, n: usize) {
    let mut sum_of_calories =  calories_vector
        .iter()
        .map(|cals| cals
             .iter()
             .sum())
        .collect::<Vec<u32>>();
    sum_of_calories
        .sort_by(|a, b| b.cmp(a));
    let mut total_sum = 0u32;
    sum_of_calories
        .iter()
        .enumerate()
        .take(n)
        .for_each(|(idx, cals)| {
            println!("{}: {}", idx+1, cals);
            total_sum += cals;
        });
    println!("Total sum: {}", total_sum);
}

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
    print_top_n_calories(vectors, 3);

    println!("done.");
}
