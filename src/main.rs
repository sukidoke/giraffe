use gnuplot::{Figure, Caption, Color};
extern crate nom;

use nom::{
    IResult, 
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple};

mod giraffe;

fn main() {
    let data = giraffe::data();

    let x_column = data.value_of("x_value").unwrap();
    let x_column_num = x_column.parse::<u64>().expect("x column should be an integer");

    let y_column = data.value_of("y_value").unwrap();
    let y_column_num = y_column.parse::<u64>().expect("y column should be an integer");

let x = [0u32, 1, 2];
let y = [3u32, 4, 5];
let mut fg = Figure::new();
fg.axes2d()
.lines(&x, &y, &[Caption("A line"), Color("black")]);
fg.show().unwrap();

    
}
