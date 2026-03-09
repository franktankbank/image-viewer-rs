pub mod qoi {
    use std::io::Read;

    use image::{ImageBuffer, DynamicImage, Rgb};
    use crate::root::error;
    use crate::root::img::ImageData;

    pub fn decode_qoi(image_file: &str) -> Result<ImageData, error::ImageViewerError> {
        let image_path = std::path::Path::new(image_file).canonicalize()?;
        let mut file = std::fs::File::open(&image_path)?;
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data)?;

        let (header, decoded) = qoi::decode_to_vec(&data)?;

        let img: ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_raw(header.width, header.height, decoded).expect("Buffer size mismatch");
        let dynamic_img: DynamicImage = DynamicImage::ImageRgb8(img);

        Ok(ImageData {
            height: header.height,
            width: header.width,
            image: dynamic_img
        })
    }
}
