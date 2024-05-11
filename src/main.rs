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

fn main() {
    println!("d01.1: {}", d01::get_solution_1());
    println!("d01.2: {}", d01::get_solution_2());

    println!("d02.1: {}", d02::get_solution_1());
    println!("d02.2: {}", d02::get_solution_2());

    println!("d03.1: {}", d03::get_solution_1());
    println!("d03.2: {}", d03::get_solution_2());

    println!("d04.1: {}", d04::get_solution_1());
    println!("d04.2: {}", d04::get_solution_2());

    println!("d05.1: {}", d05::get_solution_1());
    println!("d05.2: {}", d05::get_solution_2());

    println!("d06.1: {}", d06::get_solution_1());
    println!("d06.2: {}", d06::get_solution_2());

    println!("d07.1: {}", d07::get_solution_1());
    println!("d07.2: {}", d07::get_solution_2());

    println!("d08.1: {}", d08::get_solution_1());
    println!("d08.2: {}", d08::get_solution_2());

    println!("d09.1: {}", d09::get_solution_1());
    println!("d09.2: {}", d09::get_solution_2());

    println!("d10.1: {}", d10::get_solution_1());
    println!("d10.2: {}", d10::get_solution_2());

    println!("d11.1: {}", d11::get_solution_1());
    println!("d11.2: {}", d11::get_solution_2());

    let mut now = Instant::now();
    println!("d12.1: {}\t {}us", d12::get_solution_1(), now.elapsed().as_micros());
    now = Instant::now();
    println!("d12.2: {}\t {}us", d12::get_solution_2(), now.elapsed().as_micros());
}
