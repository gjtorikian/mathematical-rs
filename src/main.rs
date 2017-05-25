extern crate mathematical;

use mathematical::mathml_to_svg;

#[macro_use]
extern crate clap;

use std::io::Read;
use std::process;

fn main() {
    let matches = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(clap::Arg::with_name("input")
            .value_name("INPUT")
            .help("The MathML file to parse, or standard input if none is passed"))
        .arg(clap::Arg::with_name("ppi")
            .long("ppi")
            .takes_value(true)
            .value_name("PPI")
            .default_value("200")
            .help("Sets the PPI quality [default: 200]."))
        .get_matches();

    let ppi:f32 = matches
        .value_of("ppi")
        .unwrap_or("200")
        .parse()
        .unwrap_or(200.0);

    assert_gt_zero("ppi", ppi);

    let mut mathml = String::with_capacity(2048);

    match matches.values_of("input") {
        None => {
            std::io::stdin().read_to_string(&mut mathml).unwrap();
        }
        Some(fs) => {
            for f in fs {
                let mut io = std::fs::File::open(f).unwrap();
                io.read_to_string(&mut mathml).unwrap();
            }
        }
    };

    print!("{}", mathml_to_svg(mathml.as_str()));

    process::exit(0);
}

fn assert_gt_zero(name: &'static str, val: f32) {
    if val <= 0.0 {
        panic!("{} must be greater than zero!", name);
    }
}
