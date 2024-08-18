#[derive(Debug, PartialEq, Eq)]
pub struct Patch {
    name: String,
    enabled: bool,
    patches: Vec<PatchEntry>,
}

#[derive(Debug, PartialEq, Eq)]
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
    }
}
