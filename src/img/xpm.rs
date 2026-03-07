pub mod xpm {
    use image::{DynamicImage, GenericImageView};
    use crate::root::error;
    use crate::root::img::ImageData;

    pub fn decode_xpm(image_file: &str) -> Result<ImageData, error::ImageViewerError> {
        let image_path = std::path::Path::new(image_file).canonicalize()?;
        let img: DynamicImage = image::open(&image_path)?;
        let (width, height) = img.dimensions();

        Ok(ImageData {
            width,
            height,
            image: img,
        })
    }
}
