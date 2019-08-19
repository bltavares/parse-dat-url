#![doc(html_root_url = "https://docs.rs/parse-dat-url/0.1.0/")]
#![deny(missing_docs)]

//! # parse-dat-url
//! url parser to support versioned [dat](https://dat.foundation) URLs
//!
//! Useful links:
//!
//! - [dat.foundation](https://dat.foundation) - Main webpage
//! - [How dat works](https://datprotocol.github.io/how-dat-works/) - Detailed Guide
//! - [datprocol](https://github.com/datprotocol) - Main implementation
//! - [datrs](https://github.com/datrs/) - Rust implementation

use core::fmt;
use core::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;
use url::Url;

#[cfg(feature = "serde")]
mod serde;

lazy_static! {
    static ref VERSION_REGEX: Regex = Regex::new(
        r#"(?i)^(?P<scheme>\w+://)?(?P<hostname>[^/+]+)(\+(?P<version>[^/]+))?(?P<path>.*)$"#
    )
    .expect("Version regex not valid");
}

/// Possible errors returned by the parsing operation
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// Correspond to invalid regex matching.
    InvalidRegex,
    /// Correspond to invalid domain or url, such as bad IPv6 address, or bad encoding on domain names.
    /// Contains a reference to the original `url` parssing error inside.
    InvalidUrl(url::ParseError),
    /// Correspond to missing domain on data.
    MissingHostname,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidRegex => write!(f, "regex defined on library can't match the value")?,
            Error::InvalidUrl(_) => write!(f, "malformed url not conforming to URL Spec")?,
            Error::MissingHostname => write!(f, "missing hostname on url")?,
        };
        Ok(())
    }
}

impl std::error::Error for Error {}

/// Main structure exported. It holds a reference to the string itself, but it is capable of becoming owned, in order to send it across threads.
///
/// It accepts valid urls as well, such as HTTP, domains or IP based URLs. Mal-formed url data might fail, such as bad formatted IPv6 addresses.
/// It is capable to clone the structure into a onwed reference, as it uses [Cow](std::borrow::Cow) internally.
///
/// # Example
///
/// ```rust
/// use parse_dat_url::DatUrl;
///
/// if let Ok(dat_url) = DatUrl::parse("dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/path/to+file.txt") {
///   println!("{}", dat_url);
/// }
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct DatUrl<'a> {
    scheme: Cow<'a, str>,
    host: Cow<'a, str>,
    version: Option<Cow<'a, str>>,
    path: Option<Cow<'a, str>>,
    url: Url,
}

impl<'a> fmt::Display for DatUrl<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.scheme, self.host,)?;
        if let Some(version) = &self.version {
            write!(f, "+{}", version)?;
        }
        if let Some(path) = &self.path {
            write!(f, "{}", path)?;
        }
        Ok(())
    }
}

impl<'a> DatUrl<'a> {
    fn url_str(scheme: &str, host: &str, path: &Option<&str>) -> String {
        format!("{}{}{}", scheme, host, path.map_or("", |path| &path))
    }

    /// Main parsing operation. Returns a struct which makes reference to the `&str` passed, with the same lifetime.
    ///
    /// It is capable to clone the structure into a onwed reference, as it uses [Cow](std::borrow::Cow) internally.
    pub fn parse(url: &str) -> Result<DatUrl, Error> {
        let capture = VERSION_REGEX.captures(url).ok_or(Error::InvalidRegex)?;

        let version = capture.name("version").map(|c| c.as_str());

        let host = capture
            .name("hostname")
            .ok_or(Error::MissingHostname)?
            .as_str();

        let path = capture.name("path").and_then(|c| match c.as_str() {
            "" => None,
            s => Some(s),
        });

        let scheme = capture
            .name("scheme")
            .map(|c| c.as_str())
            .unwrap_or("dat://");

        let valid_url = Url::parse(&DatUrl::url_str(&scheme, &host, &path))
            .map_err(|e| Error::InvalidUrl(e))?;

        Ok(DatUrl {
            version: version.map(Cow::from),
            host: host.into(),
            path: path.map(Cow::from),
            scheme: scheme.into(),
            url: valid_url,
        })
    }

