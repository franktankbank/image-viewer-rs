pub mod psd {
    use std::io::Read;

    use image::{DynamicImage, ImageBuffer, Rgba};
    use crate::root::error;
    use crate::root::img::ImageData;
    use psd::Psd;

    pub fn decode_psd(image_file: &str) -> Result<ImageData, error::ImageViewerError> {
        let image_path: std::path::PathBuf = std::path::Path::new(image_file).canonicalize()?;
        let mut file = std::fs::File::open(&image_path)?;
        let mut buf: Vec<u8> = vec![0u8; file.metadata()?.len() as usize];
        file.read(&mut buf)?;

        let psd = Psd::from_bytes(buf.as_slice())?;

        let width: u32 = psd.width();
        let height: u32 = psd.height();

        let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(width, height, psd.rgba()).expect("Buffer size mismatch");

        let dynamic_img: DynamicImage = DynamicImage::ImageRgba8(img);

        Ok(ImageData {width, height, image: dynamic_img})
    }
}
