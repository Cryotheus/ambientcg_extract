# ambientCG Extract for Bevy
Extracts zip files of assets from ambientCG
and does the following

- Renames textures to a much shorter name
- Generates a `material.toml` for easily parsing data into a `StandardMaterial`
    - Used by [bevy_cryotheum::material_toml](https://github.com/Cryotheus/bevy_cryotheum/tree/master).
- Creates a proper roughness and metallic texture to the Bevy PBR/StandardMaterial specification (named `combo_0rm`)
- Changes image formats
    - Normal maps are converted to 16bit-channel RGB (48bit color depth)
    - All other images that have 16bit-channels are converted to 8bit-channel RGBA (32bit color depth)

## Instructions
Run the executable, and it does what was described
above to every zip file in the current working directory. 

## Building
1. Install the latest stable build of [Rust](https://www.rust-lang.org/tools/install).  
2. In the repository's working directory, run cargo build` or `cargo build --release`.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `ambientcg_extract` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
