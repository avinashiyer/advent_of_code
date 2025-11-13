use std::char;

pub fn naughty_strings(src: Vec<&str>) {
    let mut num_nice = 0;
    for tgt in src {
        let mut two_pair_present = false;
        for (pos, win) in tgt[..tgt.len() - 2].as_bytes().windows(2).enumerate() {
            for other_win in tgt[pos + 2..].as_bytes().windows(2) {
                if win == other_win {
                    two_pair_present = true;
                    break;
                }
            }
        }
        let mut sandwich = false;
        for win in tgt.as_bytes().windows(3) {
            if win[0] == win[2] {
                sandwich = true;
                break;
            }
        }
        if two_pair_present && sandwich {
            num_nice += 1;
        }
    }
    println!("{num_nice}");
}

pub fn naughty_strings_v1(src: Vec<&str>) {
    let mut num_nice = 0;

    for tgt in src {
        if tgt.len() < 3 {
            continue;
        }
        let mut double_letter = false;
        let mut bad_needle = false;
        let mut num_vowels = 0;
        if is_vowel(tgt.bytes().next().unwrap()) {
            num_vowels += 1;
        }
        for win in tgt.as_bytes().windows(2) {
            if is_vowel(win[1]) {
                num_vowels += 1;
            }
            if !double_letter && win[0] == win[1] {
                double_letter = true;
                continue;
            }
            if is_bad_needle(win) {
                bad_needle = true;
                break;
            }
        }
        if num_vowels < 3 || !double_letter || bad_needle {
            continue;
        }
        num_nice += 1;
    }
    println!("{num_nice}");
}

fn is_vowel(ch: u8) -> bool {
    "aeiou".contains(char::from(ch))
}

fn is_bad_needle(couplet: &[u8]) -> bool {
    assert!(couplet.len() == 2);
    let bads = [[b'a', b'b'], [b'c', b'd'], [b'p', b'q'], [b'x', b'y']];
    for bad in bads {
        if couplet == bad {
            return true;
        }
    }
    false
}
