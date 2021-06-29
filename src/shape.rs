use itertools::multizip;
use std::{
    f64::consts::PI,
    hash::{Hash, Hasher},
    ops,
};

use crate::{Color, Error::*, Result};

/// The number of decimal places to use when comparing points.
const PRECISION: i32 = 6;

/// A generic interface of a polygon.
pub trait Polygon {
    /// Returns the polygon's points.
    fn points(&self, margin: f64) -> Result<Vec<Point>>;

    /// Renders the polygon.
    fn render(&self, context: &cairo::Context, margin: f64) -> Result<()>;
}

/// A representation of a regular polygon (all angles and sides are equal).
#[derive(Clone, Copy, Debug)]
pub struct Shape {
    sides: i32,
    point: Point,
    rotation: f64,
    fill: Color,
    stroke: Color,
}

impl Shape {
    /// Returns a new shape, ensuring the number of sides is at least three.
    pub fn new(sides: i32, fill: Color, stroke: Color) -> Result<Shape> {
        if sides < 3 {
            return Err(InvalidShape);
        }

        Ok(Shape {
            sides,
            point: Point::origin(),
            rotation: 0.0,
            fill,
            stroke,
        })
    }

    /// Returns the shape's sides.
    pub fn sides(&self) -> i32 {
        self.sides
    }

    /// Returns the shape's point.
    pub fn point(&self) -> Point {
        self.point
    }

    /// Returns the shape's rotation.
    pub fn rotation(&self) -> f64 {
        self.rotation
    }

    /// Returns the shape's fill.
    pub fn fill(&self) -> Color {
        self.fill
    }

    /// Returns the shape's stroke.
    pub fn stroke(&self) -> Color {
        self.stroke
    }

    /// Returns the the edge indexed by index.
    fn edge(&self, index: usize, margin: f64) -> Result<Edge> {
        let es = self.edges(margin)?;
        es.get(index)
            .ok_or(OutOfBounds {
                index: index,
                length: es.len(),
                name: String::from("shape edges"),
            })
            .map(|(p0, p1)| (p0.clone(), p1.clone()))
    }

    /// Returns the shape's edges.
    fn edges(&self, margin: f64) -> Result<Vec<Edge>> {
        let ps = self.points(margin)?;

        let mut es = Vec::new();
        for i in 0..self.sides {
            let i = i as usize;
            let p0 = ps.get(i).ok_or(OutOfBounds {
                index: i,
                length: ps.len(),
                name: String::from("shape vertices"),
            })?;
            let p1 = ps.get(i + 1).ok_or(OutOfBounds {
                index: i + 1,
                length: ps.len(),
                name: String::from("shape vertices"),
            })?;

            es.push((p0.clone(), p1.clone()));
        }

        Ok(es)
    }

    /// Returns the sides-sided shape adjacent to the edge with index edge.
    pub fn adjacent(&self, sides: i32, edge: usize, fill: Color, stroke: Color) -> Result<Shape> {
        let (p0, p1) = self.edge(edge, 0.0)?;
        let angle = 2.0 * PI / sides as f64;
        let a = (p1.y - p0.y).atan2(p1.x - p0.x);
        let b = a - PI / 2.0;
        let d = 0.5 / (angle / 2.0).tan();
        let p = Point {
            x: p0.x + (p1.x - p0.x) / 2.0 + b.cos() * d,
            y: p0.y + (p1.y - p0.y) / 2.0 + b.sin() * d,
        };
        let r = a + angle * ((sides - 1) as f64 / 2.0);

        Ok(Shape {
            sides: sides,
            point: p,
            rotation: r,
            fill: fill,
            stroke: stroke,
        })
    }

    /// Renders the index of each edge as an edge label.
    pub fn render_edge_labels(&self, context: &cairo::Context, margin: f64) -> Result<()> {
        let es = self.edges(margin)?;
        for (i, e) in es.iter().enumerate() {
            let text = i.to_string();
            let (p0, p1) = e;
            let te = context.text_extents(&text)?;
            let x = p0.x + (p1.x - p0.x) / 2.0 - te.width / 2.0;
            let y = p0.y + (p1.y - p0.y) / 2.0 - te.height / 2.0;

            context.set_source_rgb(0.0, 0.0, 0.0);
            context.move_to(x, y);
            context.show_text(&text)?;
        }

        Ok(())
    }

    /// Renders text as the shape's label.
    pub fn render_label(&self, context: &cairo::Context, text: &str) -> Result<()> {
        let te = context.text_extents(text)?;
        let x = self.point.x - te.width / 2.0;
        let y = self.point.y - te.height / 2.0;

        context.set_source_rgb(0.0, 0.0, 0.0);
        context.move_to(x, y);
        context.show_text(text)?;

        Ok(())
    }

    /// Returns a copy of the shape centered at point.
    pub fn clone_at(&self, point: Point) -> Shape {
        let mut s = self.clone();
        s.point = point;

        s
    }
}

