use std::{cmp::Ordering::Less, collections::HashMap, fs::File, ops::Range, path::Path};

use crate::{Color, Dual, Error::*, Point, Polygon, Result, Shape};

/// Represents a tiling composed of an arbitrary number of regular polygons.
/// A model is used to imperatively construct a tiling by building small patterns
/// of shapes that are then repeated to fill a two-dimensional space.
/// Use `render` to render the tiling.
/// Use `render_dual` to render the dual tiling.
#[derive(Debug)]
pub struct Model {
    width: i32,
    height: i32,
    scale: f64,
    shapes: Vec<Shape>,
    lookup: HashMap<Point, Shape>,
}

impl Model {
    /// Returns an empty model.
    pub fn new(width: i32, height: i32, scale: f64) -> Model {
        Model {
            width,
            height,
            scale,
            shapes: Vec::new(),
            lookup: HashMap::new(),
        }
    }

    /// Adds shape to the model.
    pub fn add(&mut self, shape: Shape) {
        self.shapes.push(shape);
        self.lookup.insert(shape.point(), shape);
    }

    /// Attaches shape to every edge in edges of each shape in indexes.
    pub fn add_multi(
        &mut self,
        indexes: Range<usize>,
        edges: Range<usize>,
        shape: Shape,
    ) -> Result<Range<usize>> {
        let start = self.shapes.len();
        for i in indexes {
            for e in edges.clone() {
                self.attach(i, e, shape)?;
            }
        }
        let end = self.shapes.len();

        Ok(start..end)
    }

    /// Attaches shape to the edge with index edge of the shape with index index.
    fn attach(&mut self, index: usize, edge: usize, shape: Shape) -> Result<()> {
        let parent = self.shapes.get(index).ok_or(OutOfBounds {
            index: index,
            length: self.shapes.len(),
            name: String::from("model shapes"),
        })?;
        let shape = parent.adjacent(shape.sides(), edge, shape.fill(), shape.stroke())?;
        self.add(shape);

        Ok(())
    }

    /// Fills the rest of the surface with the pattern contained by the shapes
    /// with index in indexes.
    pub fn repeat(&mut self, indexes: Range<usize>) -> Result<()> {
        let mut memo: HashMap<Point, i32> = HashMap::new();
        let mut depth = 0;

        loop {
            self.repeat_r(indexes.clone(), Point::origin(), depth, &mut memo)?;
            let w = self.width as f64 / 2.0 / self.scale;
            let h = self.height as f64 / 2.0 / self.scale;
            let tl = memo.keys().any(|p| p.x < -w && p.y < -h);
            let tr = memo.keys().any(|p| p.x > w && p.y < -h);
            let bl = memo.keys().any(|p| p.x < -w && p.y > h);
            let br = memo.keys().any(|p| p.x > w && p.y > h);
            if tl && tr && bl && br {
                break;
            }
            depth += 1;
        }

        Ok(())
    }

    /// Recurisvely fills the surface by repeating a pattern of shapes.
    fn repeat_r(
        &mut self,
        indexes: Range<usize>,
        point: Point,
        depth: i32,
        memo: &mut HashMap<Point, i32>,
    ) -> Result<()> {
        if depth < 0 {
            return Ok(());
        }

        let prev_depth = *memo.get(&point).unwrap_or(&-1);
        if prev_depth >= depth {
            return Ok(());
        }

        memo.insert(point, depth);

        if prev_depth == -1 {
            self.add_repeats(point);
        }

        let mut shapes = Vec::new();
        for i in indexes.clone() {
            let s = self.shapes.get(i).ok_or(OutOfBounds {
                index: i,
                length: self.shapes.len(),
                name: String::from("model shapes"),
            })?;

            shapes.push(*s);
        }

        for s in shapes.iter() {
            self.repeat_r(indexes.clone(), point + s.point(), depth - 1, memo)?;
        }

        Ok(())
    }

    /// Adds a shape to be repeated at point.
    fn add_repeats(&mut self, point: Point) {
        for s in self.shapes.iter() {
            let p = point + s.point();
            if self.lookup.contains_key(&p) {
                continue;
            }

            self.lookup.insert(p, s.clone_at(p));
        }
    }

    /// Returns the model's dual tiling.
    fn dual(&self, fill: Color, stroke: Color) -> Result<Vec<Dual>> {
        let mut vertexes: HashMap<Point, Vec<Shape>> = HashMap::new();
        for s in self.lookup.values() {
            let points = s.points(0.0)?;
            for p in &points[0..points.len() - 1] {
                if let Some(shapes) = vertexes.get_mut(p) {
                    shapes.push(*s);
                } else {
                    vertexes.insert(*p, vec![*s]);
                }
            }
        }

        let mut duals: Vec<Dual> = Vec::new();
        for (p, shapes) in vertexes.iter_mut() {
            if shapes.len() < 3 {
                continue;
            }

            let angle = |s: &Shape| (s.point().y - p.y).atan2(s.point().x - p.x);

            shapes.sort_by(|a, b| angle(b).partial_cmp(&angle(a)).unwrap_or(Less));

            let mut points = shapes.iter().map(|s| s.point()).collect::<Vec<Point>>();
            points.push(*points.first().ok_or(OutOfBounds {
                index: 0,
                length: shapes.len(),
                name: String::from("dual shapes"),
            })?);

            duals.push(Dual::new(points, fill, stroke));
        }

        Ok(duals)
    }

    /// Renders the model.
    pub fn render(
        &self,
        background: Color,
        margin: f64,
        line_width: f64,
        show_labels: bool,
    ) -> Result<Render> {
        let (surface, context) = self.render_init(background, line_width)?;
        let shapes = self.lookup.values();

        if show_labels {
            for s in shapes.clone() {
                s.render_edge_labels(&context, margin - 0.25)?;
            }
        }
        for s in shapes.clone() {
            s.render(&context, margin)?;
        }
        if show_labels {
            for (i, s) in shapes.clone().enumerate() {
                s.render_label(&context, &i.to_string())?;
            }
        }

        Ok(Render(surface))
    }

    /// Renders the model's dual tiling.
    pub fn render_dual(
        &self,
        background: Color,
        fill: Color,
        stroke: Color,
        margin: f64,
        line_width: f64,
    ) -> Result<Render> {
        let (surface, context) = self.render_init(background, line_width)?;
        let shapes = self.dual(fill, stroke)?;

        for s in shapes.clone() {
            s.render(&context, margin)?;
        }

        Ok(Render(surface))
    }

    /// Prepares a cairo surface and context for rendering.
    fn render_init(
        &self,
        background: Color,
        line_width: f64,
    ) -> Result<(cairo::ImageSurface, cairo::Context)> {
        let surface = cairo::ImageSurface::create(cairo::Format::Rgb24, self.width, self.height)?;
        let context = cairo::Context::new(&surface)?;
        let (red, green, blue) = background.rgb_unit_int();
        context.set_line_cap(cairo::LineCap::Round);
        context.set_line_join(cairo::LineJoin::Round);
        context.set_line_width(line_width);
        context.set_font_size(18.0 / self.scale);
        context.translate(self.width as f64 / 2.0, self.height as f64 / 2.0);
        context.scale(self.scale, self.scale);
        context.set_source_rgb(red, green, blue);
        context.paint()?;

        Ok((surface, context))
    }
}

/// Represents a rendered model.
pub struct Render(cairo::ImageSurface);

impl Render {
    /// Writes a rendered model to a PNG file at path.
    pub fn write_to_png<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut file = File::create(path)?;
        self.0.write_to_png(&mut file)?;

        Ok(())
    }
}
