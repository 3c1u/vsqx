pub mod vpr;
pub mod vsqx3;
pub mod vsqx4;

pub(crate) mod write_xml;

// ダウングレード用プログラム
pub(crate) mod v5to4;

// アップグレード用プログラム
pub(crate) mod v4to5;

use failure::Fail;
#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "failed to serialize/deserialize JSON: {}", _0)]
    SerdeJsonError(serde_json::Error),
    #[fail(display = "IO Error: {}", _0)]
    IoError(std::io::Error),
    #[fail(display = "Zip Error: {}", _0)]
    ZipError(zip::result::ZipError),
    #[fail(display = "XML error: {}", _0)]
    XmlError(quick_xml::Error),
    #[fail(display = "XML deserialize error: {}", _0)]
    XmlDeError(quick_xml::DeError),
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJsonError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(e: zip::result::ZipError) -> Self {
        Self::ZipError(e)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(e: quick_xml::Error) -> Self {
        Self::XmlError(e)
    }
}

impl From<quick_xml::DeError> for Error {
    fn from(e: quick_xml::DeError) -> Self {
        Self::XmlDeError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
