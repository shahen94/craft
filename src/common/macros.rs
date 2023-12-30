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

// create macro to Implement trait to convert reqwest::Error to our own error type

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


// create macro to Implement trait to convert io::Result to our own error type

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
