mod visuals;
mod fractals;
mod math;

fn main()
{
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        panic!("Insufficent arguments provided!")
    }

    match args[1].as_str().to_lowercase().trim() {
        "cantor" => fractals::cantor::CantorsSet::new().draw(),
        "circle_line" => fractals::circle_line::CircleLine::new().draw(),
        "circle_space" => fractals::circle_space::CircleSpace::new().draw(),
        "koch" => fractals::koch::Koch::new().draw(),
        _ => panic!("No such fractal \"{}\"", args[1])
    }

}

#[cfg(test)]
pub mod tests {
    use super::math::{matrix::Matrix2, vector::Vector2};
    use std::f32::consts::PI;
    #[test]
    fn matrix_operations() {
        let rot = Matrix2::rotation(-PI / 2.0);
        let rotinv = rot.inverse().unwrap();

        assert_eq!(rotinv * rot, Matrix2::identity())
    }

    #[test]
    fn vector_operations() {
        let e1 = Vector2::new(1.0, 0.0);
        let e2 = Vector2::new(0.0, 1.0);
        let rot = Matrix2::rotation(PI / 3.0);
        let rotinv = rot.inverse().unwrap();

        assert_eq!(Vector2::new(1.0, 1.0), e1 + e2);
        assert_eq!(Vector2::new(2.0, 1.0), 2.0 * e1 + e2);
        assert_eq!(Vector2::new(1.0, -1.0), e1 - e2);
        assert_eq!(rot * rotinv * e1, e1);
    }
}