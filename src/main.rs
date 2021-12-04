mod visuals;
mod fractals;

fn main()
{
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        panic!("Insufficent arguments provided!")
    }

    match args[1].as_str().to_lowercase().trim() {
        "cantor" => fractals::cantor::CantorsSet::new().draw(),
        _ => panic!("No such fractal \"{}\"", args[1])
    }
}