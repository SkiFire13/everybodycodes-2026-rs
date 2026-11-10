use std::{path::PathBuf, sync::LazyLock};

use aes::cipher::{BlockModeDecrypt, KeyIvInit as _, block_padding::Pkcs7};
use serde::Deserialize;
use ureq::{Agent, Cookie, http::Uri};

const CRATE_ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

static TOKEN: LazyLock<String> = LazyLock::new(|| {
    let token_path = format!("{CRATE_ROOT}/.token");
    std::fs::read_to_string(token_path)
        .expect("Failed to read token")
        .trim()
        .to_string()
});

static AGENT: LazyLock<Agent> = LazyLock::new(|| {
    let agent = Agent::new_with_defaults();

    let uri = Uri::from_static("https://api.everybody.codes");
    let cookie = Cookie::parse(format!("everybody-codes={};", &*TOKEN), &uri)
        .expect("Failed to parse cookie");
    agent
        .cookie_jar_lock()
        .insert(cookie, &uri)
        .expect("Failed to insert cookie");

    agent
});

#[derive(Deserialize)]
struct SeedResponse {
    seed: u64,
}

static SEED: LazyLock<String> = LazyLock::new(|| {
    let seed_path = format!("{CRATE_ROOT}/.seed");

    if let Ok(seed) = std::fs::read_to_string(&seed_path) {
        return seed.trim().to_string();
    }

    let seed = AGENT
        .get("https://api.everybody.codes/user/me")
        .call()
        .expect("Failed to fetch seed")
        .body_mut()
        .read_json::<SeedResponse>()
        .expect("Failed to read seed response")
        .seed
        .to_string();

    if let Err(e) = std::fs::write(seed_path, &seed) {
        eprint!("Failed to store seed: {e}");
    }

    seed
});

#[derive(Deserialize)]
struct InputResponse {
    #[serde(rename = "1")]
    p1: String,
    #[serde(rename = "2")]
    p2: String,
    #[serde(rename = "3")]
    p3: String,
}

#[derive(Debug)]
pub struct Input {
    p1: String,
    p2: Option<String>,
    p3: Option<String>,
}

#[derive(Deserialize)]
pub struct KeysResponse {
    key1: String,
    key2: Option<String>,
    key3: Option<String>,
}

fn decrypt(input: &str, key: &str) -> String {
    type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

    let ciphertext = hex::decode(input).expect("Failed to hex-decode input");

    // Java's "AES/CBC/PKCS5Padding": 32-byte key, 16-byte IV (the first 16
    // bytes of the key). PKCS#5 and PKCS#7 are identical for a 16-byte block.
    let cipher = Aes256CbcDec::new_from_slices(key.as_bytes(), &key.as_bytes()[..16])
        .expect("Invalid AES key/iv length");

    let plaintext = cipher
        .decrypt_padded_vec::<Pkcs7>(&ciphertext)
        .expect("Failed to decrypt");

    String::from_utf8_lossy(&plaintext).into_owned()
}

fn input_path(event: &str, day: &str, part: u32) -> PathBuf {
    format!("{CRATE_ROOT}/inputs/{event}/{day}/{part}.txt").into()
}

fn read_input(event: &str, day: &str, part: u32) -> Option<String> {
    std::fs::read_to_string(input_path(event, day, part)).ok()
}

fn write_input(event: &str, day: &str, part: u32, input: &str) {
    let path = input_path(event, day, part);
    std::fs::create_dir_all(path.parent().unwrap()).expect("Failed to create input directory");
    std::fs::write(path, input).expect("Failed to write input");
}

fn fetch_inputs(event: &str, day: &str) -> Input {
    let seed = &*SEED;
    let input_url = format!("https://everybody.codes/assets/{event}/{day}/input/{seed}.json");
    let aes_url = format!("https://api.everybody.codes/event/{event}/quest/{day}");

    let input = AGENT
        .get(input_url)
        .call()
        .expect("Failed to fetch input")
        .body_mut()
        .read_json::<InputResponse>()
        .expect("Failed to read input response");

    let keys = AGENT
        .get(aes_url)
        .call()
        .expect("Failed to fetch AES keys")
        .body_mut()
        .read_json::<KeysResponse>()
        .expect("Failed to read AES keys");

    let input = Input {
        p1: decrypt(&input.p1, &keys.key1),
        p2: keys.key2.map(|key2| decrypt(&input.p2, &key2)),
        p3: keys.key3.map(|key3| decrypt(&input.p3, &key3)),
    };

    write_input(event, day, 1, &input.p1);
    if let Some(p2) = &input.p2 {
        write_input(event, day, 2, p2);
    }
    if let Some(p3) = &input.p3 {
        write_input(event, day, 3, p3);
    }

    input
}

pub fn get_input(event: &str, day: &str, part: u32) -> String {
    if let Some(input) = read_input(event, day, part) {
        return input;
    }

    let inputs = fetch_inputs(event, day);

    let input = match part {
        1 => Some(inputs.p1),
        2 => inputs.p2,
        3 => inputs.p3,
        _ => panic!("Invalid part"),
    };

    input.expect("Failed to get input")
}
