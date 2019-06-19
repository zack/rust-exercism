pub fn raindrops(n: u32) -> String {
    let three_as_factor = n % 3 == 0;
    let five_as_factor = n % 5 == 0;
    let seven_as_factor = n % 7 == 0;

    let mut sound = String::from("");

    if three_as_factor {
        sound.push_str("Pling");
    }
    if five_as_factor {
        sound.push_str("Plang");
    }
    if seven_as_factor {
        sound.push_str("Plong");
    }

    if sound.len() > 0 {
        sound
    } else {
        n.to_string()
    }
}
