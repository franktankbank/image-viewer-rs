pub mod jxl {
    use image::{DynamicImage, ImageDecoder, ImageBuffer, Rgb};
    use crate::root::error;
    use crate::root::img::ImageData;

    pub fn decode_jxl(image_file: &str) -> Result<ImageData, error::ImageViewerError> {
        let image_path = std::path::Path::new(image_file).canonicalize()?;

        let file = std::fs::File::open(&image_path)?;

        let decoder = jxl_oxide::integration::JxlDecoder::new(file)?;

        let (width, height) = decoder.dimensions();

        let buf_size = (width * height * 3) as usize;
        let mut buf = vec![0u8; buf_size];
        decoder.read_image(&mut buf)?;

        let img: ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_raw(width, height, buf).expect("Buffer size mismatch");
        let dynamic_img: DynamicImage = DynamicImage::ImageRgb8(img);

        Ok(ImageData {width, height, image: dynamic_img})
    }
}
