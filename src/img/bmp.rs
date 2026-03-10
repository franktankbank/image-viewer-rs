pub mod bmp {
    use image::{DynamicImage, GenericImageView, ImageReader};
    use crate::root::error;
    use crate::root::img::ImageData;

    pub fn decode_bmp(image_file: &str) -> Result<ImageData, error::ImageViewerError> {
        let image_path = std::path::Path::new(image_file).canonicalize()?;

        let img = ImageReader::open(image_path)?;

        let dynamic_img: DynamicImage = img.decode()?;
        let (width, height) = dynamic_img.dimensions();

        Ok(ImageData {width, height, image: dynamic_img})
    }
}
