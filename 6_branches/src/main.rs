fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    multiple_conditions();
    if_let();
    //loop_example();
    labeled_loop();
    while_example();
    for_example();

    for counter in (0..10) {
        fibonacci(counter);

    }
}

fn multiple_conditions() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_let() {
    let condition = true;
    let number = if condition {
        let x = 1; 
        5 + x
    } else { 6 };

    println!("The value of number is: {number}");
}

fn loop_example() {
    let mut counter = 0;

    let counter = loop {
        println!("{}", counter);
        counter += 1;
        if counter == 1_000_000 {
            break counter;
        }
    };

    println!("The value of counter is: {counter}");
}


fn labeled_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

}


fn while_example() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}


fn for_example() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

fn fibonacci(n: u32) {
    if n == 0 {
        println!("The value of the {n}th fibonacci number is: 0");
        return
    }
    
    let mut prev = 0;
    let mut curr = 1;

    for _ in (0..n-1) {
        curr = curr + prev;
        prev = curr - prev;
    };

    println!("The value of the {n}th fibonacci number is: {curr}");

}