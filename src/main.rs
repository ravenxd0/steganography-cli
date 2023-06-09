mod lsbstego;

use std::{env, error, path::Path};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: steganography <cover-Image-Path> <Stego-Image_Path> [<Message>]");
        println!("       steganography <stego-Image_path");
        return Ok(());
    }

    let lsb_stego = lsbstego::LSBstego::new();

    if args.len() > 2 {
        let cover_image_path = Path::new(&args[1]);
        let stego_image_path = Path::new(&args[2]);
        let secret_message = &args[3];

        lsb_stego.encode_text(cover_image_path, stego_image_path ,secret_message)?;

        println!("Message is encoded in the Cover Image Successfully.");
        println!("Stego Image Path: {}", stego_image_path.display());
    } else {
        let stego_image_path = Path::new(&args[1]);
        let secret_message = lsb_stego.decode_text(stego_image_path)?;

        println!("Message is decoded from Stego image Successfully");
        println!("The Secret Message : {}", secret_message);
    }

    Ok(())
}
