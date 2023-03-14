
fn main() {
    let file_name = "resources/input.txt";
    let file_content = std::fs::read_to_string(file_name)
        .expect("Could not read file");

    let stacks = get_stacks(&file_content);
    execute_movements(&file_content, stacks);
}

fn execute_movements(content: &str, mut stacks: Vec<Vec<char>>) {
    content
        .lines()
        .skip(10)
        .map(|line| {
            let splt: Vec<&str> = line.split(' ').collect();
            return vec![splt[1], splt[3], splt[5]];
        })
        .map(|line| {
            println!("{:?}", line);
            return line
                .iter()
                .filter(|item| item.chars().all(char::is_numeric))
                .map(|item| item.parse::<usize>().unwrap())
                .collect()
        })
        .for_each(|line: Vec<usize>| {
            let mut items = vec![];
            for _ in 0..line[0] {
                let item = stacks[line[1]-1].pop().unwrap();
                items.insert(0, item);
            }
            stacks[line[2]-1].append(&mut items);
        });
    let stacks: Vec<&char> = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect();
        
    println!("{:?}", stacks);
} 

fn get_stacks(content: &str) -> Vec<Vec<char>> {
    // Initialize stacks
    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..9 {
        stacks.push(vec![]);
    }
    // Get crate names
    let names_str: &str = content
        .lines()
        .skip(8)
        .take(1)
        .collect::<Vec<&str>>()
        .first()
        .unwrap();
    content
        .lines()
        .take(8)
        .for_each(|line| {
            line
                .chars()
                .enumerate()
                .for_each(|(idx, c)| {
                    if c.is_alphabetic() {
                        let stack_idx = names_str
                            .chars()
                            .nth(idx)
                            .unwrap()
                            .to_digit(10)
                            .unwrap_or_default() - 1;
                        let stack_idx = usize::try_from(stack_idx).unwrap();
                        stacks[stack_idx].insert(0, c);
                    }
                })
        });
    return stacks;
}