    /// Converts a [DatUrl](parse_dat_url::DatUrl) with a `'a` lifetime into a owned struct, with the `'static` lifetime.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<Error>> {
    /// #
    /// use parse_dat_url::{DatUrl, Error};
    /// // A dynamic URL example.
    /// let url = String::from("dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/path/to+file.txt");
    /// let dat_url = DatUrl::parse(&url)?;
    /// let owned_dat_url : DatUrl<'static> = dat_url.into_owned();
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn into_owned(self) -> DatUrl<'static> {
        DatUrl {
            host: self.host.to_owned().into_owned().into(),
            scheme: self.scheme.to_owned().into_owned().into(),
            version: self.version.to_owned().map(|v| v.into_owned().into()),
            path: self.path.to_owned().map(|p| p.into_owned().into()),
            url: self.url,
        }
    }

    /// Returns a reference to the scheme used on the url. If no scheme is provided on the string, it fallsback to `dat://`
    #[inline]
    pub fn scheme(&self) -> &Cow<str> {
        &self.scheme
    }

    /// Returns the host part of the url.
    #[inline]
    pub fn host(&self) -> &Cow<str> {
        &self.host
    }

    /// Returns a reference to the version on the dat url, if present.
    #[inline]
    pub fn version(&self) -> &Option<Cow<str>> {
        &self.version
    }

    /// Returns a reference to the path on the dat url, if present.
    #[inline]
    pub fn path(&self) -> &Option<Cow<str>> {
        &self.path
    }
}

impl<'a> FromStr for DatUrl<'a> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DatUrl::parse(s).map(DatUrl::into_owned)
    }
}

impl<'a> std::convert::TryFrom<&'a str> for DatUrl<'a> {
    type Error = Error;

    #[inline]
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        DatUrl::parse(s)
    }
}

impl<'a> AsRef<Url> for DatUrl<'a> {
    #[inline]
    fn as_ref(&self) -> &Url {
        &self.url
    }
}

