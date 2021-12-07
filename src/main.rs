mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;

fn main()
{
    let arg = std::env::args().skip(1).next().expect("Missing problem argument");
    match arg.as_ref() {
        "1a" => d1::simple(),
        "1b" => d1::complex(),
        "2a" => d2::simple(),
        "2b" => d2::complex(),
        "3a" => d3::simple(),
        "3b" => d3::complex(),
        "4a" => d4::simple(),
        "4b" => d4::complex(),
        "5a" => d5::simple(),
        "5b" => d5::complex(),
        "6a" => d6::simple(),
        "6b" => d6::complex(),
        x => panic!("Unsolved problem {}!", x)
    }
}
