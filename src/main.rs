use structopt::StructOpt;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;

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
        "04" => {
            day_04::first_solution();
            day_04::second_solution();
        }
        "05" => {
            day_05::first_solution();
            day_05::second_solution();
        }
        "06" => {
            day_06::first_solution();
            day_06::second_solution();
        }
        "07" => {
            day_07::first_solution();
            day_07::second_solution();
        }
        "08" => {
            day_08::first_solution();
            day_08::second_solution();
        }
        "09" => {
            day_09::first_solution();
            day_09::second_solution();
        }
        "10" => {
            day_10::first_solution();
            day_10::second_solution();
        }
        _ => println!("Not yet implemented"),
    };
}
