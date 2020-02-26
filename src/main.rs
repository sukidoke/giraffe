extern crate csv;

use gnuplot::{Color, Figure};
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;

use gnuplot::AxesCommon;

mod giraffe;

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn eat(xval: usize, yval: usize) -> Result<(Vec<i64>, Vec<i64>), Box<dyn Error>> {
    let mut x = Vec::new();
    let mut y = Vec::new();

    let file_path = get_first_arg()?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;

    for result in rdr.records() {
        let record = result?;
        x.push(record.get(xval).unwrap().parse::<i64>().unwrap());
        y.push(record.get(yval).unwrap().parse::<i64>().unwrap());
    }

    Ok((x, y))
}

fn main() {
    let data = giraffe::data();

    let x_column = data
        .value_of("x_value")
        .unwrap()
        .parse::<usize>()
        .expect("x column should be an integer");

    let y_column = data
        .value_of("y_value")
        .unwrap()
        .parse::<usize>()
        .expect("y column should be an integer");

    let title = data.value_of("title").unwrap();
    let x_label = data.value_of("x_label").unwrap();
    let y_label = data.value_of("y_label").unwrap();

    match eat(x_column, y_column) {
        Ok((x, y)) => {
            let mut fg = Figure::new();
            fg.set_title(title)
                .axes2d()
                .set_x_label(x_label, &[])
                .set_y_label(y_label, &[])
                .lines(&x, &y, &[Color("black")]);
            fg.show().unwrap();
        }
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}
