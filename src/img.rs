pub mod img {
    #[path = "jxl.rs"]
    mod jxl_mod;
    #[path = "xpm.rs"]
    mod xpm_mod;
    #[path = "qoi.rs"]
    mod qoi_mod;
    #[path = "psd.rs"]
    mod psd_mod;

    use image::DynamicImage;

    pub use jxl_mod::jxl;
    pub use xpm_mod::xpm;
    pub use qoi_mod::qoi;
    pub use psd_mod::psd;

    pub struct ImageData {
        pub width: u32,
        pub height: u32,
        pub image: DynamicImage
    }
}
