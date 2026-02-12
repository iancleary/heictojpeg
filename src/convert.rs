use std::fs;
use std::path::Path;

use image::codecs::jpeg::JpegEncoder;
use img_parts::jpeg::Jpeg;
use img_parts::ImageEXIF;
use libheif_rs::{ColorSpace, HeifContext, LibHeif, RgbChroma};

/// Convert a HEIC file to JPEG, preserving EXIF data.
pub fn convert_heic_to_jpeg(
    input: &Path,
    output: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_file(input.to_str().ok_or("Invalid input path")?)?;
    let handle = ctx.primary_image_handle()?;

    // Extract EXIF metadata before decoding
    let exif_data = extract_exif_from_heif(&handle);

    // Decode to RGB
    let image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;
    let width = image.width();
    let height = image.height();
    let planes = image.planes();
    let interleaved = planes.interleaved.ok_or("No interleaved plane found")?;
    let stride = interleaved.stride;
    let data = interleaved.data;

    // Build a contiguous RGB buffer (stride may include padding)
    let mut rgb_data = Vec::with_capacity((width * height * 3) as usize);
    for row in 0..height as usize {
        let start = row * stride;
        let end = start + (width as usize * 3);
        rgb_data.extend_from_slice(&data[start..end]);
    }

    // Encode to JPEG in memory
    let rgb_image =
        image::RgbImage::from_raw(width, height, rgb_data).ok_or("Failed to create RGB image")?;
    let mut jpeg_bytes = Vec::new();
    let encoder = JpegEncoder::new_with_quality(&mut jpeg_bytes, 95);
    rgb_image.write_with_encoder(encoder)?;

    // If we have EXIF data, inject it into the JPEG
    if let Some(exif) = exif_data {
        let mut jpeg = Jpeg::from_bytes(jpeg_bytes.into())?;
        jpeg.set_exif(Some(exif.into()));
        let mut output_bytes = Vec::new();
        jpeg.encoder().write_to(&mut output_bytes)?;
        fs::write(output, output_bytes)?;
    } else {
        fs::write(output, jpeg_bytes)?;
    }

    Ok(())
}

/// Extract EXIF data from a HEIF image handle.
fn extract_exif_from_heif(
    handle: &libheif_rs::ImageHandle,
) -> Option<Vec<u8>> {
    let exif_fourcc: four_cc::FourCC = four_cc::FourCC(*b"Exif");
    let count = handle.number_of_metadata_blocks(exif_fourcc) as usize;
    if count == 0 {
        return None;
    }

    let mut ids = vec![0u32; count];
    handle.metadata_block_ids(&mut ids, exif_fourcc);

    handle.metadata(ids[0]).ok().map(|data| {
        // libheif EXIF metadata has a 4-byte offset prefix — skip it
        if data.len() > 4 {
            data[4..].to_vec()
        } else {
            data
        }
    })
}
