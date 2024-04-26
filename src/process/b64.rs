use crate::{Base64Format, Base64SubCmd};
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::fs::File;
use std::io::Read;

pub fn process_base64(base64: Base64SubCmd) -> anyhow::Result<()> {
    match base64 {
        Base64SubCmd::Encode(encode) => self::encode(encode.input.as_str(), encode.format),
        Base64SubCmd::Decode(decode) => self::decode(decode.input.as_str(), decode.format),
    }
}

fn encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut buf = Vec::new();
    if input == "-" {
        std::io::stdin().read_to_end(&mut buf)?
    } else {
        let mut file = File::open(input)?;
        file.read_to_end(&mut buf)?
    };

    match format {
        Base64Format::Standard => {
            println!("encode Standard {}", input);
            println!("{}", STANDARD.encode(buf));
        }
        Base64Format::UrlSafe => {
            println!("encode UrlSafe {}", input);
            println!("{}", URL_SAFE_NO_PAD.encode(buf));
        }
    };
    Ok(())
}

fn decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    match format {
        Base64Format::Standard => {
            println!("decode Standard {}", input);
            let res = STANDARD.decode(input)?;
            String::from_utf8(res)?
        }
        Base64Format::UrlSafe => {
            println!("decode UrlSafe {}", input);
            let res = URL_SAFE_NO_PAD.decode(input)?;
            String::from_utf8(res)?
        }
    };
    Ok(())
}
