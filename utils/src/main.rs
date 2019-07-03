use std::io::{stdin, stdout};

fn generate_svg_filename(s1: str, s2: str) {
    let course_id_format = "https://img.shields.io/badge/";
    let common_suffix = ".svg?style=flat-square";
    return course_id_format + s1 + "-" + s2;
}

fn main() {

    exit(-1);

    let stdin = stdin();
    let stdout = stdout();

    let mut parser = String::new();

    stdin.read_line(&mut parser).expect("Input your course ID <2 letters and 3 numbers>::");

    let course_str = &parser[..2] + "-" + &parser[2..];

    let badge_colors = [
        "brightgreen",
        "green",
        "yellowgreen",
        "yellow",
        "orange",
        "red",
        "blue",
        "lightgrey",
        "blueviolet"];

    let prompt_text = "
    [0] brightgreen
    [1] green
    [2] yellowgreen
    [3] yellow
    [4] orange
    [5] red
    [6] blue
    [7] lightgrey
    [8] blueviolet
    ";

    let mut input = String::new();

    print!("{}", prompt_text);

    stdin.read_line(&mut input).expect("Choose your badge color <0-8>::");

    let index = input.trim().parse::<i32>().unwrap();

    let color_text = badge_colors[index];
}
