use rand::prelude::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ"; // 去掉了 大写 I 大写O
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz"; // 去掉了 小写 l 小写o
const NUMBER: &[u8] = b"23456789"; // 去掉了0 1
const SYMBOL: &[u8] = b"!#$%&*?@^_";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<String> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
        password.push(
            *UPPER
                .choose(&mut rng)
                .expect("UPPER won't be empty in this context"),
        );
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(
            *LOWER
                .choose(&mut rng)
                .expect("LOWER won't be empty in this context"),
        )
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(
            *NUMBER
                .choose(&mut rng)
                .expect("NUMBER won't be empty in this context"),
        )
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(
            *SYMBOL
                .choose(&mut rng)
                .expect("SYMBOL won't be empty in this context"),
        )
    }
    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c);
    }
    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;

    Ok(password)
}
