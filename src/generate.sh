#!/usr/bin/env bash
# Generates a Rust static array of keybind strings indexed by bitflags.
#
# Index layout (u16, 14 bits used):
#   bit 13 = meta   (0x2000)
#   bit 12 = hyper  (0x1000)
#   bit 11 = super  (0x0800)
#   bit 10 = shift  (0x0400)
#   bit  9 = alt    (0x0200)
#   bit  8 = ctrl   (0x0100)
#   bits 7:0 = key index  (0=a..25=z, 26..255 reserved for extra keys)
#
# Array size = 2^14 = 16384. Slots with no defined keybind contain "".
# Lookup: KEYBINDS[(CTRL | ALT) as usize | letter_idx('t')]

OUTPUT="keybinds.rs"
LETTERS=({a..z})
N=${#LETTERS[@]}   # 26
ARRAY_SIZE=16384

# Groups: name and hex mask
declare -a GNAMES=( NONE    CTRL     ALT     SHIFT    SUPER    HYPER    META    CTRL_ALT  CTRL_SHIFT  CTRL_ALT_SHIFT )
declare -a GMASKS=( 0x0000  0x0100   0x0200  0x0400   0x0800   0x1000   0x2000  0x0300    0x0500      0x0700         )

{
cat <<'HEADER'
// Auto-generated keybind lookup table.
//
// Index layout (u16, 14 bits used):
//   bit 13 = meta   (0x2000)
//   bit 12 = hyper  (0x1000)
//   bit 11 = super  (0x0800)
//   bit 10 = shift  (0x0400)
//   bit  9 = alt    (0x0200)
//   bit  8 = ctrl   (0x0100)
//   bits 7:0 = key index  (0 = 'a' .. 25 = 'z', 26..255 reserved)
//
// Lookup example:
//   KEYBINDS[(CTRL | ALT) as usize | letter_idx('t')]
//   => "ctrl+alt+t"
//
// Slots with no defined keybind contain "".

// --- Modifier bit flags ---
pub const CTRL:  u16 = 0x0100;
pub const ALT:   u16 = 0x0200;
pub const SHIFT: u16 = 0x0400;
pub const SUPER: u16 = 0x0800;
pub const HYPER: u16 = 0x1000;
pub const META:  u16 = 0x2000;

// --- Key index helper ---
#[inline(always)]
pub const fn letter_idx(c: char) -> usize {
    (c as usize) - ('a' as usize)
}

// --- Group boundary constants ---
HEADER

for i in "${!GNAMES[@]}"; do
    name="${GNAMES[$i]}"
    mask=$(( 16#${GMASKS[$i]#0x} ))
    end=$(( mask + N - 1 ))
    printf 'pub const %s_START: usize = 0x%04X; // %d\n' "$name" "$mask" "$mask"
    printf 'pub const %s_END:   usize = 0x%04X; // %d\n' "$name" "$end"  "$end"
done

echo ""
echo "pub const KEYBINDS: [&str; $ARRAY_SIZE] = ["

for (( idx=0; idx<ARRAY_SIZE; idx++ )); do
    key_idx=$(( idx & 0xFF ))
    ctrl=$(( (idx >> 8) & 1 ))
    alt=$((  (idx >> 9) & 1 ))
    shift=$(( (idx >> 10) & 1 ))
    sup=$((  (idx >> 11) & 1 ))
    hyper=$(( (idx >> 12) & 1 ))
    meta=$(( (idx >> 13) & 1 ))

    if (( key_idx < N )); then
        letter="${LETTERS[$key_idx]}"
        prefix=""
        (( ctrl  )) && prefix="${prefix}ctrl+"
        (( alt   )) && prefix="${prefix}alt+"
        (( shift )) && prefix="${prefix}shift+"
        (( sup   )) && prefix="${prefix}super+"
        (( hyper )) && prefix="${prefix}hyper+"
        (( meta  )) && prefix="${prefix}meta+"
        printf '    "%s%s",\n' "$prefix" "$letter"
    else
        echo '    "",'
    fi
done

cat <<'FOOTER'
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_count() {
        assert_eq!(KEYBINDS.len(), 16384);
    }

    #[test]
    fn test_plain() {
        assert_eq!(KEYBINDS[letter_idx('a')], "a");
        assert_eq!(KEYBINDS[letter_idx('z')], "z");
    }

    #[test]
    fn test_ctrl() {
        assert_eq!(KEYBINDS[CTRL as usize | letter_idx('c')], "ctrl+c");
    }

    #[test]
    fn test_alt() {
        assert_eq!(KEYBINDS[ALT as usize | letter_idx('x')], "alt+x");
    }

    #[test]
    fn test_super() {
        assert_eq!(KEYBINDS[SUPER as usize | letter_idx('t')], "super+t");
    }

    #[test]
    fn test_hyper() {
        assert_eq!(KEYBINDS[HYPER as usize | letter_idx('h')], "hyper+h");
    }

    #[test]
    fn test_meta() {
        assert_eq!(KEYBINDS[META as usize | letter_idx('m')], "meta+m");
    }

    #[test]
    fn test_ctrl_alt() {
        assert_eq!(KEYBINDS[(CTRL | ALT) as usize | letter_idx('t')], "ctrl+alt+t");
    }

    #[test]
    fn test_ctrl_alt_shift() {
        assert_eq!(KEYBINDS[(CTRL | ALT | SHIFT) as usize | letter_idx('f')], "ctrl+alt+shift+f");
    }

    #[test]
    fn test_reserved_slots_empty() {
        assert_eq!(KEYBINDS[26], "");
        assert_eq!(KEYBINDS[CTRL as usize | 200], "");
    }

    #[test]
    fn test_group_boundary_constants() {
        assert_eq!(KEYBINDS[NONE_START],           "a");
        assert_eq!(KEYBINDS[CTRL_START],           "ctrl+a");
        assert_eq!(KEYBINDS[ALT_START],            "alt+a");
        assert_eq!(KEYBINDS[SHIFT_START],          "shift+a");
        assert_eq!(KEYBINDS[SUPER_START],          "super+a");
        assert_eq!(KEYBINDS[HYPER_START],          "hyper+a");
        assert_eq!(KEYBINDS[META_START],           "meta+a");
        assert_eq!(KEYBINDS[CTRL_ALT_START],       "ctrl+alt+a");
        assert_eq!(KEYBINDS[CTRL_SHIFT_START],     "ctrl+shift+a");
        assert_eq!(KEYBINDS[CTRL_ALT_SHIFT_START], "ctrl+alt+shift+a");
    }
}
FOOTER

} > "$OUTPUT"

echo "Generated $OUTPUT (${ARRAY_SIZE} slots)."
