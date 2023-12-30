/// This macro is used to create Error structs and implements new method for them.
/// 
/// # Example
/// ```
/// error_impl!(GzipDownloadError);
/// ```
#[macro_export]
macro_rules! error_impl {
    ($error:ident) => {
        #[derive(Debug)]
        pub struct $error {
            pub reason: String,
        }

        impl $error {
            pub fn new(reason: String) -> $error {
                $error { reason }
            }
        }
    };
}

/// This macro is used to implement From<reqwest::Error> for Error structs.
/// 
/// # Example
/// ```
/// convert_from_reqwest!(PackageNotFoundError);
/// ```
#[macro_export]
macro_rules! convert_from_reqwest {
    ($error:ident) => {
        impl From<reqwest::Error> for $error {
            fn from(error: reqwest::Error) -> Self {
                $error::new(error.to_string())
            }
        }
    };
}



/// This macro is used to implement From<std::io::Error> for Error structs.
/// 
/// # Example
/// ```
/// convert_from_io!(UnzipError);
/// ```
#[macro_export]
macro_rules! convert_from_io {
    ($error:ident) => {
        impl From<std::io::Error> for $error {
            fn from(error: std::io::Error) -> Self {
                $error::new(error.to_string())
            }
        }
    };
}
