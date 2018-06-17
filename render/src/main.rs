extern crate syntect;
#[macro_use]
extern crate structopt;
extern crate printpdf;

use structopt::StructOpt;
use std::path::{Path, PathBuf};

use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
//use syntect::util::as_24_bit_terminal_escaped;
use syntect::easy::HighlightFile;
use std::io::{BufRead, BufWriter};
use std::error::Error;
use std::fs::File;
use printpdf::*;


fn render(input_file_path: &Path, output_file_path: &Path) -> Result<(), Box<Error>> {
    let ss = SyntaxSet::load_defaults_nonewlines();
    let ts = ThemeSet::load_defaults();

    let mut highlighter = HighlightFile::new(input_file_path, &ss, &ts.themes["base16-ocean.dark"])?;

    let z = Mm(0.0);
    let w = Mm(210.0);
    let h = Mm(297.0);

    let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", w, h, "Layer 1");
    let font = doc.add_external_font(File::open("/usr/share/fonts/TTF/DejaVuSansMono-Bold.ttf").unwrap()).unwrap();
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font_size = 3;
    let bg_color = Color::Rgb(Rgb::new(
            0.0,
            0.0,
            0.0,
            None));

    let rect_points = vec![(Point::new(z, z), false),
                   (Point::new(z, h), false),
                   (Point::new(w, h), false),
                   (Point::new(w, z), false)];

    let background_rect = Line {
        points: rect_points,
        is_closed: true,
        has_fill: true,
        has_stroke: true,
        is_clipping_path: false,
    };
    current_layer.set_fill_color(bg_color);

    current_layer.add_shape(background_rect);

    current_layer.begin_text_section();
    current_layer.set_font(&font, font_size);
    current_layer.set_text_cursor(Mm(10.0), Mm(270.0));
    current_layer.set_line_height(font_size);
    current_layer.set_word_spacing(0);
    current_layer.set_character_spacing(0);
    current_layer.set_text_rendering_mode(TextRenderingMode::Fill);
    for maybe_line in highlighter.reader.lines() {
        let line = maybe_line?;
        let regions: Vec<(Style, &str)> = highlighter.highlight_lines.highlight(&line);
        //println!("{}", as_24_bit_terminal_escaped(&regions[..], true));
        for (style, text) in regions {
            let color = style.foreground;
            let color = Color::Rgb(Rgb::new(
                color.r as f64/255.0,
                color.g as f64/255.0,
                color.b as f64/255.0,
                None));
            current_layer.set_fill_color(color);
            current_layer.write_text(text, &font);
        }
        current_layer.add_line_break();
    }
    current_layer.end_text_section();

    doc.save(&mut BufWriter::new(File::create(output_file_path).unwrap())).unwrap();

    Ok(())
}

#[derive(StructOpt)]
#[structopt()]
struct Options {
    #[structopt(short="i", help="Path text file", parse(from_os_str))]
    input: PathBuf,

    #[structopt(short="o", help="Output file path", parse(from_os_str))]
    output: PathBuf,
}

fn main() -> Result<(), Box<Error>> {
    let options = Options::from_args();
    render(&options.input, &options.output)
}
