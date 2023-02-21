use clap::{Command, Arg};
use esvg::{page::Page, path, shapes};
use polygonical::point::Point;

fn main() {
    let matches = Command::new("mandala")
        .arg(
            Arg::new("output")
                .required(true)
                .short('o')
                .long("output")
                .default_value("./out.svg")
                .help("where to save the result"),
        )
        .arg(
            Arg::new("paper")
                .required(false)
                .short('p')
                .long("paper")
                .default_value("A4")
                .help("What size paper to use"),
        )
        .arg(
            Arg::new("rings")
                .required(true)
                .short('r')
                .long("rings")
                .default_value("5")
                .help("the number of rings to create"),
        )
        .arg(
            Arg::new("lines")
                .required(true)
                .short('l')
                .long("lines")
                .default_value("10")
                .help("the number of lines to include"),
        )
        .arg(
            Arg::new("center")
                .required(false)
                .short('c')
                .long("center")
                .default_value("center")
                .help("where to put the center, center|edge|corner"),
        )
        .get_matches();

    let paper_name = matches.value_of("paper").unwrap();
    // default to 96dpi and half inch borders
    // TODO: allow customisation of dpi and margin values
    let paper = esvg::page::Page::build_page(paper_name, 96, 0.5).unwrap();

    let rings = matches.value_of("rings").unwrap().parse::<u32>().unwrap();
    let lines = matches.value_of("lines").unwrap().parse::<u32>().unwrap();
    let center = matches.value_of("center").unwrap();

    let result = generate(&paper, rings, lines, center);

    esvg::save(matches.value_of("output").unwrap(), &result).unwrap();
}



fn generate(page: &Page, rings : u32, lines: u32, center: &str) -> esvg::Element {

    let mut doc = create_document(page);

    let (center, start_angle, end_angle, radius, circle) = match center {
        "edge" => (
            page.center_left(), 
            -90.0_f64.to_radians(), 
            90.0_f64.to_radians(), 
            page.display_width_px().min(page.display_height_px()/2), // which ever is shorter the distance to the other side of the page or the top of the page
            false
        ),
        "corner" => (
            page.top_left(), 
            0.0_f64.to_radians(), 
            90.0_f64.to_radians(), 
            page.display_width_px().min(page.display_height_px()), // which ever is shorter the width or height of the page
            false
        ),
        _ => (
            page.center(), 
            0.0_f64.to_radians(), 
            360.0_f64.to_radians(), 
            page.display_width_px().min(page.display_height_px()) / 2, // half of which ever is shorter, the width or height of the page
            true
        ),
    };

    let rotation_range = end_angle - start_angle;

    for i in 0..lines+1 {
        let angle = ((rotation_range / lines as f64) * i as f64) + start_angle;
        let end_point = Point::new(radius, 0).rotate(angle).translate(&center);

        let l = path::Data::new()
            .move_to(center)
            .line_to(end_point)
            .to_path();

        doc.add(&l);
    }

    for i in 0..(rings+1) {
        let r = ((radius as f64 / rings as f64) * i as f64).round() as i32 ;
        if circle {
            let c = shapes::circle(center, r);
            doc.add(&c);
        } else {
            let start_point = Point::new(r, 0).rotate(start_angle).translate(&center);
            let end_point = Point::new(r, 0).rotate(end_angle).translate(&center);
            let l = path::Data::new()
                .move_to(start_point)
                .arc_to(end_point, r, r, 0.0, false, true)
                .to_path();

            doc.add(&l);
        }
    }

    doc

}


fn create_document(page: &Page) -> esvg::Element {
    let mut doc = esvg::create_document(page);
    doc.set("stroke", "black");
    doc.set("fill", "black");
    doc.set("fill-opacity", "1");
    doc.set("stroke-opacity", "1");
    doc.set("stroke-width", "1mm");
    doc.set("stroke-linejoin", "miter");
    doc.set("stroke-dasharray", "none");
    doc.set("stroke-linecap", "square");
    doc.set("stroke-miterlimit", "10");
    doc.set("stroke-dashoffset", "0");
    doc.set("color-rendering", "auto");
    doc.set("color-interpolation", "auto");
    doc.set("text-rendering", "auto");
    doc.set("shape-rendering", "auto");
    doc.set("image-rendering", "auto");
    doc.set("font-weight", "normal");
    doc.set("font-family", "Dialog");
    doc.set("font-style", "normal");
    doc.set("font-size", "12");

    let mut generic_defs = esvg::Element::new("defs");
    generic_defs.set("id", "genericDefs");

    doc.add(&generic_defs);

    doc
}