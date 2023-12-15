use std::{
    fs::{self, File},
    io::{self, stdin, Read, Write},
    path::Path,
};

fn update_file() {}

fn delete_content() {
    read_from_file();

    print!("Enter Task No. U Wish To Delete: ");
    io::stdout().flush().unwrap();

    let mut user_choice: String = String::new();

    stdin()
        .read_line(&mut user_choice)
        .ok()
        .expect("Failed To Delete Task");

    let user_choice: usize = user_choice.trim().parse::<usize>().unwrap();

    clearscreen::clear().expect("Failed To Clear Console");

    let mut data_file: fs::File = fs::OpenOptions::new().read(true).open("data.txt").unwrap();

    let mut file_content: String = String::new();

    data_file.read_to_string(&mut file_content).unwrap();

    let lines: Vec<String> = file_content
        .split("\n")
        .map(|s: &str| s.to_string())
        .collect();

    let mut data_file: fs::File = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("data.txt")
        .unwrap();

    println!("Data {:?}", lines);

    // let total_length: usize = lines.len();

    for (index, line) in lines.iter().enumerate() {
        if index + 1 == user_choice {
            return;
        }

        data_file
            .write(line.as_bytes())
            .expect("Error Occurred While Writing Content To File");
    }

    println!("Task Deleted");
}

fn file_checker() -> bool {
    Path::new("data.txt").exists()
}

fn read_from_file() {
    clearscreen::clear().expect("Failed To Clear Console");
    println!("Here's The List Of Task\n");

    let mut data_file: File = File::open("data.txt").unwrap();

    let mut file_content: String = String::new();

    data_file.read_to_string(&mut file_content).unwrap();

    let lines: Vec<String> = file_content
        .split("\n")
        .map(|s: &str| s.to_string())
        .collect();

    let total_length: usize = lines.len();

    for (index, line) in lines.iter().enumerate() {
        if total_length - 1 == index {
            return;
        }

        println!("{}: {:?}", index + 1, line.replace("\r", ""));
    }
}

fn write_to_file(input_string: &mut String) {
    clearscreen::clear().expect("Failed To Clear Console");

    if file_checker() {
        println!("Opening File");
        let mut data_file: fs::File = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("data.txt")
            .unwrap();

        data_file
            .write(input_string.as_bytes())
            .expect("Error Occurred While Writing Content To File");

        println!("Task Added");

        read_from_file();
    } else {
        println!("Creating File");
        let mut data_file: File = File::create("data.txt").unwrap();

        data_file
            .write(input_string.as_bytes())
            .expect("Error Occurred While Writing Content To File");

        read_from_file();
    }
}

fn take_user_input() {
    clearscreen::clear().expect("Failed To Clear Console");
    io::stdout().flush().unwrap();

    let mut input_string: String = String::new();

    stdin()
        .read_line(&mut input_string)
        .ok()
        .expect("Failed To Add Task");

    write_to_file(&mut input_string);
}

fn main() {
    clearscreen::clear().expect("Failed To Clear Console");
    println!("Im Hoping You Are Having Best Time\n1 - Write\n2 - Read\n3 - Update\n4 - Delete");
    print!("Choose <1, 2, 3, 4> | ");
    io::stdout().flush().unwrap();

    let mut user_choice: String = String::new();

    stdin().read_line(&mut user_choice).unwrap();
    let user_choice: i32 = user_choice.trim().parse::<i32>().unwrap();

    match user_choice {
        1 => take_user_input(),
        2 => read_from_file(),
        3 => update_file(),
        4 => delete_content(),
        _ => panic!("Error"),
    }
}
