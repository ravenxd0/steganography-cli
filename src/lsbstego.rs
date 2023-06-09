use image::{ImageBuffer, Rgb, RgbImage};
use std::path::Path;

pub struct LSBstego {
    header_pixel_length: u32
}

impl LSBstego {

    pub fn new() -> Self {
        Self { header_pixel_length: 4 }
    }

    fn check_capacity(&self,no_of_bytes: usize, (width,height): (u32,u32)) -> bool {

        no_of_bytes >= 2usize.pow(self.header_pixel_length * 3) && (no_of_bytes as u32 * 8) >= (width*height*3)
    }

    pub fn encode_text(
        &self,
        cover_img_path: &Path,
        secret_msg: &str,
    ) -> Result<RgbImage, Box<dyn std::error::Error>> {
        let cover_img = image::open(cover_img_path)?.to_rgb8();
        let (width, height) = cover_img.dimensions();

        if self.check_capacity(secret_msg.bytes().len(), (width,height)) {
            return Err("Size of message is greater than capacity".into());
        }


        let mut stego_img = ImageBuffer::new(width, height);
        let mut header_pixels_count = 0;
        // Convert message into collection of bits
        let msg_bits: Vec<u8> = secret_msg
            .bytes()
            .flat_map(|byte| (0..8).map(move |i| (byte >> i) & 1))
            .collect();
        let length_of_bytes = secret_msg.bytes().len();
        let length_of_bits = length_of_bytes * 8;
        let mut msg_bits_index = 0;


        for (x, y, pixel) in stego_img.enumerate_pixels_mut() {
            let mut current_pixel = cover_img.get_pixel(x, y).to_owned();

            // First Fixed pixels hold message length
            if header_pixels_count < self.header_pixel_length {
                current_pixel[0] = (current_pixel[0] & 0xFE)
                    | (((length_of_bytes >> (header_pixels_count * 3)) & 0x01) as u8);
                current_pixel[1] = (current_pixel[1] & 0xFE)
                    | (((length_of_bytes >> (1 + header_pixels_count * 3)) & 0x01) as u8);
                current_pixel[2] = (current_pixel[2] & 0xFE)
                    | (((length_of_bytes >> (2 + header_pixels_count * 3)) & 0x01) as u8);
                header_pixels_count += 1;
            } else {
                // Embed bits of message in LSB of pixel channels
                if msg_bits_index < length_of_bits {
                    current_pixel[0] =
                        (current_pixel[0] & 0xFE) | (msg_bits[msg_bits_index] & 0x01);
                    msg_bits_index += 1;
                }
                if msg_bits_index < length_of_bits {
                    current_pixel[1] =
                        (current_pixel[1] & 0xFE) | (msg_bits[msg_bits_index] & 0x01);
                    msg_bits_index += 1;
                }
                if msg_bits_index < length_of_bits {
                    current_pixel[2] =
                        (current_pixel[2] & 0xFE) | (msg_bits[msg_bits_index] & 0x01);
                    msg_bits_index += 1;
                }
            }

            // Save the modified pixel in Stego image
            *pixel = image::Rgb([current_pixel[0], current_pixel[1], current_pixel[2]]);
        }

        Ok(stego_img)
    }

    pub fn decode_text(&self, stego_img_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let stego_img = image::open(stego_img_path)?.into_rgb8();

        let mut header_pixels_count = 0;
        let mut message_length = 0u32;
        let mut message_bits: Vec<u8> = Vec::new();
        let mut are_lsb_zeroes_considered = false;
        let mut no_of_zero_bits = 0;

        for (_x, _y, pixel) in stego_img.enumerate_pixels() {
            if header_pixels_count < self.header_pixel_length {
                // extract message length
                message_length = (message_length << 1) | (pixel[0] & 0x01) as u32;
                message_length = (message_length << 1) | (pixel[1] & 0x01) as u32;
                message_length = (message_length << 1) | (pixel[2] & 0x01) as u32;
                header_pixels_count += 1;

                if !are_lsb_zeroes_considered {
                    are_lsb_zeroes_considered =
                        Self::consider_lsb_0_bits(pixel, &mut no_of_zero_bits);
                }

                if header_pixels_count >= self.header_pixel_length {
                    // message_length have reverse representation of binary of length
                    let mut binary_string: String =
                        format!("{:b}", message_length).chars().rev().collect();
                    // if first bit is zero which is LSB of length which will be ignored at first as it is MSB now so add zero
                    if are_lsb_zeroes_considered {
                        binary_string.push_str("0".repeat(no_of_zero_bits as usize).as_str());

                        are_lsb_zeroes_considered = false;
                        no_of_zero_bits = 0;
                    }
                    // convert into number from binary string
                    message_length = u32::from_str_radix(&binary_string, 2)?;
                }
            } else {
                // Extract message bit from lsb of pixel channel
                if message_bits.len() as u32 >= message_length * 8 {
                    break;
                }
                message_bits.push(pixel[0] & 0x01);
                if message_bits.len() as u32 >= message_length * 8 {
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
        // Take 8 bits and convert it into byte . We reversed because Bits significance are reversed on
        // extraction.
        for chunk in message_bits.chunks_exact(8) {
            let byte: u8 = chunk.iter().rev().fold(0, |acc, &bit| (acc << 1) | bit);
            bytes.push(byte);
        }

        // Convert bytes into String
        let secret_msg = String::from_utf8_lossy(&bytes).trim().to_string();

        Ok(secret_msg)
    }

    fn consider_lsb_0_bits(pixel: &Rgb<u8>, no_of_zero_bits: &mut u8) -> bool {
        let mut are_lsb_zeroes_considered = false;

        if (pixel[0] & 0x01) == 0 {
            *no_of_zero_bits += 1;
        } else {
            are_lsb_zeroes_considered = true;
            return are_lsb_zeroes_considered;
        }
        if (pixel[1] & 0x01) == 0 {
            *no_of_zero_bits += 1;
        } else {
            are_lsb_zeroes_considered = true;
            return are_lsb_zeroes_considered;
        }
        if (pixel[2] & 0x01) == 0 {
            *no_of_zero_bits += 1;
        } else {
            are_lsb_zeroes_considered = true;
            return are_lsb_zeroes_considered;
        }

        are_lsb_zeroes_considered
    }
}
