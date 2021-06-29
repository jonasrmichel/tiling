use tiling::{Color, Model, Result, Shape};

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 1024;
const SCALE: f64 = 128.0;
const MARGIN: f64 = 0.1;
const SHOW_LABELS: bool = false;
const LINE_WIDTH: f64 = 0.1;

pub fn main() -> Result<()> {
    let blue = Color::new(56, 103, 165)?;
    let yellow = Color::new(242, 205, 21)?;
    let gold = Color::new(242, 174, 45)?;
    let orange = Color::new(216, 140, 73)?;
    let burnt = Color::new(191, 86, 47)?;

    Ex3636::render(blue, yellow, gold, orange, burnt)?;
    Ex33434::render(blue, yellow, gold, orange, burnt)?;
    Ex33336::render(blue, yellow, gold, orange, burnt)?;
    Ex333333::render(blue, yellow, gold, orange, burnt)?;

    Ok(())
}

trait Example {
    fn render(
        background: Color,
        stroke: Color,
        fill_0: Color,
        fill_1: Color,
        fill_2: Color,
    ) -> Result<()>;
}

struct Ex3636();

impl Example for Ex3636 {
    fn render(
        background: Color,
        stroke: Color,
        fill_0: Color,
        fill_1: Color,
        fill_2: Color,
    ) -> Result<()> {
        let mut model = Model::new(WIDTH, HEIGHT, SCALE);
        model.add(Shape::new(6, fill_1, stroke)?);
        let a = model.add_multi(0..1, 0..6, Shape::new(3, fill_0, stroke)?)?;
        let b = model.add_multi(a, 1..2, Shape::new(6, fill_1, stroke)?)?;
        model.repeat(b)?;

        model
            .render(background, MARGIN, LINE_WIDTH, SHOW_LABELS)?
            .write_to_png("3.6.3.6.png")?;
        model
            .render_dual(background, fill_0, stroke, MARGIN, LINE_WIDTH)?
            .write_to_png("3.6.3.6-dual.png")?;

        Ok(())
    }
}

struct Ex33434();

impl Example for Ex33434 {
    fn render(
        background: Color,
        stroke: Color,
        fill_0: Color,
        fill_1: Color,
        fill_2: Color,
    ) -> Result<()> {
        let mut model = Model::new(WIDTH, HEIGHT, SCALE);
        model.add(Shape::new(4, fill_1, stroke)?);
        let a = model.add_multi(0..1, 0..4, Shape::new(3, fill_2, stroke)?)?;
        let b = model.add_multi(a, 1..2, Shape::new(4, fill_1, stroke)?)?;
        let c = model.add_multi(b, 2..4, Shape::new(3, fill_2, stroke)?)?;
        let d = model.add_multi(c, 2..3, Shape::new(4, fill_1, stroke)?)?;
        model.repeat(d)?;

        model
            .render(background, MARGIN, LINE_WIDTH, SHOW_LABELS)?
            .write_to_png("3.3.4.3.4.png")?;
        model
            .render_dual(background, fill_0, stroke, MARGIN, LINE_WIDTH)?
            .write_to_png("3.3.4.3.4-dual.png")?;

        Ok(())
    }
}

struct Ex33336();

impl Example for Ex33336 {
    fn render(
        background: Color,
        stroke: Color,
        fill_0: Color,
        fill_1: Color,
        fill_2: Color,
    ) -> Result<()> {
        let mut model = Model::new(WIDTH, HEIGHT, SCALE);
        model.add(Shape::new(6, fill_2, stroke)?);
        let a = model.add_multi(0..1, 0..6, Shape::new(3, fill_0, stroke)?)?;
        let b = model.add_multi(a.clone(), 1..2, Shape::new(3, fill_0, stroke)?)?;
        let c = model.add_multi(a.clone(), 2..3, Shape::new(3, fill_0, stroke)?)?;
        let d = model.add_multi(c, 1..2, Shape::new(6, fill_2, stroke)?)?;
        model.repeat(d)?;

        model
            .render(background, MARGIN, LINE_WIDTH, SHOW_LABELS)?
            .write_to_png("3.3.3.3.6.png")?;
        model
            .render_dual(background, fill_0, stroke, MARGIN, LINE_WIDTH)?
            .write_to_png("3.3.3.3.6-dual.png")?;

        Ok(())
    }
}

struct Ex333333();

impl Example for Ex333333 {
    fn render(
        background: Color,
        stroke: Color,
        fill_0: Color,
        fill_1: Color,
        fill_2: Color,
    ) -> Result<()> {
        let mut model = Model::new(WIDTH, HEIGHT, SCALE);
        model.add(Shape::new(3, fill_2, stroke)?);
        let a = model.add_multi(0..1, 0..3, Shape::new(3, fill_1, stroke)?)?;
        let b = model.add_multi(a, 1..3, Shape::new(3, fill_2, stroke)?)?;
        model.repeat(b)?;

        model
            .render(background, MARGIN, LINE_WIDTH, SHOW_LABELS)?
            .write_to_png("3.3.3.3.3.3.png")?;
        model
            .render_dual(background, fill_0, stroke, MARGIN, LINE_WIDTH)?
            .write_to_png("3.3.3.3.3.3-dual.png")?;

        Ok(())
    }
}
