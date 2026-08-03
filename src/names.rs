/// City wordlist for generated workspace names. Names must be short,
/// pronounceable, lowercase, and kebab-safe.
pub const CITIES: &[&str] = &[
    "oslo", "kyoto", "lisbon", "quito", "havana", "denver", "porto", "malmo",
    "riga", "tunis", "osaka", "lagos", "perth", "bogota", "dakar", "hanoi",
    "leeds", "turin", "basel", "seoul", "cairo", "quebec", "manila", "nairobi",
    "geneva", "dublin", "austin", "boise", "fresno", "juneau", "laredo", "macon",
    "nagoya", "odense", "padua", "reno", "salem", "tampa", "utrecht", "vigo",
];

/// Pick an unused city name, given a predicate that reports whether a name is
/// already taken (workspace or branch, checked globally). Falls back to
/// numeric suffixes when the wordlist is exhausted.
pub fn generate(is_taken: impl Fn(&str) -> bool) -> String {
    let offset = std::process::id() as usize % CITIES.len();
    for i in 0..CITIES.len() {
        let candidate = CITIES[(offset + i) % CITIES.len()];
        if !is_taken(candidate) {
            return candidate.to_string();
        }
    }
    let mut n = 2;
    loop {
        let candidate = format!("{}-{n}", CITIES[offset]);
        if !is_taken(&candidate) {
            return candidate;
        }
        n += 1;
    }
}
