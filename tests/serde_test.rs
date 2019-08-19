use parse_dat_url::DatUrl;
use parse_dat_url::Error as ParseError;
use serde_test::{assert_tokens, Token};

#[test]
fn it_serializes_with_serde() -> Result<(), ParseError> {
    let dat = DatUrl::parse(
        "584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/path.txt",
    )?;

    assert_tokens(&dat, &[
        Token::BorrowedStr("dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/path.txt"),
    ]);
    assert_tokens(&dat, &[
        Token::Str("dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/path.txt"),
    ]);
    assert_tokens(&dat, &[
        Token::String("dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/path.txt"),
    ]);
    Ok(())
}
