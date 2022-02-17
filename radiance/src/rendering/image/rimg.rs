use alloc::vec::Vec;
use core2::io::Cursor;

use super::{ImageFormat, RgbaImage};

pub fn load_from_memory_with_format(buffer: &[u8], format: ImageFormat) -> Option<RgbaImage> {
    match format {
        ImageFormat::BMP => load_bmp_from_memory(buffer),
        ImageFormat::TGA => load_tga_from_memory(buffer),
        ImageFormat::DDS => load_dds_from_memory(buffer),
    }
}

fn load_bmp_from_memory(b: &[u8]) -> Option<RgbaImage> {
    let bmp = tinybmp::RawBmp::from_slice(b).ok()?;
    let buffer = Vec::from(bmp.image_data());
    Some(RgbaImage::new(buffer, bmp.size().width, bmp.size().height))
}

fn load_tga_from_memory(b: &[u8]) -> Option<RgbaImage> {
    let tga = tinytga::RawTga::from_slice(b).ok()?;
    let buffer = Vec::from(tga.image_data());
    Some(RgbaImage::new(buffer, tga.size().width, tga.size().height))
}

fn load_dds_from_memory(b: &[u8]) -> Option<RgbaImage> {
    let mut cursor = Cursor::new(b);
    let dds = dds::DDS::decode(&mut cursor).ok()?;
    let buffer = dds.layers[0]
        .iter()
        .flat_map(|p| Into::<[u8; 4]>::into(*p))
        .collect();
    Some(RgbaImage::new(buffer, dds.header.width, dds.header.height))
}
