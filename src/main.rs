use std::array;
use std::process::exit;


type greyscale = Vec<Vec<u8>>;


enum Channels { Three = 3, Four = 4 }
enum Colorspace { Zero = 0, One = 1 }

struct QoiHeader {
    magic: [u8; 4],
    width: usize,
    height: usize,
    channels: Channels,
    colorspace: Colorspace,
}

#[derive(Copy, Clone)]
struct Pixel { r: u8, g: u8, b: u8, a: u8 }

fn generate_sample_image(header: &QoiHeader) -> greyscale {
    let mut im: greyscale = vec![];
    for i in 0..header.height {
        im.push(vec![]);
        for j in 0..header.width {
            im[i].push((j % 255) as u8);
        }
    }

    return im;
}

fn hash(pixel: &Pixel) -> u8 {
    ((3 * pixel.r as u16 + 5 * pixel.g as u16 + 7 * pixel.b as u16 + 11 * pixel.a as u16) % 64) as u8
}

fn encode_qoi(header: &QoiHeader, im: greyscale) {
    let mut running: [Pixel; 64];
    for i in 0..header.height {
        running = [Pixel { r: 0, g: 0, b: 0, a: 255 }; 64];
        let mut pixel = Pixel { r: 0, g: 0, b: 0, a: 255 };
        for j in 0..header.width {
            let index = hash(&pixel);
        }
    }
}

fn parse_args() -> String {
    let args: Vec<String> = std::env::args().collect();

    let input_file = match args.len() {
        2 => &args[1],
        _ => {
            println!("Only one input file can be given");
            exit(1);
        },
    };

    return input_file.to_owned();
}

fn main() {
    let input_file = parse_args();

    if input_file.ends_with(".qoi") {
        println!("Got QOI input file");
    } else if input_file.ends_with(".png") {
        println!("Got PNG input file");
    } else {
        println!("Using sample image");
        let sample_header = QoiHeader {
            magic: [113, 111, 105, 102],
            width: 200,
            height: 100,
            channels: Channels::Three,
            colorspace: Colorspace::One
        };
        let im = generate_sample_image(&sample_header);
        encode_qoi(&sample_header, im);
    }
}
