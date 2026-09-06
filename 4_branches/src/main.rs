fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");


    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter >= 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // loop labels with loop returns
    let mut count = 0;
    let break_val = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up 2 * 2;
            }
            remaining -= 1;
        }

        count += 1;
    };

    println!("\n\nThe value of break_val is: {break_val}");
    println!("End count = {count}");

    // while loop over collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // for loop over collection
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // for loop over range (more concise than while)
    for number in (1..=4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}