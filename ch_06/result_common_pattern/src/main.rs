use std::string::FromUtf8Error;

fn str_upper_match(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let ret = match String::from_utf8(str) {
        Ok(str) => str.to_uppercase(),
        Err(err) => return Err(err),
    };

    println!("Conversion succeeded: {}", ret);
    Ok(ret)
}

fn str_upper_match_concise(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let ret = String::from_utf8(str).map(|s| s.to_uppercase())?;
    println!("Conversion succeeded: {}", ret);
    Ok(ret)
}

fn main() -> Result<(), &'static str> {
    let invalid_str = str_upper_match(vec![197, 198]);
    println!("{:?}", invalid_str);

    let invalid_str = str_upper_match_concise(vec![197, 198]);
    println!("{:?}", invalid_str);

    let valid_str = str_upper_match(vec![121, 97, 89]);
    println!("{:?}", valid_str);

    let s = vec!["apple", "mango", "banana"];
    let _ = s.get(4).ok_or("I got only 3 fruits")?;
    Ok(())
}
