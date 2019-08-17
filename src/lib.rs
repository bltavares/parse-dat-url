use core::fmt;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;
use url::Url;

lazy_static! {
    static ref VERSION_REGEX: Regex = Regex::new(
        r#"(?i)^(?P<scheme>\w+://)?(?P<hostname>[^/+]+)(\+(?P<version>[^/]+))?(?P<path>.*)$"#
    )
    .expect("Version regex not valid");
}

#[derive(Debug, Eq, PartialEq)]
pub struct DatUrl<'a> {
    scheme: Cow<'a, str>,
    host: Cow<'a, str>,
    version: Option<Cow<'a, str>>,
    path: Option<Cow<'a, str>>,
    url: Url,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidUrl,
    MissingHostname,
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

    pub fn parse(url: &str) -> Result<DatUrl, Error> {
        let capture = VERSION_REGEX.captures(url).ok_or(Error::InvalidUrl)?;

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

        let valid_url =
            Url::parse(&DatUrl::url_str(&scheme, &host, &path)).map_err(|_| Error::InvalidUrl)?;

        Ok(DatUrl {
            version: version.map(Cow::from),
            host: host.into(),
            path: path.map(Cow::from),
            scheme: scheme.into(),
            url: valid_url,
        })
    }

    pub fn into_owned(self) -> DatUrl<'static> {
        DatUrl {
            host: self.host.to_owned().into_owned().into(),
            scheme: self.scheme.to_owned().into_owned().into(),
            version: self.version.to_owned().map(|v| v.into_owned().into()),
            path: self.path.to_owned().map(|p| p.into_owned().into()),
            url: self.url,
        }
    }

    #[inline]
    pub fn scheme(&self) -> &Cow<str> {
        &self.scheme
    }

    #[inline]
    pub fn host(&self) -> &Cow<str> {
        &self.host
    }

    #[inline]
    pub fn version(&self) -> &Option<Cow<str>> {
        &self.version
    }

    #[inline]
    pub fn path(&self) -> &Option<Cow<str>> {
        &self.path
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
