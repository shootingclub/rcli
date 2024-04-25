use crate::genpass::GenPassOpts;
use crate::{PASSWORD_LOWER, PASSWORD_NUMBER, PASSWORD_SYMBOL, PASSWORD_UPPER};
use rand::prelude::SliceRandom;

pub fn process_genpass(opts: GenPassOpts) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();
    if opts.uppercase {
        chars.extend_from_slice(PASSWORD_UPPER);
    }
    if opts.lowercase {
        chars.extend_from_slice(PASSWORD_LOWER);
    }
    if opts.number {
        chars.extend_from_slice(PASSWORD_NUMBER);
    }
    if opts.symbol {
        chars.extend_from_slice(PASSWORD_SYMBOL);
    }

    // 根据密码长度=随机次数
    for _ in 0..opts.length {
        let c = chars.choose(&mut rng).unwrap();
        password.push(*c);
    }

    // println!("{:?}", String::from_utf8(password.clone()).unwrap());
    password.shuffle(&mut rng);
    println!("{:?}", String::from_utf8(password.clone()).unwrap());

    Ok(())
}
