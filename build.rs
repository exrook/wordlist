use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

#[allow(dead_code)]
fn eff_list<W: Write>(out: W, words: &str, name: &str) -> io::Result<()> {
    let words = words
        .lines()
        .flat_map(|l| l.split_whitespace().skip(1).next());
    write_array(out, words, name)
}

#[allow(dead_code)]
fn pgpfone_lowercase<W: Write>(out: W, words: &str, name: &str) -> io::Result<()> {
    let words = words.split_whitespace().map(|w| w.to_ascii_lowercase());
    write_array(out, words, name)
}

#[allow(dead_code)]
fn space_list<W: Write>(out: W, words: &str, name: &str) -> io::Result<()> {
    let words = words.split_whitespace();
    write_array(out, words, name)
}
#[allow(dead_code)]
fn line_list<W: Write>(out: W, words: &str, name: &str) -> io::Result<()> {
    let words = words.lines();
    write_array(out, words, name)
}
fn write_array<W: Write, I: Iterator<Item = S>, S: AsRef<str>>(
    mut out: W,
    list: I,
    name: &str,
) -> io::Result<()> {
    write!(&mut out, "pub const {}: &[&str] = &[", name)?;
    for word in list {
        write!(&mut out, "\"{}\", ", word.as_ref())?;
    }
    writeln!(&mut out, "];")?;
    Ok(())
}

macro_rules! bip {
    ($file:ident, $lower:literal, $upper:literal) => {
        line_list(
            &mut $file,
            include_str!(concat!("./bip_39_", $lower, ".txt")),
            concat!("BIP_39_", $upper),
        )?;
    };
}

fn main() -> io::Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mut file = File::create(Path::new(&out_dir).join("words.rs")).unwrap();
    #[cfg(feature = "eff_large")]
    eff_list(
        &mut file,
        include_str!("eff_large_wordlist.txt"),
        "EFF_LARGE",
    )?;
    #[cfg(feature = "eff_short_1")]
    eff_list(
        &mut file,
        include_str!("./eff_short_wordlist_1.txt"),
        "EFF_SHORT_1",
    )?;
    #[cfg(feature = "eff_short_2")]
    eff_list(
        &mut file,
        include_str!("./eff_short_wordlist_2_0.txt"),
        "EFF_SHORT_2",
    )?;
    #[cfg(feature = "pgpfone_two_syllable")]
    space_list(
        &mut file,
        include_str!("./pgpfone_two_syllable.txt"),
        "PGPFONE_TWO_SYLLABLE",
    )?;
    #[cfg(feature = "pgpfone_three_syllable")]
    space_list(
        &mut file,
        include_str!("./pgpfone_three_syllable.txt"),
        "PGPFONE_THREE_SYLLABLE",
    )?;
    #[cfg(feature = "pgpfone_two_syllable_lowercase")]
    pgpfone_lowercase(
        &mut file,
        include_str!("./pgpfone_two_syllable.txt"),
        "PGPFONE_TWO_SYLLABLE_LOWERCASE",
    )?;
    #[cfg(feature = "pgpfone_three_syllable_lowercase")]
    pgpfone_lowercase(
        &mut file,
        include_str!("./pgpfone_three_syllable.txt"),
        "PGPFONE_THREE_SYLLABLE_LOWERCASE",
    )?;
    #[cfg(feature = "bip_39_english")]
    bip!(file, "english", "ENGLISH");
    #[cfg(feature = "bip_39_french")]
    bip!(file, "french", "FRENCH");
    #[cfg(feature = "bip_39_italian")]
    bip!(file, "italian", "ITALIAN");
    #[cfg(feature = "bip_39_japanese")]
    bip!(file, "japanese", "JAPANESE");
    #[cfg(feature = "bip_39_korean")]
    bip!(file, "korean", "KOREAN");
    #[cfg(feature = "bip_39_portuguese")]
    bip!(file, "portuguese", "PORTUGUESE");
    #[cfg(feature = "bip_39_spanish")]
    bip!(file, "spanish", "SPANISH");
    Ok(())
}
