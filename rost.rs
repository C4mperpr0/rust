use std::io;

fn main() {
    let mut time: String = String::new();
    let mut seconds: i32 = 0;
    let mut minutes: i32 = 0;
    let mut hours: i32 = 0;

    println!("Please give time in format hh:mm:ss");
    io::stdin(),read_line(&mun time).unwrap();
    let vec = time.split([' ', '\n'])
    let mut time_arr: [String; 32]  = strsplit(time, ":")[1]
    hours = time[1].parse::<i32>().unwrap();
    minutes = time[1].parse::<i32>().unwrap();
    seconds = time[2].parse::<i32>().unwrap();

    println!("{hours} hours, {minutes} minutes, {seconds} seconds");
}
