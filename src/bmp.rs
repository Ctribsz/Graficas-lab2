use std::fs::File;
use std::io::{Write, BufWriter};

const BMP_HEADER_SIZE: usize = 54;
const BMP_PIXEL_OFFSET: usize = 54;
const BMP_BITS_PER_PIXEL: usize = 32;

pub fn write_bmp_file(
    file_path: &str,       // Path to the output BMP file
    buffer: &[u32],        // Framebuffer pixel data
    width: usize,          // Width of the image
    heigh3t: usize          // Height of the image
) -> std::io::Result<()> {
    // Create a buffered writer for the file
    let file = File::create(file_path)?;
    let mut file = BufWriter::new(file);

    // Write the BMP header
    write_bmp_header(&mut file, width, height)?;

    // Write the pixel data from the framebuffer
    write_pixel_data(&mut file, buffer, width, height)?;

    Ok(())
}

fn write_bmp_header(
    file: &mut BufWriter<File>, // Buffered writer for the file
    width: usize,               // Width of the image
    height: usize               // Height of the image
) -> std::io::Result<()> {
    let file_size = BMP_HEADER_SIZE + (width * height * BMP_BITS_PER_PIXEL / 8);

    // Write BMP signature
    file.write_all(b"BM")?;

    // Write file size
    file.write_all(&(file_size as u32).to_le_bytes())?;

    // Write reserved bytes
    file.write_all(&[0u8; 4])?;

    // Write pixel data offset
    file.write_all(&(BMP_PIXEL_OFFSET as u32).to_le_bytes())?;

    // Write DIB header size
    file.write_all(&((BMP_HEADER_SIZE - 14) as u32).to_le_bytes())?;

    // Write image width
    file.write_all(&(width as u32).to_le_bytes())?;

    // Write image height
    file.write_all(&(height as u32).to_le_bytes())?;

    // Write planes
    file.write_all(&(1u16).to_le_bytes())?;

    // Write bits per pixel
    file.write_all(&(BMP_BITS_PER_PIXEL as u16).to_le_bytes())?;

    // Write compression method (no compression)
    file.write_all(&[0u8; 4])?;

    // Write pixel data size
    file.write_all(&(((width * height * BMP_BITS_PER_PIXEL / 8) as u32).to_le_bytes()))?;

    // Write resolution (pixels per meter)
    file.write_all(&[0u8; 4])?;
    file.write_all(&[0u8; 4])?;

    // Write number of colors (default)
    file.write_all(&[0u8; 4])?;

    // Write important colors (default)
    file.write_all(&[0u8; 4])?;

    Ok(())
}

fn write_pixel_data(
    file: &mut BufWriter<File>, // Buffered writer for the file
    buffer: &[u32],             // Framebuffer pixel data
    width: usize,               // Width of the image
    height: usize               // Height of the image
) -> std::io::Result<()> {
    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = buffer[y * width + x];
            file.write_all(&pixel.to_le_bytes())?;
        }
    }

    Ok(())
}
