fn main() {
    let numbers = vec![10, 23, 45, 70, 11, 15, 90];
    let target = 70;
    let result = search_number(&numbers, target);
    println!("Search result: {:?}", result);

    let result = search_char("hello world", 'w');
    println!("Search result: {:?}", result);
}


fn search_number(numbers: &[i32], target: i32) -> (bool, usize) {
    for (index, value) in numbers.iter().enumerate() {
        if value == &target {
            println!("Found the target: {} at index {}", target, index);
            return (true, index);
        }
    }

    (false, 0)
}


fn search_char(text: &str, target: char) -> (bool, usize) {
    for (index, value) in text.chars().enumerate() {
        if value == target {
            println!("Found the target: {} at index {}", target, index);
            return (true, index);
        }
    }

    (false, 0)
}