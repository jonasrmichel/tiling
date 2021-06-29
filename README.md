# tiling

[![Crates.io](https://img.shields.io/crates/v/tiling?style=flat-square)](https://crates.io/crates/tiling)
[![Crates.io](https://img.shields.io/crates/d/tiling?style=flat-square)](https://crates.io/crates/tiling)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/jonasrmichel/tiling/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/jonasrmichel/tiling/blob/main/LICENSE-MIT)

*tiling* is a library for constructing tilings of regular polygons and their
dual tilings.

<img src="https://github.com/jonasrmichel/tiling/raw/main/assets/tiling.png" alt="tiling" width="128">

# Resources

- [Documentation](https://docs.rs/tiling)
- [Tilings by regular polygons](http://en.wikipedia.org/wiki/Tiling_by_regular_polygons)
- [List of Euclidian uniform tilings](https://en.wikipedia.org/wiki/List_of_Euclidean_uniform_tilings)

# Examples

Here are some tilings produced by the examples in the [`examples`](./examples) directory.

<img src="https://github.com/jonasrmichel/tiling/raw/main/assets/examples.gif" alt="examples" width="1024">

# Requirements

*tiling* uses [cairo-rs](https://crates.io/crates/cairo-rs) for rendering and 
requires [cairo](https://www.cairographics.org/download/) to be installed.

# Usage

Create an empty tiling model.

```rust
let (width, height, scale) = (1024, 1024, 128.0);

let mut model = Model::new(width, height, scale);
```

Place a polygon at the origin.
This adds a hexagon.

```rust
let stroke = Color::new(242, 60, 60)?;
let fill_hexagon = Color::new(242, 194, 106)?;

model.add(Shape::new(6, fill_hexagon, stroke)?);
```

At this point we can render the model.

```rust
let background = Color::new(242, 242, 242)?;
let margin = 0.1;
let show_labels = false;
let line_width = 0.1;

let render = model.render(background, margin, line_width, show_labels)?;
render.write_to_png("output.png")?;
```

<img src="https://github.com/jonasrmichel/tiling/raw/main/assets/intro-0.png" alt="hexagon" width="1024">

Let's continue by attaching a square to each of the hexagon's sides.

```rust
let fill_square = Color::new(23, 216, 146)?;

let squares = model.add_multi(0..1, 0..6, Shape::new(4, fill_square, stroke)?)?;
```

The first parameter `0..1` is a range that indicates the shape(s) to attach to
(by their index).
In this example, the square is attached to the hexagon (index `0`).

> When `show_labels` is `true`, each shape is labeled with its index.

The second paramter `0..6` is a range that indicates the edge(s) to attach to
(by their index).
In this example, the square is attached to all six edges of the hexagon.

> When `show_labels` is `true`, each edge is labeled with its index.

The final paramter defines the shape to add (a square).

The `add_multi` method returns a range containing the indexes of the added square
shapes so they can be referenced later.
We'll see how to do that next.

<img src="https://github.com/jonasrmichel/tiling/raw/main/assets/intro-1.png" alt="hexagon squares" width="1024">

Now, attach triangles to all of the squares using the previously returned range 
`squares`.
Here, a triangle is attached to edge `1` of each square.

```rust
let fill_triangle = Color::new(242, 209, 48)?;

let _ = model.add_multi(squares.clone(), 1..2, Shape::new(3, fill_triangle, stroke)?)?;
```

<img src="https://github.com/jonasrmichel/tiling/raw/main/assets/intro-2.png" alt="hexagon squares triangles" width="1024">

Let's wrap up by attaching a hexagon to the outer edge of each square. 
These hexagons will define the repeating positions of the tiling.

```rust
let hexagons = model.add_multi(squares.clone(), 2..3, Shape::new(6, fill_hexagon, stroke)?)?;
```

<img src="https://github.com/jonasrmichel/tiling/raw/main/assets/intro-3.png" alt="hexagon squares triangles hexagons" width="1024">

Now that the tiling's repeating pattern is complete, use the `repeat` method to
fill the rest of the surface with the pattern.

```rust
model.repeat(hexagons)?;
```

<img src="https://github.com/jonasrmichel/tiling/raw/main/assets/intro-4.png" alt="hexagon squares triangles hexagons repeated" width="1024">

Once satisfied, disable the shape and edge labels and adjust the scale.

The complete code for this example is in [`examples/intro.rs`](./examples/intro.rs).

Dual tilings may be created using the `render_dual` method.
A tiling's dual is formed by drawing edges between the centers of adjacent polygons.

Here is the dual tiling of the above example.

<img src="https://github.com/jonasrmichel/tiling/raw/main/assets/intro-5.png" alt="hexagon squares triangles hexagons dual tiling" width="1024">

# Installation

*tiling* is available on [crates.io](https://crates.io/crates/tiling) and can be
included in your Cargo enabled project.

```toml
[dependencies]
tiling = "0.1.0"
```

# Future improvements

- Declarative API
- Reduce memory usage by reusing `Shape` references
- Generate tiling models by interpreting their [*vertex configuration*](https://en.wikipedia.org/wiki/List_of_Euclidean_uniform_tilings)
- Command line tool
- Support shape and edge attachment via disjoint ranges
- Support different image output types

# Acknowledgements

This library was inspired by Michael Fogleman's [Tiling](https://github.com/fogleman/Tiling)
Python tool.
Several of the low-level geometric computations in this crate are based on
implementations found in Tiling.