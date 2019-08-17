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
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidUrl,
    MissingHostname,
}

impl<'a> DatUrl<'a> {
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

        Ok(DatUrl {
            version: version.map(Cow::from),
            host: host.into(),
            path: path.map(Cow::from),
            scheme: scheme.into(),
        })
    }

    pub fn into_owned(self) -> DatUrl<'static> {
        DatUrl {
            host: self.host.to_owned().into_owned().into(),
            scheme: self.scheme.to_owned().into_owned().into(),
            version: self.version.to_owned().map(|v| v.into_owned().into()),
            path: self.path.to_owned().map(|p| p.into_owned().into()),
        }
    }

    pub fn url_str(&self) -> String {
        format!(
            "{}{}{}",
            self.scheme,
            self.host,
            self.path.as_ref().map_or("", |path| &path)
        )
    }
}

impl<'a> From<DatUrl<'a>> for Url {
    fn from(dat_url: DatUrl<'a>) -> Self {
        Url::parse(&dat_url.url_str()).expect("invalid daturl object")
    }
}

// #[cfg(test)]
// mod tests {
//     use pretty_assertions::assert_eq;

//     use super::DatUrl;

//     const INPUTS: &str =
//         "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+1/
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+c1/
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1/
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1.0.0/
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+latest/
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/path/to+file.txt
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+1/path/to+file.txt
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+c1/path/to+file.txt
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1/path/to+file.txt
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1.0.0/path/to+file.txt
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+latest/path/to+file.txt
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+1
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+c1
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1.0.0
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+latest
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/path/to+file.txt
// dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+1/
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+c1/
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1/
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1.0.0/
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+latest/
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/path/to+file.txt
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+1/path/to+file.txt
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+c1/path/to+file.txt
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1/path/to+file.txt
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1.0.0/path/to+file.txt
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+latest/path/to+file.txt
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+1
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+c1
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1.0.0
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+latest
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21
// 584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/path/to+file.txt
// dat://foo.com+0.0.0.1/
// dat://foo.com+1/
// dat://foo.com+c1/
// dat://foo.com+v1/
// dat://foo.com+v1.0.0/
// dat://foo.com+latest/
// dat://foo.com+0.0.0.1/path/to+file.txt
// dat://foo.com+1/path/to+file.txt
// dat://foo.com+c1/path/to+file.txt
// dat://foo.com+v1/path/to+file.txt
// dat://foo.com+v1.0.0/path/to+file.txt
// dat://foo.com+latest/path/to+file.txt
// dat://foo.com+0.0.0.1
// dat://foo.com+1
// dat://foo.com+c1
// dat://foo.com+v1
// dat://foo.com+v1.0.0
// dat://foo.com+latest
// dat://foo.com/
// dat://foo.com
// dat://foo.com/path/to+file.txt
// foo.com+0.0.0.1/
// foo.com+1/
// foo.com+c1/
// foo.com+v1/
// foo.com+v1.0.0/
// foo.com+latest/
// foo.com+0.0.0.1/path/to+file.txt
// foo.com+1/path/to+file.txt
// foo.com+c1/path/to+file.txt
// foo.com+v1/path/to+file.txt
// foo.com+v1.0.0/path/to+file.txt
// foo.com+latest/path/to+file.txt
// foo.com+0.0.0.1
// foo.com+1
// foo.com+c1
// foo.com+v1
// foo.com+v1.0.0
// foo.com+latest
// foo.com/
// foo.com
// foo.com/path/to+file.txt";

//     const OUTPUTS: &[DatUrl] = &[
//         DatUrl {
//             version: Some("0.0.0.1".into()),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("c1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1.0.0"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("latest"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("0.0.0.1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("c1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1.0.0"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("latest"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("0.0.0.1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("c1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1.0.0"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("latest"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: None,
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: None,
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: None,
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("0.0.0.1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("c1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1.0.0"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("latest"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("0.0.0.1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("c1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1.0.0"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("latest"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("0.0.0.1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("c1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1.0.0"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("latest"),
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: None,
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: None,
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: None,
//             host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("0.0.0.1"),
//             host: "foo.com",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("1"),
//             host: "foo.com",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("c1"),
//             host: "foo.com",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1"),
//             host: "foo.com",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1.0.0"),
//             host: "foo.com",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("latest"),
//             host: "foo.com",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("0.0.0.1"),
//             host: "foo.com",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("1"),
//             host: "foo.com",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("c1"),
//             host: "foo.com",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1"),
//             host: "foo.com",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1.0.0"),
//             host: "foo.com",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("latest"),
//             host: "foo.com",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("0.0.0.1"),
//             host: "foo.com",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("1"),
//             host: "foo.com",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("c1"),
//             host: "foo.com",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1"),
//             host: "foo.com",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1.0.0"),
//             host: "foo.com",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("latest"),
//             host: "foo.com",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: None,
//             host: "foo.com",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: None,
//             host: "foo.com",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: None,
//             host: "foo.com",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("0.0.0.1"),
//             host: "foo.com",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("1"),
//             host: "foo.com",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("c1"),
//             host: "foo.com",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1"),
//             host: "foo.com",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1.0.0"),
//             host: "foo.com",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("latest"),
//             host: "foo.com",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("0.0.0.1"),
//             host: "foo.com",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("1"),
//             host: "foo.com",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("c1"),
//             host: "foo.com",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1"),
//             host: "foo.com",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1.0.0"),
//             host: "foo.com",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("latest"),
//             host: "foo.com",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("0.0.0.1"),
//             host: "foo.com",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("1"),
//             host: "foo.com",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("c1"),
//             host: "foo.com",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1"),
//             host: "foo.com",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("v1.0.0"),
//             host: "foo.com",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: Some("latest"),
//             host: "foo.com",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: None,
//             host: "foo.com",
//             path: Some("/"),
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: None,
//             host: "foo.com",
//             path: None,
//             scheme: "dat://",
//         },
//         DatUrl {
//             version: None,
//             host: "foo.com",
//             path: Some("/path/to+file.txt"),
//             scheme: "dat://",
//         },
//     ];

//     #[test]
//     fn it_parses_the_urls() {
//         for (url, output) in INPUTS.lines().zip(OUTPUTS) {
//             assert_eq!(&DatUrl::parse(url), output);
//         }
//         assert_eq!(INPUTS.lines().count(), OUTPUTS.len());
//     }
// }