impl Polygon for Shape {
    /// Returns the polygon's points.
    fn points(&self, margin: f64) -> Result<Vec<Point>> {
        let angle = 2.0 * PI / self.sides as f64;
        let rotation = self.rotation - PI / 2.0;
        let angles = (0..=self.sides)
            .map(|i| (i % self.sides) as f64 * angle + rotation)
            .collect::<Vec<f64>>();
        let d = {
            let a = angle / 2.0;
            0.5 / a.sin() - margin / a.cos()
        };

        let points = angles
            .iter()
            .map(|a| Point {
                x: self.point.x + a.cos() * d,
                y: self.point.y + a.sin() * d,
            })
            .collect();

        Ok(points)
    }

    /// Renders the polygon.
    fn render(&self, context: &cairo::Context, margin: f64) -> Result<()> {
        render(context, self.points(margin)?, self.fill, self.stroke)
    }
}

/// A representation of a polygon within a dual tiling.
#[derive(Clone, Debug)]
pub struct Dual {
    points: Vec<Point>,
    fill: Color,
    stroke: Color,
}

impl Dual {
    /// Returns a new dual with vertices points.
    pub fn new(points: Vec<Point>, fill: Color, stroke: Color) -> Dual {
        Dual {
            points,
            fill,
            stroke,
        }
    }

    /// Computes the inset polygon for a polygon with vertices points.
    fn inset_polygon(points: Vec<Point>, margin: f64) -> Result<Vec<Point>> {
        let p = points.get(points.len() - 2).ok_or(OutOfBounds {
            index: points.len() - 2,
            length: points.len(),
            name: String::from("shape points"),
        })?;
        let mut points = points.clone();
        points.insert(0, *p);

        let mut rs = multizip((&points, &points[1..], &points[2..]))
            .map(|(p0, p1, p2)| Dual::inset_corner((p0.clone(), p1.clone(), p2.clone()), margin))
            .collect::<Vec<Point>>();
        rs.push(rs[0]);

        Ok(rs)
    }

    /// Computes the inset corner for a tuple of three points.
    fn inset_corner(plane: Plane, margin: f64) -> Point {
        let (p0, p1, p2) = plane;
        let a0 = (p1.y - p0.y).atan2(p1.x - p0.x) - PI / 2.0;
        let a1 = (p2.y - p1.y).atan2(p2.x - p1.x) - PI / 2.0;
        let (ax0, ay0) = (p0.x + a0.cos() * margin, p0.y + a0.sin() * margin);
        let (ax1, ay1) = (p1.x + a0.cos() * margin, p1.y + a0.sin() * margin);
        let (bx0, by0) = (p1.x + a1.cos() * margin, p1.y + a1.sin() * margin);
        let (bx1, by1) = (p2.x + a1.cos() * margin, p2.y + a1.sin() * margin);
        let (ady, adx) = (ay1 - ay0, ax0 - ax1);
        let (bdy, bdx) = (by1 - by0, bx0 - bx1);
        let c0 = ady * ax0 + adx * ay0;
        let c1 = bdy * bx0 + bdx * by0;
        let d = ady * bdx - bdy * adx;

        Point {
            x: (bdx * c0 - adx * c1) / d,
            y: (ady * c1 - bdy * c0) / d,
        }
    }
}

impl Polygon for Dual {
    /// Returns the polygon's points.
    fn points(&self, margin: f64) -> Result<Vec<Point>> {
        if margin == 0.0 {
            Ok(self.points.clone())
        } else {
            Dual::inset_polygon(self.points.clone(), margin)
        }
    }

    /// Renders the polygon.
    fn render(&self, context: &cairo::Context, margin: f64) -> Result<()> {
        render(context, self.points(margin)?, self.fill, self.stroke)
    }
}

/// Represents a point in two-dimensional space.
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Returns a point at the origin.
    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    /// Returns a normalized point component.
    /// This is used to simplify equality tests and hashing.
    fn normalize(n: f64) -> i32 {
        (n * 10_f64.powi(PRECISION)).round() as i32
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        Point::normalize(self.x) == Point::normalize(other.x)
            && Point::normalize(self.y) == Point::normalize(other.y)
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Point::normalize(self.x).hash(state);
        Point::normalize(self.y).hash(state);
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

/// A representation of an edge in two-dimensional space.
type Edge = (Point, Point);

/// A representation of a plane in two-dimensional space.
type Plane = (Point, Point, Point);

/// Renders the polygon defined by points.
fn render(context: &cairo::Context, points: Vec<Point>, fill: Color, stroke: Color) -> Result<()> {
    for i in 0..points.len() {
        let p = points[i];
        match i {
            0 => context.move_to(p.x, p.y),
            _ => context.line_to(p.x, p.y),
        }
    }

    let (r, g, b) = fill.rgb_unit_int();
    context.set_source_rgb(r, g, b);
    context.fill_preserve()?;

    let (r, g, b) = stroke.rgb_unit_int();
    context.set_source_rgb(r, g, b);
    context.stroke()?;

    Ok(())
}