impl<'a> From<DatUrl<'a>> for Url {
    #[inline]
    fn from(dat_url: DatUrl<'a>) -> Self {
        dat_url.url
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::DatUrl;
    use url::Url;

    #[test]
    fn it_parses_the_urls() {
        let inputs: &str =
            "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+1/
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+c1/
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1/
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1.0.0/
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+latest/
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/path/to+file.txt
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+1/path/to+file.txt
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+c1/path/to+file.txt
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1/path/to+file.txt
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1.0.0/path/to+file.txt
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+latest/path/to+file.txt
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+1
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+c1
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1.0.0
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+latest
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/path/to+file.txt
dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+1/
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+c1/
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1/
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1.0.0/
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+latest/
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/path/to+file.txt
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+1/path/to+file.txt
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+c1/path/to+file.txt
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1/path/to+file.txt
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1.0.0/path/to+file.txt
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+latest/path/to+file.txt
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+1
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+c1
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1.0.0
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+latest
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21
584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/path/to+file.txt
dat://example.com+0.0.0.1/
dat://example.com+1/
dat://example.com+c1/
dat://example.com+v1/
dat://example.com+v1.0.0/
dat://example.com+latest/
dat://example.com+0.0.0.1/path/to+file.txt
dat://example.com+1/path/to+file.txt
dat://example.com+c1/path/to+file.txt
dat://example.com+v1/path/to+file.txt
dat://example.com+v1.0.0/path/to+file.txt
dat://example.com+latest/path/to+file.txt
dat://example.com+0.0.0.1
dat://example.com+1
dat://example.com+c1
dat://example.com+v1
dat://example.com+v1.0.0
dat://example.com+latest
dat://example.com/
dat://example.com
dat://example.com/path/to+file.txt
example.com+0.0.0.1/
example.com+1/
example.com+c1/
example.com+v1/
example.com+v1.0.0/
example.com+latest/
example.com+0.0.0.1/path/to+file.txt
example.com+1/path/to+file.txt
example.com+c1/path/to+file.txt
example.com+v1/path/to+file.txt
example.com+v1.0.0/path/to+file.txt
example.com+latest/path/to+file.txt
example.com+0.0.0.1
example.com+1
example.com+c1
example.com+v1
example.com+v1.0.0
example.com+latest
example.com/
example.com
example.com/path/to+file.txt
192.0.2.0
192.0.2.0+v1
192.0.2.0+0.0.0.1/path/to+file.txt
192.0.2.0/path/to+file.txt
[2001:DB8::0]
[2001:DB8::0]+0.0.0.1/path/to+file.txt";

        let outputs: &[DatUrl] = &[
            DatUrl {
                version: Some("0.0.0.1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("c1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1.0.0".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("latest".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("0.0.0.1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/path/to+file.txt",
                )
                .expect("Invalid test data"),
                },
            DatUrl {
                version: Some("c1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1.0.0".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("latest".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("0.0.0.1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("c1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1.0.0".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("latest".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: None,
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/",
                )
                .expect("Invalid test data"), },
            DatUrl {
                version: None,
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: None,
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("0.0.0.1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("c1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1.0.0".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("latest".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("0.0.0.1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("c1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1.0.0".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("latest".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("0.0.0.1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: Some("c1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: Some("v1".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1.0.0".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("latest".into()),
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: None,
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: None,
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: None,
                host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("0.0.0.1".into()),
                host: "example.com".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("1".into()),
                host: "example.com".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("c1".into()),
                host: "example.com".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1".into()),
                host: "example.com".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1.0.0".into()),
                host: "example.com".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("latest".into()),
                host: "example.com".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("0.0.0.1".into()),
                host: "example.com".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("1".into()),
                host: "example.com".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/path/to+file.txt",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: Some("c1".into()),
                host: "example.com".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/path/to+file.txt",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: Some("v1".into()),
                host: "example.com".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/path/to+file.txt",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: Some("v1.0.0".into()),
                host: "example.com".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/path/to+file.txt",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: Some("latest".into()),
                host: "example.com".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/path/to+file.txt",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: Some("0.0.0.1".into()),
                host: "example.com".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: Some("1".into()),
                host: "example.com".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: Some("c1".into()),
                host: "example.com".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: Some("v1".into()),
                host: "example.com".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: Some("v1.0.0".into()),
                host: "example.com".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: Some("latest".into()),
                host: "example.com".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: None,
                host: "example.com".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: None,
                host: "example.com".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: None,
                host: "example.com".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/path/to+file.txt",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: Some("0.0.0.1".into()),
                host: "example.com".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: Some("1".into()),
                host: "example.com".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: Some("c1".into()),
                host: "example.com".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),

            },
            DatUrl {
                version: Some("v1".into()),
                host: "example.com".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1.0.0".into()),
                host: "example.com".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("latest".into()),
                host: "example.com".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("0.0.0.1".into()),
                host: "example.com".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("1".into()),
                host: "example.com".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
   url: Url::parse(
                    "dat://example.com/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("c1".into()),
                host: "example.com".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1".into()),
                host: "example.com".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1.0.0".into()),
                host: "example.com".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("latest".into()),
                host: "example.com".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("0.0.0.1".into()),
                host: "example.com".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("1".into()),
                host: "example.com".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("c1".into()),
                host: "example.com".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1".into()),
                host: "example.com".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1.0.0".into()),
                host: "example.com".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("latest".into()),
                host: "example.com".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: None,
                host: "example.com".into(),
                path: Some("/".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: None,
                host: "example.com".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: None,
                host: "example.com".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://example.com/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: None,
                host: "192.0.2.0".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://192.0.2.0",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("v1".into()),
                host: "192.0.2.0".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://192.0.2.0",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("0.0.0.1".into()),
                host: "192.0.2.0".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://192.0.2.0/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: None,
                host: "192.0.2.0".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://192.0.2.0/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: None,
                host: "[2001:DB8::0]".into(),
                path: None,
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://[2001:DB8::0]",
                )
                .expect("Invalid test data"),
            },
            DatUrl {
                version: Some("0.0.0.1".into()),
                host: "[2001:DB8::0]".into(),
                path: Some("/path/to+file.txt".into()),
                scheme: "dat://".into(),
                url: Url::parse(
                    "dat://[2001:DB8::0]/path/to+file.txt",
                )
                .expect("Invalid test data"),
            },
        ];

        for (url, output) in inputs.lines().zip(outputs) {
            assert_eq!(&DatUrl::parse(url).expect("Invalid test data"), output);
        }
        // assert_eq!(inputs.lines().count(), outputs.len());
    }
}
