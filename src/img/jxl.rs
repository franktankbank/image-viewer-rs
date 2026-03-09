pub mod jxl {
    use image::{DynamicImage, ImageDecoder};
    use crate::root::error;
    use crate::root::img::ImageData;

    pub fn decode_jxl(image_file: &str) -> Result<ImageData, error::ImageViewerError> {
        let image_path = std::path::Path::new(image_file).canonicalize()?;

        let file = std::fs::File::open(&image_path)?;

        let decoder = jxl_oxide::integration::JxlDecoder::new(file)?;

        let (width, height) = decoder.dimensions();

        let dynamic_img: DynamicImage = DynamicImage::from_decoder(decoder)?;

        Ok(ImageData {width, height, image: dynamic_img})
    }
}
