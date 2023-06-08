use image::{self, DynamicImage,ImageBuffer};

fn main() {
    let cover_img = image::open("cover.png").unwrap();
    let secret_msg = "asfaskfnaldnsaonaspoaspcokascpowkopqopdmwopdmapoddmsfl;asmf;alfmas;lfmas;lfmas;lfma;lfm;l";

    encode_msg(cover_img,secret_msg);
    println!("Text Encoded Successfuly");

    let stego_img = image::open("stego.png").unwrap();
    let decoded_msg = decode_msg(stego_img);
    println!("Decoded Message: {}",decoded_msg);
}

fn encode_msg(cover_img_path: DynamicImage,secret_msg: &str) {
        let cover_img = cover_img_path.into_rgb8();
        let (width,height) = cover_img.dimensions();

        let mut stego_img = ImageBuffer::new(width,height);
        let mut header_pixels_count = 0;
        let header_pixel_length = 4; 
        let msg_bits: Vec<u8> = secret_msg.bytes().flat_map(|byte|
                (0..8).map(move |i| (byte >> i) & 1)
            ).collect();
        let length_of_bytes = secret_msg.bytes().len();
        println!("{} {:b}",length_of_bytes,length_of_bytes);
        let length_of_bits = length_of_bytes * 8;
        let mut msg_bits_index = 0;


        if length_of_bytes > 2usize.pow(header_pixel_length * 3) {
            println!("Size of message is greater than capcity");
            return;
        }

        for (x,y,pixel) in stego_img.enumerate_pixels_mut() {
            let mut current_pixel = cover_img.get_pixel(x,y).to_owned();
            
            if header_pixels_count < header_pixel_length {
                current_pixel[0] = 
                    (current_pixel[0] & 0xFE) | (((length_of_bytes >> (header_pixels_count*3)) &  0x01) as u8);
                current_pixel[1] = 
                    (current_pixel[1] & 0xFE) | (((length_of_bytes >> (1 + header_pixels_count*3)) & 0x01) as u8);
                current_pixel[2] = 
                    (current_pixel[2] & 0xFE) | (((length_of_bytes >> (2 + header_pixels_count*3)) & 0x01) as u8);

                header_pixels_count += 1;
            } else {
                if msg_bits_index < length_of_bits {
                    current_pixel[0] = 
                        (current_pixel[0] & 0xFE) | ( msg_bits[msg_bits_index] & 0x01); 
                    msg_bits_index += 1;
                }
                if msg_bits_index < length_of_bits {
                    current_pixel[1] =
                        (current_pixel[1] & 0xFE) | ( msg_bits[msg_bits_index] & 0x01);
                    msg_bits_index += 1;
                } 
                if msg_bits_index < length_of_bits {
                    current_pixel[2] = 
                        (current_pixel[2] & 0xFE) | ( msg_bits[msg_bits_index] & 0x01);
                    msg_bits_index += 1;
                }

            }

            *pixel = image::Rgb([
                current_pixel[0],
                current_pixel[1],
                current_pixel[2],
            ]);

        }

        stego_img.save("stego.png").unwrap();
        
}

fn decode_msg(stego_img_path: DynamicImage) -> String {

    let stego_img = stego_img_path.to_rgb8();
   
    let mut header_pixels_count = 0;
    let header_pixel_length = 4;
    let mut message_length = 0u32;
    let mut message_bits: Vec<u8> = Vec::new();
    let is_first_bit_zero = if (stego_img.get_pixel(0, 0)[0] & 0x01 ) == 0 {
        true
    } else {
        false
    };

    for (_x,_y,pixel) in stego_img.enumerate_pixels() {
       if  header_pixels_count < header_pixel_length {
           message_length = (message_length << 1 ) | (pixel[0] & 0x01) as u32;
           message_length = (message_length << 1 ) | (pixel[1] & 0x01) as u32;
           message_length = (message_length << 1 ) | (pixel[2] & 0x01) as u32;

           header_pixels_count += 1;
           if header_pixels_count >= header_pixel_length {
               let mut binary_string: String = format!("{:b}",message_length).chars().rev().collect();
               if is_first_bit_zero {
                   binary_string.push('0');
               }
               message_length = u32::from_str_radix(&binary_string, 2).unwrap();

           }
       } else {
           if message_bits.len() as u32 >= message_length * 8  {
               break;
           } 
            message_bits.push(pixel[0] & 0x01);
           if message_bits.len() as u32  >= message_length * 8  {
               break;
           } 
            message_bits.push(pixel[1] & 0x01);
           if message_bits.len() as u32 >= message_length * 8 {
               break;
           } 
            message_bits.push(pixel[2] & 0x01);
       } 
    }

    let mut bytes: Vec<u8> = Vec::new();
    for chunk in message_bits.chunks(8) {
        let byte: u8 = chunk.iter().rev().fold(0,|acc,&bit| (acc << 1) | bit );
        bytes.push(byte);
    }

    String::from_utf8_lossy(&bytes).trim_end_matches(char::from(0)).to_string()

}

