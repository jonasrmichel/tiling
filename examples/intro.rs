use tiling::{Color, Model, Result, Shape};

pub fn main() -> Result<()> {
    let width = 1024;
    let height = 1024;
    let scale = 128.0;
    let stroke = Color::new(242, 60, 60)?;
    let fill_hexagon = Color::new(242, 194, 106)?;
    let fill_square = Color::new(23, 216, 146)?;
    let fill_triangle = Color::new(242, 209, 48)?;
    let background = Color::new(242, 242, 242)?;
    let margin = 0.1;
    let show_labels = false;
    let line_width = 0.1;

    // create an empty model
    let mut model = Model::new(width, height, scale);

    // add a hexagon
    model.add(Shape::new(6, fill_hexagon, stroke)?);

    // attach a square to each side of the hexagon
    let squares = model.add_multi(0..1, 0..6, Shape::new(4, fill_square, stroke)?)?;

    // attach a triangle between the squares
    let _ = model.add_multi(squares.clone(), 1..2, Shape::new(3, fill_triangle, stroke)?)?;

    // attach a hexagon to the outer edge of each square
    let hexagons = model.add_multi(squares.clone(), 2..3, Shape::new(6, fill_hexagon, stroke)?)?;

    // fill the surface with the pattern
    model.repeat(hexagons)?;

    // render the tiling
    let render = model.render(background, margin, line_width, show_labels)?;
    render.write_to_png("intro.png")?;

    // render the dual tiling
    let render_dual = model.render_dual(background, fill_hexagon, stroke, margin, line_width)?;
    render_dual.write_to_png("intro-dual.png")?;

    Ok(())
}
