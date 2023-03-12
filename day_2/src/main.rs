use regex::Regex;

fn main() {
    let file_name = "resources/strategy.txt";
    let file_content = std::fs::read_to_string(file_name)
        .expect("Could not read file");

    let total_points_first_strat: u32 = file_content
        .lines()
//        .take(5)
        .map(|line| get_points_first_strat(line).unwrap())
        .sum();
    let total_points_second_strat: u32 = file_content
        .lines()
//        .take(5)
        .map(|line| get_points_second_strat(line).unwrap())
        .sum();

    println!("Total points  (first strategy): {}", total_points_first_strat);
    println!("Total points (second strategy): {}", total_points_second_strat);
    println!("Done.");
}

fn get_points_second_strat(line: &str) -> Result<u32, ()> {
    let re_check = Regex::new(r"(A|B|C) (X|Y|Z)").unwrap();
    if !re_check.is_match(line) {
        return Err(());
    }

    let mut points = 0;
    let played: &str = line.split_whitespace().last().unwrap();
    match played {
        "Y" => points += 3, // Draw
        "Z" => points += 6, // Win
        _ => (),
    };

    let re_rock = Regex::new(r"A Y|B X|C Z").unwrap();
    let re_paper = Regex::new(r"A Z|B Y|C X").unwrap();
    let re_scissors = Regex::new(r"A X|B Z|C Y").unwrap();
    
    if re_rock.is_match(line) {
        points += 1;
    } else if re_paper.is_match(line) {
        points += 2;
    } else if re_scissors.is_match(line) {
        points += 3;
    }
    return Ok(points);
}

fn get_points_first_strat(line: &str) -> Result<u32, ()> {
    let re_check = Regex::new(r"(A|B|C) (X|Y|Z)").unwrap();
    if !re_check.is_match(line) {
        return Err(());
    }

    let mut points = 0;
    let played: &str = line.split_whitespace().last().unwrap();
    match played {
        "X" => points += 1,
        "Y" => points += 2,
        "Z" => points += 3,
        _ => (),
    };

    let re_win = Regex::new(r"A Y|B Z|C X").unwrap();
    let re_draw = Regex::new(r"A X|B Y|C Z").unwrap();
    
    if re_win.is_match(line) {
        points += 6;
    } else if re_draw.is_match(line) {
        points += 3;
    }
    return Ok(points);
}
