fn main() {
    let file_name = "resources/input.txt";
    let file_content = std::fs::read_to_string(file_name)
        .expect("Could not read file");

    let count: usize = file_content
        .lines()
        .map(|line| get_sections(line))
        .filter(|line| {
           return 
               line[0][0] <= line[1][0] && line[0][1] >= line[1][1] ||
               line[0][0] >= line[1][0] && line[0][1] <= line[1][1];
        })
        .count();

    println!("{:?}", count);

    let count = file_content
        .lines()
        .map(|line| get_sections(line))
        .filter(|line| {
            return !(line[0][0] > line[1][1] || line[0][1] < line[1][0])
        })
        .count();
    println!("{:?}", count);
}

fn get_sections(line: &str) -> Vec<Vec<u32>> {
    return line.split(',')
        .map(|elf| elf.split('-')
             .map(|section| section.parse().unwrap())
             .collect())
        .collect();
}
