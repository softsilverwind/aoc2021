mod d1;
mod d2;
mod d3;

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
        x => panic!("Unsolved problem {}!", x)
    }
}
