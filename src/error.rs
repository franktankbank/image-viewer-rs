pub mod error {
    use std::ffi::NulError;
    use std::fmt;
    use std::error::Error;
    use std::io::Error as IoError;
    use image::ImageError;
    use sdl2::video::WindowBuildError;
    use imgui_glow_renderer::InitError;
    use pure_magic::Error as PureMagicError;
    use qoi::Error as QoiError;
    use psd::PsdError;

    #[derive(Debug)]
    pub struct UnsupportedImageError {
        pub mime_type: String
    }

    impl fmt::Display for UnsupportedImageError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "this image format is unsupported\nMime Type: {}", self.mime_type)
        }
    }

    impl Error for UnsupportedImageError {}

    #[derive(Debug)]
    pub struct OpenImageError;

    impl fmt::Display for OpenImageError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "failed to open image")
        }
    }

    impl Error for OpenImageError {}

    #[derive(Debug)]
    pub struct XpmDecodeError {
        pub msg: String
    }

    impl fmt::Display for XpmDecodeError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.msg)
        }
    }

    impl Error for XpmDecodeError {}

    #[derive(Debug)]
    pub enum ImageIdentityError {
        Io(IoError),
        PureMagic(PureMagicError),
        UnsupportedImage(UnsupportedImageError)
    }

    impl fmt::Display for ImageIdentityError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ImageIdentityError::Io(e) => write!(f, "Io Error: {}", e),
                ImageIdentityError::PureMagic(e) => write!(f, "Pure Magic Error: {}", e),
                ImageIdentityError::UnsupportedImage(e) => write!(f, "Unsupported Image Error: {}", e)
            }
        }
    }

    impl Error for ImageIdentityError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                ImageIdentityError::Io(e) => Some(e),
                ImageIdentityError::PureMagic(e) => Some(e),
                ImageIdentityError::UnsupportedImage(e) => Some(e)
            }
        }
    }

    impl From<IoError> for ImageIdentityError {
        fn from(err: IoError ) -> Self {
            ImageIdentityError::Io(err)
        }
    }
    impl From<PureMagicError> for ImageIdentityError {
        fn from(err: PureMagicError) -> Self {
            ImageIdentityError::PureMagic(err)
        }
    }

    impl From<UnsupportedImageError> for ImageIdentityError {
        fn from(err: UnsupportedImageError) -> Self {
            ImageIdentityError::UnsupportedImage(err)
        }
    }

    #[derive(Debug)]
    pub enum ImageViewerError {
        Io(IoError),
        Image(ImageError),
        ImguiInit(InitError),
        Sdl2WindowBuild(WindowBuildError),
        Sdl2Generic(String),
        PureMagic(PureMagicError),
        ImageIdentity(ImageIdentityError),
        XpmDecode(XpmDecodeError),
        OpenImage(OpenImageError),
        Nul(NulError),
        Qoi(QoiError),
        Psd(PsdError)
    }

    impl fmt::Display for ImageViewerError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ImageViewerError::Io(e) => write!(f, "Io Error: {}", e),
                ImageViewerError::Image(e) => write!(f, "Image Error: {}", e),
                ImageViewerError::ImguiInit(e) => write!(f, "Imgui Init Error: {}", e),
                ImageViewerError::Sdl2WindowBuild(e) => write!(f, "Sdl2 Window Build Error: {}", e),
                ImageViewerError::Sdl2Generic(key) => write!(f, "Generic Sdl2 Error: {}", key),
                ImageViewerError::PureMagic(e) => write!(f, "Pure Magic Error: {}", e),
                ImageViewerError::ImageIdentity(e) => write!(f, "Image Identity Error: {}", e),
                ImageViewerError::XpmDecode(e) => write!(f, "Xpm Decode Error: {}", e),
                ImageViewerError::OpenImage(e) => write!(f, "Open Image Error: {}", e),
                ImageViewerError::Nul(e) => write!(f, "NUL Error: {}", e),
                ImageViewerError::Qoi(e) => write!(f, "Qoi Error: {}", e),
                ImageViewerError::Psd(e) => write!(f, "Psd Error: {}", e)
            }
        }
    }

    impl Error for ImageViewerError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                ImageViewerError::Io(e) => Some(e),
                ImageViewerError::Image(e) => Some(e),
                ImageViewerError::ImguiInit(e) => Some(e),
                ImageViewerError::Sdl2WindowBuild(e) => Some(e),
                ImageViewerError::Sdl2Generic(_) => None,
                ImageViewerError::PureMagic(e) => Some(e),
                ImageViewerError::ImageIdentity(e) => Some(e),
                ImageViewerError::XpmDecode(e) => Some(e),
                ImageViewerError::OpenImage(e) => Some(e),
                ImageViewerError::Nul(e) => Some(e),
                ImageViewerError::Qoi(e) => Some(e),
                ImageViewerError::Psd(e) => Some(e)
            }
        }
    }

    impl From<IoError> for ImageViewerError {
        fn from(err: IoError) -> Self {
            ImageViewerError::Io(err)
        }
    }

    impl From<ImageError> for ImageViewerError {
        fn from(err: ImageError) -> Self {
            ImageViewerError::Image(err)
        }
    }

    impl From<InitError> for ImageViewerError {
        fn from(err: InitError) -> Self {
            ImageViewerError::ImguiInit(err)
        }
    }

    impl From<WindowBuildError> for ImageViewerError {
        fn from(err: WindowBuildError) -> Self {
            ImageViewerError::Sdl2WindowBuild(err)
        }
    }

    impl From<String> for ImageViewerError {
        fn from(err: String) -> Self {
            ImageViewerError::Sdl2Generic(err)
        }
    }

    impl From<PureMagicError> for ImageViewerError {
        fn from(err: PureMagicError) -> Self {
            ImageViewerError::PureMagic(err)
        }
    }

    impl From<ImageIdentityError> for ImageViewerError {
        fn from(err: ImageIdentityError) -> Self {
            ImageViewerError::ImageIdentity(err)
        }
    }

    impl From<XpmDecodeError> for ImageViewerError {
        fn from(err: XpmDecodeError) -> Self {
            ImageViewerError::XpmDecode(err)
        }
    }

    impl From<OpenImageError> for ImageViewerError {
        fn from(err: OpenImageError) -> Self {
            ImageViewerError::OpenImage(err)
        }
    }

    impl From<NulError> for ImageViewerError {
        fn from(err: NulError) -> Self {
            ImageViewerError::Nul(err)
        }
    }

    impl From<QoiError> for ImageViewerError {
        fn from(err: QoiError) -> Self {
            ImageViewerError::Qoi(err)
        }
    }

    impl From<PsdError> for ImageViewerError {
        fn from(err: PsdError) -> Self {
            ImageViewerError::Psd(err)
        }
    }
}
