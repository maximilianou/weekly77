use anyhow::Result;
use image::io::Reader as ImageReader;
use image::{ImageOutputFormat, DynamicImage};
use std::io::Cursor;

/// Process uploaded image bytes. If size > 4MB, downscale and recompress to be < 1MB.
/// Returns JSON-like metadata about processed image.
pub async fn process_image_bytes(data: &[u8]) -> Result<serde_json::Value> {
    let size = data.len();
    // Try to decode image
    let img = ImageReader::new(Cursor::new(data)).with_guessed_format()?.decode()?;

    // If larger than 4MB, downscale progressively until output < 1MB
    let mut output = vec![];
    let mut quality = 90u8;
    let mut resized = img;

    if size > 4 * 1024 * 1024 {
        // scale by 0.8 repeatedly until roughly small enough
        let mut factor = 0.8f32;
        loop {
            let neww = (resized.width() as f32 * factor).max(64.0) as u32;
            let newh = (resized.height() as f32 * factor).max(64.0) as u32;
            resized = DynamicImage::ImageRgba8(resized.resize_exact(neww, newh, image::imageops::FilterType::Lanczos3).to_rgba8());
            output.clear();
            resized.write_to(&mut Cursor::new(&mut output), ImageOutputFormat::Jpeg(quality))?;
            if output.len() <= 1 * 1024 * 1024 || quality <= 30 {
                break;
            }
            // reduce quality and shrink more
            quality = (quality as i32 - 15).max(30) as u8;
            factor *= 0.8f32;
        }
    } else {
        // encode as jpeg with moderate quality
        resized.write_to(&mut Cursor::new(&mut output), ImageOutputFormat::Jpeg(80))?;
    }

    Ok(serde_json::json!({"input_size": size, "output_size": output.len(), "quality": quality}))
}
