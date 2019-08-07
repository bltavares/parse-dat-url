use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
// static ref SCHEME_REGEX: Regex = Regex::new(r#"(?i)[a-z]+://"#).expect("Scheme regex should be valid");
static ref VERSION_REGEX: Regex = Regex::new(r#"(?i)^(?P<scheme>dat://)?(?P<hostname>[^/]+)(\+(?P<version>[^/]+))(?P<path>.*)$"#).expect("Version rege should be valid");
}

#[derive(Debug, Eq, PartialEq)]
pub struct DatUrl<'a> {
    version: Option<&'a str>,
    host: &'a str,
    path: Option<&'a str>,
    href: &'a str,
}

impl<'a> DatUrl<'a> {
    pub fn parse(url: &str) -> DatUrl {
        let capture = VERSION_REGEX.captures(url).expect("Valid dat url");

        DatUrl {
            version: capture.name("version").map(|c| c.as_str()),
            host: capture
                .name("hostname")
                .map(|c| c.as_str())
                .expect("Hostname is required"),
            path: capture.name("path").and_then(|c| match c.as_str() {
                "" => None,
                s => Some(s),
            }),
            href: url,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::DatUrl;

    const INPUTS: &str =
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
dat://foo.com+0.0.0.1/
dat://foo.com+1/
dat://foo.com+c1/
dat://foo.com+v1/
dat://foo.com+v1.0.0/
dat://foo.com+latest/
dat://foo.com+0.0.0.1/path/to+file.txt
dat://foo.com+1/path/to+file.txt
dat://foo.com+c1/path/to+file.txt
dat://foo.com+v1/path/to+file.txt
dat://foo.com+v1.0.0/path/to+file.txt
dat://foo.com+latest/path/to+file.txt
dat://foo.com+0.0.0.1
dat://foo.com+1
dat://foo.com+c1
dat://foo.com+v1
dat://foo.com+v1.0.0
dat://foo.com+latest
dat://foo.com/
dat://foo.com
dat://foo.com/path/to+file.txt
foo.com+0.0.0.1/
foo.com+1/
foo.com+c1/
foo.com+v1/
foo.com+v1.0.0/
foo.com+latest/
foo.com+0.0.0.1/path/to+file.txt
foo.com+1/path/to+file.txt
foo.com+c1/path/to+file.txt
foo.com+v1/path/to+file.txt
foo.com+v1.0.0/path/to+file.txt
foo.com+latest/path/to+file.txt
foo.com+0.0.0.1
foo.com+1
foo.com+c1
foo.com+v1
foo.com+v1.0.0
foo.com+latest
foo.com/
foo.com
foo.com/path/to+file.txt";

    const OUTPUTS: &[DatUrl] = &[
        DatUrl {
            version: Some("0.0.0.1"),
            host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
            path: Some("/"),
            href: "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/",
        },
        DatUrl {
            version: Some("1"),
            host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
            path: Some("/"),
            href: "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+1/",
        },
        DatUrl {
            version: Some("c1"),
            host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
            path: Some("/"),
            href: "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+c1/",
        },
        DatUrl {
            version: Some("v1"),
            host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
            path: Some("/"),
            href: "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1/",
        },
        DatUrl {
            version: Some("v1.0.0"),
            host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
            path: Some("/"),
            href: "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1.0.0/",
        },
        DatUrl {
            version: Some("latest"),
            host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
            path: Some("/"),
            href: "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+latest/",
        },
        DatUrl {
            version: Some("0.0.0.1"),
            host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
            path: Some("/path/to+file.txt"),
            href: "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/path/to+file.txt",
        },
        DatUrl {
            version: Some("1"),
            host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
            path: Some("/path/to+file.txt"),
            href: "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+1/path/to+file.txt",
        },
        DatUrl {
            version: Some("c1"),
            host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
            path: Some("/path/to+file.txt"),
            href: "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+c1/path/to+file.txt",
        },
        DatUrl {
            version: Some("v1"),
            host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
            path: Some("/path/to+file.txt"),
            href: "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1/path/to+file.txt",
        },
        DatUrl {
            version: Some("v1.0.0"),
            host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
            path: Some("/path/to+file.txt"),
            href: "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1.0.0/path/to+file.txt",
        },
        DatUrl {
            version: Some("latest"),
            host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
            path: Some("/path/to+file.txt"),
            href: "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+latest/path/to+file.txt",
        },
        DatUrl {
            version: Some("0.0.0.1"),
            host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
            path: None,
            href: "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1",
        },
        DatUrl {
            version: Some("1"),
            host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
            path: None,
            href: "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+1",
        },
        DatUrl {
            version: Some("c1"),
            host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
            path: None,
            href: "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+c1",
        },
        DatUrl {
            version: Some("v1"),
            host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
            path: None,
            href: "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1",
        },
        DatUrl {
            version: Some("v1.0.0"),
            host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
            path: None,
            href: "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1.0.0",
        },
        DatUrl {
            version: Some("latest"),
            host: "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
            path: None,
            href: "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+latest",
        },
    ];

    #[test]
    fn it_parses_the_urls() {
        for (url, output) in INPUTS.lines().zip(OUTPUTS) {
            assert_eq!(&DatUrl::parse(url), output);
        }
    }
}
