use std::collections::HashMap;

pub fn calc(word: &String) -> Option<u32> {
    let hashmap = generate_hashmap();
    let mut sum = 0;

    for ch in word.chars() {
        if let Some(&val) = hashmap.get(&ch) {
            sum += val;
        }
        else {
            return None;
        };
    };

    Some(sum)
}

fn generate_hashmap() -> HashMap<char, u32> {
    let mut gematria = HashMap::new();

    let letters  = [
        ('א', 1),
        ('ב', 2),
        ('ג', 3),
        ('ד', 4),
        ('ה', 5),
        ('ו', 6),
        ('ז', 7),
        ('ח', 8),
        ('ט', 9),
        ('י', 10),
        ('כ', 20),
        ('ך', 20),
        ('ל', 30),
        ('מ', 40),
        ('ם', 40),
        ('נ', 50),
        ('ן', 50),
        ('ס', 60),
        ('ע', 70),
        ('פ', 80),
        ('ף', 80),
        ('צ', 90),
        ('ץ', 90),
        ('ק', 100),
        ('ר', 200),
        ('ש', 300),
        ('ת', 400),
        (' ', 0),
        ('\'', 0),
        ('\"', 0)
    ];

    for (letter, value) in letters.iter() {
        gematria.insert(*letter, *value);
    }

    gematria
}