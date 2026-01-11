fn main() {
    let numbers = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    let target = 20;
    
    let (found, index) = binary_search(&numbers, target);
    if found {
        println!("Found {} at index {}", target, index);
    } else {
        println!("{} not found", target);
    }
}


fn binary_search(numbers: &[i32], target: i32) -> (bool, usize) {
    let mut numbers = numbers;

    loop {
        let center = (numbers.len() / 2) as usize;

        if numbers.len() < center + 1 {
            return (false, 0);
        }

        let center_value = numbers[center];
        if target < center_value {
            numbers = &numbers[..center];
        } else if target > center_value {
            numbers = &numbers[center + 1..];
        } else {
            return (true, center)
        }
    }

    return (false, 0);
}