pub mod img {
    #[path = "jxl.rs"]
    mod jxl_mod;
    #[path = "xpm.rs"]
    mod xpm_mod;

    use image::DynamicImage;

    pub use jxl_mod::jxl;
    pub use xpm_mod::xpm;

    pub struct ImageData {
        pub width: u32,
        pub height: u32,
        pub image: DynamicImage
    }
}
