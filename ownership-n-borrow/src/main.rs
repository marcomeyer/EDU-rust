
fn take_ownership_sum(v: Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in v {
        sum += value;
    }

    return sum;
}

fn borrow_sum(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in v {
        sum += *value;
    }

    return sum;
}

fn move_it() {
    let values = vec![1,2,3,4,5];
    let sum = take_ownership_sum(values);
    println!("{}", sum);
    
    // DOESNT COMPILE: value borrowed after move
    // println!("Sum of {} values {}", values.len(), sum)
}

fn borrow_it() {
    let values = vec![1,2,3,4,5];
    let sum = borrow_sum(&values);
    println!("{}", sum);
    
    // DOES WORK
    println!("Sum of {} values {}", values.len(), sum)
}

fn main() {
    move_it();
    borrow_it();
}