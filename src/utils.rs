use std::collections::HashMap;

// Taken from https://github.com/hyprland-community/Hyprkeys
pub fn mod_mask_to_string(mod_mask: u16) -> Vec<String> {
    // TODO: Make this const
    let mod_masks = HashMap::from([
        (1, "SHIFT"),
        (2, "CAPS"),
        (4, "CTRL"),
        (8, "ALT"),
        (16, "MOD2"),
        (32, "MOD3"),
        (64, "SUPER"),
        (128, "MOD5"),
    ]);
    let mut cur_val = 7;
    let mut result: Vec<String> = Vec::new();
    let mut mod_mask = mod_mask;

    while mod_mask > 0 {
        let mod_val = 1 << cur_val;
        if mod_mask >= mod_val {
            mod_mask -= mod_val;
            result.push(mod_masks[&(1 << cur_val)].to_string());
        }
        cur_val -= 1;
    }
    result
}
