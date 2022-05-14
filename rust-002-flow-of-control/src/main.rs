#![allow(dead_code)]
use std::mem;

fn main() {
    //println!("Hello, world!");
    // if_flow();
    //loop_flow();

    let mut my_num = 100;
    {
        my_num = 333;
        println!("{}",my_num);

    }
    println!("{}", my_num);

}

fn loop_flow() {

    println!("we will get a loop here ");

    let tail_number = 100;
    let mut start_number = 0u32;

    loop {

        start_number += 1;
        println!("start number now is {}", start_number);

        if tail_number == start_number {
            break;
        }
    }
}

/*
fn if_flow() {
    let num = 10;
    if num > 10 {
        println!("n is {} and {} > 10", num, num);
    } else if num == 10 {
        println!("now num is {}", num);
    } else {
        println!("you got nothing !");
    }

    // 表达式返回
    let result =
        if num < 10 {
            println!("num less than {}, you will get a {} + 5", num, num);
            num + 5
        } else {
            println!("you got num + 4");
            num + 4
        };

    println!("we got a if statement, and result is {}", result);
}
*/