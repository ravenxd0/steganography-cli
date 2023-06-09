
# Program for Steganography in Rust

This is a command-line interface (CLI) program implemented in Rust for steganography using the Least Significant Bit (LSB) encoding technique. The program allows you to hide a message within an image by modifying the LSB of each color channel in the image pixels.


## Usage 

Encoding Message:
```
steganography <cover_image> <stego_image> <secret_message>
```

Decoding Message:
```
steganography <stego_image>
```

## Note
- The program uses LSB encoding with 1 bit per channel, meaning only the least significant bit of each color channel is modified to encode the message
- This Program only supports PNG image format now.
- Be aware that increasing the message size or using more bits per channel for encoding may impact the visual quality of the output image.
