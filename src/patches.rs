use serde::{Deserialize, Deserializer};
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Patch {
    name: String,
    enabled: bool,
    patches: Vec<PatchEntry>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct PatchEntry {
    #[serde(deserialize_with = "hex_deserialize")]
    offset: u32,
    #[serde(deserialize_with = "hex_bytes_deserialize")]
    original: Vec<u8>,
    #[serde(deserialize_with = "hex_bytes_deserialize")]
    patch: Vec<u8>,
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_file() {
        let input = indoc! {r#"
          name = "Skip splash screens on startup"
          enabled = true

          [[patches]]
          offset = "ca6ce"
          # jp LEGOStarWarsSaga.exe+CAD10
          original = "0f 8a 3c 06 00 00"
          # jmp LEGOStarWarsSaga.exe+CAD10; nop
          patch = "e9 3d 06 00 00 90"
        "#};
        let patch: Patch = toml::from_str(input).unwrap();
        assert_eq!(
            patch,
            Patch {
                name: "Skip splash screens on startup".to_string(),
                enabled: true,
                patches: vec![PatchEntry {
                    offset: 0xca6ce,
                    original: vec![0x0f, 0x8a, 0x3c, 0x06, 0x00, 0x00],
                    patch: vec![0xe9, 0x3d, 0x06, 0x00, 0x00, 0x90],
                }]
            }
        );
    }
}

fn hex_deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    u32::from_str_radix(&s, 16).map_err(serde::de::Error::custom)
}

fn hex_bytes_deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let without_whitespace = s.replace(" ", "");
    let chars = without_whitespace.chars().collect::<Vec<_>>();
    let pairs: Vec<&[char]> = chars.chunks(2).collect();
    let bytes = pairs
        .into_iter()
        .map(|pair| {
            let pair: String = pair.iter().collect();
            u8::from_str_radix(&pair, 16).map_err(serde::de::Error::custom)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(bytes)
}
