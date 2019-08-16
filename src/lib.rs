use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

lazy_static! {
// static ref SCHEME_REGEX: Regex = Regex::new(r#"(?i)[a-z]+://"#).expect("Scheme regex should be valid");
static ref VERSION_REGEX: Regex = Regex::new(r#"(?i)^(?P<scheme>dat://)?(?P<hostname>[^/+]+)(\+(?P<version>[^/]+))?(?P<path>.*)$"#).expect("Version rege should be valid");
}

#[derive(Debug, Eq, PartialEq)]
pub struct DatUrl<'a> {
    scheme: Cow<'a, str>,
    host: Cow<'a, str>,
    version: Option<Cow<'a, str>>,
    path: Option<Cow<'a, str>>,
}

impl<'a> DatUrl<'a> {
    pub fn parse<S>(url: S) -> DatUrl<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        let capture = VERSION_REGEX.captures(&url.into()).expect("Invalid dat url");

        let version = capture.name("version").map(|c| c.as_str());

        let host = capture
            .name("hostname")
            .map(|c| c.as_str())
            .expect("Hostname is required");

        let path = capture.name("path").and_then(|c| match c.as_str() {
            "" => None,
            s => Some(s),
        });

        let schema = capture
            .name("scheme")
            .map(|c| c.as_str())
            .unwrap_or("dat://");

        DatUrl {
            version: version.map(Cow::from),
            host: host.into(),
            path: path.map(Cow::from),
            scheme: schema.into(),
        }
    }
}
