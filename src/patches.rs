use serde::Deserialize;
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Patch {
    name: String,
    enabled: bool,
    patches: Vec<PatchEntry>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct PatchEntry {
    offset: u32,
    original: Vec<u8>,
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
