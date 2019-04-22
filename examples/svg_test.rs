extern crate clap;
extern crate svg;
extern crate xbar;

use clap::{App, Arg};
use svg::node::element::path::Data;
use svg::node::{element::Path, element::Text as TextBox, Text};
use svg::Document;
use xbar::{Connection, Crossbar};

static BLOCK_W: usize = 20;
static BLOCK_H: usize = 20;
static MARGIN_X: usize = 40;
static MARGIN_Y: usize = 40;
static LEFT_TEXT_PAD: usize = 30;

pub fn main() {
    let matches = App::new("xbar")
        .version("1.0")
        .author("kubuzetto <dvr.sahin@gmail.com>")
        .about("Renders a crossbar switch with N terminals")
        .arg(
            Arg::with_name("num_terms")
                .help("Number of terminals")
                .value_name("count")
                .long("num_terms")
                .takes_value(true)
                .required(true)
                .short("n"),
        )
        .arg(
            Arg::with_name("output")
                .help("Output SVG file path")
                .value_name("filePath")
                .takes_value(true)
                .long("output")
                .required(true)
                .short("o"),
        )
        .get_matches();
    let num = matches
        .value_of("num_terms")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let path = matches.value_of("output").unwrap();
    render(path, num);
    println!(
        "Crossbar switch with {} terminals was printed to the SVG file {}.",
        num, path
    );
}

fn render(file: &str, n: usize) {
    let w = Crossbar::columns(n) * BLOCK_W + BLOCK_W + 2 * MARGIN_X + LEFT_TEXT_PAD;
    let h = (Crossbar::rows(n) + Crossbar::blocks(n) - 1) * BLOCK_H + 2 * MARGIN_Y;
    let mut doc = Document::new().set("viewBox", (0, 0, w, h)).add(
        Path::new()
            .set("fill", "#ffffff")
            .set("stroke", "#444444")
            .set("stroke-width", 2)
            .set(
                "d",
                Data::new()
                    .move_to((0, 0))
                    .line_to((w, 0))
                    .line_to((w, h))
                    .line_to((0, h))
                    .close(),
            ),
    );
    for val in Crossbar::new(n) {
        doc = render_one(doc, val, n);
    }
    svg::save(file, &doc).unwrap();
}

fn render_one(doc: Document, val: Connection, n: usize) -> Document {
    let l0 = MARGIN_X + LEFT_TEXT_PAD;
    let l1 = l0 + (1 + val.col_idx) * BLOCK_W;
    let t0 = MARGIN_Y + BLOCK_H * (val.start.block_idx * (n + 1) + val.start.row_idx);
    let t1 = MARGIN_Y + BLOCK_H * (val.end.block_idx * (n + 1) + val.end.row_idx);
    doc.add(
        Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 2)
            .set(
                "d",
                Data::new()
                    .move_to((l0, t0))
                    .line_to((l1, t0))
                    .line_to((l1, t1))
                    .line_to((l0, t1)),
            ),
    )
    .add(
        TextBox::new()
            .set("y", t0 + BLOCK_H / 4)
            .set("x", MARGIN_X)
            .set(
                "style",
                format!(
                    "font-size:{}px; \
                     font-family:sans-serif; \
                     fill:#000000; \
                     fill-opacity:1; \
                     stroke:none;",
                    BLOCK_H
                ),
            )
            .add(Text::new(format!("{}", val.start.row_idx))),
    )
    .add(
        TextBox::new()
            .set("y", t1 + BLOCK_H / 4)
            .set("x", MARGIN_X)
            .set(
                "style",
                format!(
                    "font-size:{}px; \
                     font-family:sans-serif; \
                     fill:#000000; \
                     fill-opacity:1; \
                     stroke:none;",
                    BLOCK_H
                ),
            )
            .add(Text::new(format!("{}", val.end.row_idx))),
    )
}
