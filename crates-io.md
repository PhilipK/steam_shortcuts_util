# Steam Shortcuts utility

Steam Shortcuts is a utility crate that helps you to manage your Steam shortcuts.
It is a simple Rust crate that provides a simple interface to manage your Steam shortcuts.

## Getting started

First include the crate in your project:

```toml
[dependencies]
steam_shortcuts_util = "1.0.0"
```

Then you can use it:

```rust
 use steam_shortcuts_util::parse_shortcuts;
 use steam_shortcuts_util::shortcuts_to_bytes;

 fn example() -> Result<(), Box<dyn std::error::Error>> {
     // This path should be to your steam shortcuts file
     // Usually located at $SteamDirectory/userdata/$SteamUserId/config/shortcuts.vdf
     let content = std::fs::read("src/testdata/shortcuts.vdf")?;
     let shortcuts = parse_shortcuts(content.as_slice())?;
     assert_eq!(shortcuts[0].app_name, "Celeste");
     assert_eq!(3, shortcuts[0].tags.len());

     let shortcut_bytes_vec = shortcuts_to_bytes(&shortcuts);
     assert_eq!(shortcut_bytes_vec, content);
     Ok(())
 }
```

*Be aware that if you overwrite the shortcuts.vdf file, you will have to restart Steam for the changes to take effect.*
