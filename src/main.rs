use std::time::Instant;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod util;

fn main() {
    aoc_result!(01, 1, d01::get_solution_1());
    aoc_result!(01, 2, d01::get_solution_2());

    aoc_result!(02, 1, d02::get_solution_1());
    aoc_result!(02, 2, d02::get_solution_2());

    aoc_result!(03, 1, d03::get_solution_1());
    aoc_result!(03, 2, d03::get_solution_2());

    aoc_result!(04, 1, d04::get_solution_1());
    aoc_result!(04, 2, d04::get_solution_2());

    aoc_result!(05, 1, d05::get_solution_1());
    aoc_result!(05, 2, d05::get_solution_2());

    aoc_result!(06, 1, d06::get_solution_1());
    aoc_result!(06, 2, d06::get_solution_2());

    aoc_result!(07, 1, d07::get_solution_1());
    aoc_result!(07, 2, d07::get_solution_2());

    aoc_result!(08, 1, d08::get_solution_1());
    aoc_result!(08, 2, d08::get_solution_2());

    aoc_result!(09, 1, d09::get_solution_1());
    aoc_result!(09, 2, d09::get_solution_2());

    aoc_result!(10, 1, d10::get_solution_1());
    aoc_result!(10, 2, d10::get_solution_2());

    aoc_result!(11, 1, d11::get_solution_1());
    aoc_result!(11, 2, d11::get_solution_2());

    aoc_result!(12, 1, d12::get_solution_1());
    aoc_result!(12, 2, d12::get_solution_2());

    aoc_result!(13, 1, d13::get_solution_1());
    aoc_result!(13, 2, d13::get_solution_2());

    aoc_result!(14, 1, d14::get_solution_1());
    aoc_result!(14, 2, d14::get_solution_2());

    aoc_result!(15, 1, d15::get_solution_1());
    aoc_result!(15, 2, d15::get_solution_2());
}

#[macro_export]
macro_rules! aoc_result {
    ( $d:literal, $p:literal, $r:expr ) => {
        let now = Instant::now();
        println!(
            "d{:2}.{}: {:16}\t{:10}us",
            $d,
            $p,
            $r,
            now.elapsed().as_micros()
        );
    };
}
