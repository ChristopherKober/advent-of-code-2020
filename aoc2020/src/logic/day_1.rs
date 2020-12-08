use super::super::service::parser::get_file_lines;

const TARGET : i32 = 2020;

pub fn execute<'a,I>(part: i32, files: &mut I) 
    where I : Iterator<Item = &'a str>  
{
    println!("Solving Day 1: Report Repair...");

    match part {
        1 => execute_part_1(files),
        2 => execute_part_2(files),
        _ => println!("Unknown part input, aborting...")
    }
}

fn execute_part_1<'a,I>(files: &mut I) 
    where I : Iterator<Item = &'a str> 
{
    println!("Solving Part 1. Using target number {}", TARGET);

    let file_name = files.next();
    let lines = get_file_lines(file_name.unwrap()).unwrap();
    let lines : Vec<i32> = lines.map(|l| l.unwrap().parse::<i32>().unwrap()).collect();

    let mut num1 : i32 = -1;
    let mut num2 : i32 = -1;

    'outer: for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i == j { continue }

            if lines[i] + lines[j] == TARGET {
                num1 = lines[i];
                num2 = lines[j];
                break 'outer;
            }
        }
    }

    if num1 + num2 == TARGET {
        println!("Found numbers {} and {}",num1, num2);
        println!("Multiplying them together gives: {} * {} = {}",num1, num2, num1 * num2);
    }
}

fn execute_part_2<'a,I>(files: &mut I) 
    where I : Iterator<Item = &'a str> 
{
    println!("Solving Part 2. Using target number {}", TARGET);

    let file_name = files.next();
    let lines = get_file_lines(file_name.unwrap()).unwrap();
    let lines : Vec<i32> = lines.map(|l| l.unwrap().parse::<i32>().unwrap()).collect();

    let mut num1 : i32 = -1;
    let mut num2 : i32 = -1;
    let mut num3 : i32 = -1;

    'outer: for i in 0..lines.len() {
        for j in 0..lines.len() {
            for k in 0..lines.len() {
                if i == j || i == k || j == k { continue }

                if lines[i] + lines[j] + lines[k] == TARGET {
                    num1 = lines[i];
                    num2 = lines[j];
                    num3 = lines[k];
                    break 'outer;
                }
            }
        }
    }

    if num1 + num2 + num3 == TARGET {
        println!("Found numbers {}, {}, and {}",num1, num2, num3);
        println!("Multiplying them together gives: {} * {} * {} = {}",num1, num2, num3, num1 * num2* num3);
    }
}