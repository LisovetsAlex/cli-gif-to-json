use clap::Parser;
use serde::Serialize;
use serde_json::to_writer_pretty;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(name = "GIF Saver")]
#[command(about = "Converts GIF frames to JSON using gif-for-cli", long_about = None)]
struct Args {
    /// Path to input GIF file
    #[arg(short, long)]
    input: String,

    /// Output JSON file path
    #[arg(short, long)]
    output: String,

    /// Character to represent pixels
    #[arg(short, long, default_value = "#")]
    character: char,

    /// Number of columns
    #[arg(long, default_value_t = 20)]
    cols: u32,

    /// Number of rows
    #[arg(long, default_value_t = 10)]
    rows: u32,

    /// Maximum number of frames
    #[arg(long, default_value_t = 100)]
    max_frames: u32,
}

#[derive(Serialize)]
struct Pixel {
    character: char,
    color: String,
}

#[derive(Serialize)]
struct Frame {
    pixels: Vec<Vec<Pixel>>,
}
impl Default for Frame {
    fn default() -> Self {
        Frame { pixels: Vec::new() }
    }
}

#[derive(Serialize)]
struct Gif {
    frames: Vec<Frame>,
}
impl Default for Gif {
    fn default() -> Self {
        Gif { frames: Vec::new() }
    }
}

fn main() {
    let args = Args::parse();

    println!("Executing gif-for-cli command...");

    let mut child = Command::new("gif-for-cli")
        .args(&[
            &args.input,
            "--cols",
            &args.cols.to_string(),
            "--rows",
            &args.rows.to_string(),
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start gif-for-cli");

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);

    let mut line_number = 0;
    let mut gif = Gif::default();
    let mut frame = Frame::default();

    println!("Collecting gif data...");

    for line in reader.lines() {
        let line = line.unwrap();

        if line.trim() != "" {
            let pixels = create_pixel_row(&line, args.character);
            frame.pixels.push(pixels);
        }

        line_number += 1;

        if line_number >= args.rows {
            println!("{:?}. frame collected!", gif.frames.len() + 1);

            gif.frames.push(frame);
            frame = Frame::default();
            line_number = 0;

            if gif.frames.len() >= usize::try_from(args.max_frames).unwrap_or(usize::MAX) {
                break;
            }
        }
    }

    println!("Savig data to json...");

    let file = File::create(&args.output).expect("Failed to create output file");
    to_writer_pretty(file, &gif).expect("Failed to write JSON");

    println!("Done.");
}

fn create_pixel_row(line: &str, character: char) -> Vec<Pixel> {
    let line_clean = line.replace("\"", "");
    let pixels_str: Vec<&str> = line_clean.split('#').collect();
    let mut pixels: Vec<Pixel> = Vec::new();

    for pixel_str in pixels_str {
        if pixel_str.trim() == "" {
            continue;
        } else if let Some(pixel) = create_pixel(pixel_str, character) {
            pixels.push(pixel);
        }
    }

    pixels
}

fn create_pixel(string: &str, character: char) -> Option<Pixel> {
    let rgb_command = string.replace("m", "");
    let rgb_settings: Vec<&str> = rgb_command.split(';').collect();
    let len = rgb_settings.len();

    if len < 3 {
        eprintln!("Invalid pixel string: {}", string);
        return None;
    }

    let rgb = &rgb_settings[len.saturating_sub(3)..];

    let r = rgb[0].parse::<u8>().ok()?;
    let g = rgb[1].parse::<u8>().ok()?;
    let b = rgb[2].parse::<u8>().ok()?;

    let color = rgb_to_hex(r, g, b);

    Some(Pixel { character, color })
}

fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}
