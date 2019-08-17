use parse_dat_url::DatUrl;
use parse_dat_url::Error as ParseError;
use pretty_assertions::assert_eq;
use url::Url;

#[test]
fn it_deals_with_owned_strings() {
    assert_eq!(
        DatUrl::parse(
            "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/"
        ),
        DatUrl::parse(
            &"dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/"
                .to_string()
        )
    )
}

#[test]
fn it_becomes_owned() {
    let dat_url = {
        let url: String =
            "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/"
                .into();
        DatUrl::parse(&url).expect("invalid test data").into_owned()
    };

    assert_eq!(
        dat_url,
        DatUrl::parse(
            "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/"
        )
        .expect("invalid test data")
    )
}

#[test]
fn dat_url_struct_is_also_a_valid_url() {
    assert_eq!(
        Url::parse("dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21/")
            .expect("invalid test data"),
        DatUrl::parse(
            "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/"
        )
        .expect("invalid test data")
        .into()
    )
}

#[test]
fn invalid_url_is_not_valid() {
    assert_eq!(DatUrl::parse("dat://["), Err(ParseError::InvalidUrl))
}

#[test]
fn converts_dat_url_into_string() {
    assert_eq!(
        "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/path.txt",
        format!(
            "{}",
            DatUrl::parse(
                "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/path.txt"
            )
            .expect("invalid test data")
        )
    );
}
