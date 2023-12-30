use crate::{error_impl, convert_from_reqwest, convert_from_io};

error_impl!(VersionError);
error_impl!(JsonError);

error_impl!(PackageNotFoundError);
error_impl!(GzipDownloadError);
error_impl!(UnzipError);
error_impl!(InstallError);
error_impl!(UninstallError);


convert_from_reqwest!(PackageNotFoundError);
convert_from_reqwest!(GzipDownloadError);
convert_from_io!(UnzipError);
convert_from_io!(GzipDownloadError);