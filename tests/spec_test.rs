
    use pretty_assertions::assert_eq;
    use parse_dat_url::DatUrl;

   #[test]
   fn it_deals_with_owned_strings() {
       assert_eq!(
           DatUrl::parse("dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/"),
           DatUrl::parse("dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/".to_string())
       )
   }

   #[test]
   fn it_becomes_owned() {
       let datUrl = {
            let url: String =  "dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/".into();
            DatUrl::parse(url)
       };

       assert_eq!(
           datUrl,
           DatUrl::parse("dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+0.0.0.1/")
       )

   }