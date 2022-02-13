use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;
use colored::Colorize;
fn main() {
    count_for_four([1, 9, 8, 9]);

    /*
    for mut i in 0..10 {
        for mut j in 0..10 {
            countForTwo([i, j]);
        }
    }*/
}
fn count_for_four( init: [u8;4]) {
    println!("1 9 8 9");
    let mut all = init;
    let mut max: u8 = 0;
    let mut min: u8 = 200;
    for mut i in 1..1561 {
        let sum: u8 = all.iter().sum();
        if (sum < min) {
            min = sum;
        }
        if (sum > max) {
            max = sum;
        }

        let new_item = sum % 10;
        print!("{}.", i);
        print!(" {}", new_item);
        pt_blue(format!(" {}", sum));
        pt_green(format!(" {}", new_item as i32 - all[0] as i32));
        all = change_all_four(all, new_item);
        println!();
    }
    print!("Min: {}   | Max: {}", min, max);
}

fn count_for_two( init: [u8;2]) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("prdel")
        .unwrap();

    println!("1 9 8 9");
    let mut all = init;
    let mut max: u8 = 0;
    let mut min: u8 = 200;
    for mut i in 1..100 {
        let sum: u8 = all.iter().sum();
        if sum < min {
            min = sum;
        }
        if sum > max {
            max = sum;
        }

        let new_item = sum % 10;
        print!("{}.", i);
        print!(" {}", new_item);
        pt_blue(format!(" {}", sum));
        pt_green(format!(" {}", all[0] as i32 - new_item as i32));
        all = change_all(all, new_item);
        println!();
        if all[0] == init[0] && all[1] == init[1] {
            if i == 0 {
                continue;
            }
            let s = println!("for [{}, {}] Found at: {}",init[0], init[1], i);
            file.write(format!("for [{}, {}] Found at: {}\n", init[0], init[1], i).as_bytes()).unwrap();
            break;
        }
    }
    print!("Min: {}   | Max: {}", min, max);
    Ok(())
}

fn pt_blue(s: String){
    print!(" {}", s.blue());
}
fn pt_green(s: String){
    print!(" {}", s.green());
}
fn change_all_four(all: [u8;4], p0: u8) -> [u8; 4] {
    [all[1], all[2], all[3], p0]
}
fn change_all(all: [u8;2], p0: u8) -> [u8; 2] {
    [all[1], p0]
}

fn bit_print4(num: u8){
    let mut s= format!("{:#006b}", num);
    let mut x = s.strip_prefix("0b");
    let value = x.as_deref().unwrap_or("default string");
    println!("{}", value);
}