use super::day_1;

pub fn route_puzzle<'a,I>(day: i32, part: i32, files: &mut I) 
    where I : Iterator<Item = &'a str> 
{
    match day {
        1 => day_1::execute(part, files),
        2 => println!("This day ({}) and part ({}) have not been implemented yet!", day, part),
        _ => println!("This day ({}) and part ({}) have not been implemented yet!", day, part)
    }
}