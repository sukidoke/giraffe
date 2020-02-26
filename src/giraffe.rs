use clap::{App, Arg};

pub fn data() -> clap::ArgMatches<'static> {
    let matches = App::new("giraffe")
        .version("0.1")
        .author("Sukyoung Jeong <suki.jeong@samsung.com>")
        .about("is a graph tool")

        .arg(Arg::with_name("datafile")
             .help("Data file to draw graph")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("x_value")
             .short("x")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("y_value")
             .short("y")
             .takes_value(true)
             .required(true))
        .get_matches();

    return matches;
}
