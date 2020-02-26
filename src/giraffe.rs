use clap::{App, Arg};

pub fn data() -> clap::ArgMatches<'static> {
    let matches = App::new("giraffe")
        .version("0.1")
        .author("Sukyoung Jeong <suki.jeong@samsung.com>")
        .about("is a graph tool")
        .arg(
            Arg::with_name("datafile")
                .help("Data csv file to draw graph")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("x_value")
                .help("# of x value in the csv")
                .short("x")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("y_value")
                .help("# of x value in the csv")
                .short("y")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("title")
                .help("Title of the graph")
                .long("title")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("x_label")
                .help("X-axis label")
                .long("x_label")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("y_label")
                .help("Y-axis label")
                .long("y_label")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    return matches;
}
