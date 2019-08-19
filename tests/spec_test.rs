use parse_dat_url::DatUrl;
use parse_dat_url::Error as ParseError;
use pretty_assertions::assert_eq;
use std::convert::TryInto;
use url::Url;

#[test]
fn it_exposes_the_fields() -> Result<(), ParseError> {
    let dat = DatUrl::parse(
        "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/file.txt",
    )?;

    assert_eq!("dat://", dat.scheme());
    assert_eq!(
        "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21",
        dat.host()
    );
    assert_eq!(&Some("0.0.0.1".into()), dat.version());
    assert_eq!(&Some("/file.txt".into()), dat.path());
    Ok(())
}

#[test]
fn parses_from_str() {
    assert_eq!(
        DatUrl::parse(
            "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/file.txt",
        ),
        "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/file.txt".parse()
    );
}

#[test]
fn try_from_str() {
    assert_eq!(
        DatUrl::parse(
            "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/file.txt",
        ),
        "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/file.txt".try_into()
    );
}

#[test]
fn coerces_to_url() -> Result<(), Box<dyn std::error::Error>> {
    let dat = DatUrl::parse(
        "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/file.txt",
    )?;

    let url = Url::parse(
        "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/file.txt",
    )?;

    assert_eq!(dat.as_ref(), &url);
    Ok(())
}

#[test]
fn it_deals_with_owned_strings() {
    let reference =
        "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/";
    let owned = reference.to_string();

    assert_eq!(DatUrl::parse(reference), DatUrl::parse(&owned))
}

#[test]
fn it_becomes_owned() -> Result<(), ParseError> {
    let reference =
        "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/";

    let dat_url = {
        let url = reference.to_string();
        DatUrl::parse(&url)?.into_owned()
    };

    assert_eq!(dat_url, DatUrl::parse(reference)?);
    Ok(())
}

#[test]
fn dat_url_struct_is_also_a_valid_url() -> Result<(), Box<dyn std::error::Error>> {
    let dat_url = DatUrl::parse(
        "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/",
    )?;
    assert_eq!(
        Url::parse("dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/")?,
        dat_url.into()
    );
    Ok(())
}

#[test]
fn invalid_url_is_not_valid() {
    assert_eq!(
        DatUrl::parse("dat://["),
        Err(ParseError::InvalidUrl(url::ParseError::InvalidIpv6Address))
    )
}

#[test]
fn converts_dat_url_into_string() -> Result<(), ParseError> {
    let dat_url = DatUrl::parse(
        "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/path.txt",
    )?;
    assert_eq!(
        "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/path.txt",
        format!("{}", &dat_url)
    );
    Ok(())
}
