//! # tiling
//!
//! *tiling* is a library for constructing tilings of regular polygons and their
//! dual tilings.
//!
//! # Resources
//!
//! - [Tilings by regular polygons](http://en.wikipedia.org/wiki/Tiling_by_regular_polygons)
//! - [List of Euclidian uniform tilings](https://en.wikipedia.org/wiki/List_of_Euclidean_uniform_tilings)
//!
//! # Requirements
//!
//! *tiling* uses [cairo-rs](https://crates.io/crates/cairo-rs) for rendering and
//! requires [cairo](https://www.cairographics.org/download/) to be installed.
//!
//! # Example
//!
//! Create an empty tiling model.
//!
//! ```rust
//! let (width, height, scale) = (1024, 1024, 128.0);
//!
//! let mut model = Model::new(width, height, scale);
//! ```
//!
//! Place a polygon at the origin.
//! This adds a hexagon.
//!
//! ```rust
//! let stroke = Color::new(242, 60, 60)?;
//! let fill_hexagon = Color::new(242, 194, 106)?;
//!
//! model.add(Shape::new(6, fill_hexagon, stroke)?);
//! ```
//!
//! At this point we can render the model.
//!
//! ```rust
//! let background = Color::new(242, 242, 242)?;
//! let margin = 0.1;
//! let show_labels = false;
//! let line_width = 0.1;
//!
//! let render = model.render(background, margin, line_width, show_labels)?;
//! render.write_to_png("output.png")?;
//! ```
//!
//! Let's continue by attaching a square to each of the hexagon's sides.
//!
//! ```rust
//! let fill_square = Color::new(23, 216, 146)?;
//!
//! let squares = model.add_multi(0..1, 0..6, Shape::new(4, fill_square, stroke)?)?;
//! ```
//!
//! The first parameter `0..1` is a range that indicates the shape(s) to attach to
//! (by their index).
//! In this example, the square is attached to the hexagon (index `0`).
//!
//! > When `show_labels` is `true`, each shape is labeled with its index.
//!
//! The second paramter `0..6` is a range that indicates the edge(s) to attach to
//! (by their index).
//! In this example, the square is attached to all six edges of the hexagon.
//!
//! > When `show_labels` is `true`, each edge is labeled with its index.
//!
//! The final paramter defines the shape to add (a square).
//!
//! The `add_multi` method returns a range containing the indexes of the added square
//! shapes so they can be referenced later.
//! We'll see how to do that next.
//!
//! Now, attach triangles to all of the squares using the previously returned range
//! `squares`.
//! Here, a triangle is attached to edge `1` of each square.
//!
//! ```rust
//! let fill_triangle = Color::new(242, 209, 48)?;
//!
//! let _ = model.add_multi(squares.clone(), 1..2, Shape::new(3, fill_triangle, stroke)?)?;
//! ```
//!
//! Let's wrap up by attaching a hexagon to the outer edge of each square.
//! These hexagons will define the repeating positions of the tiling.
//!
//! ```rust
//! let hexagons = model.add_multi(squares.clone(), 2..3, Shape::new(6, fill_hexagon, stroke)?)?;
//! ```
//!
//! Now that the tiling's repeating pattern is complete, use the `repeat` method to
//! fill the rest of the surface with the pattern.
//!
//! ```rust
//! model.repeat(hexagons)?;
//! ```
//!
//! Once satisfied, disable the shape and edge labels and adjust the scale.
//!
//! Dual tilings may be created using the `render_dual` method.
//! A tiling's dual is formed by drawing edges between the centers of adjacent polygons.
pub use color::Color;
pub use error::{Error, Result};
pub use model::Model;
pub use shape::{Dual, Point, Polygon, Shape};

pub mod color;
pub mod error;
pub mod model;
pub mod shape;
