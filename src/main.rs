use structopt::StructOpt;

mod day_01;
mod day_02;
mod day_03;

#[derive(Debug, StructOpt)]
struct Cli {
    day: String,
}

fn main() {
    let args = Cli::from_args();
    match args.day.as_str() {
        "01" => {
            day_01::first_solution();
            day_01::second_solution();
        }
        "02" => {
            day_02::first_solution();
            day_02::second_solution();
        }
        "03" => {
            day_03::first_solution();
            day_03::second_solution();
        }
        _ => println!("Not yet implemented"),
    };
}
