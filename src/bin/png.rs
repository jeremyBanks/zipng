use bitvec;
use fiction::fonts;
use fiction::png;

// TODO: the PNG should let you see into the actual zip data, in the style of a
// hex editor like hexyl, by using an 8-bit color pallette.
// This is cooler than it being a file in the archive itself.
// Hexyl uses:
// - null: #242 dark grey
// - printable: #F0F cyan
// - whitespace: #0F0 green
// - non-printable: #F0F red
// - non-ascii: #FF0 yellow
// something like that is okay an an overall scheme, but we're going to define
// unique colors for each of the 256 possible byte value.

const WIDTH: usize = 256; // a quarter-chunk

const COLOR_PAIRS: &[(u8, (u8, u8, u8))] = &[
    (0, (0x00, 0x00, 0x00)),
    (1, (0xFF, 0x00, 0x00)),
    // 0                should be total black
    // ascii whitespace should be mid greys
    // ascii printables should be white/light grey
    // ascii controlsss should be purple
    // non-ascii should be gold to brown
    (128, (255, 255, 128)),
    // ...
    (254, (255, 255, 253)),
    (255, (255, 255, 255)),
];

fn main() {
    let mut canvas = [false; 4096];
    fonts::PrintOptions::default().print("Hello, world!", &mut canvas);

    for (i, bit) in canvas.into_iter().enumerate() {
        if i % 64 == 0 {
            println!();
        }
        print!("{}", if bit { "⬛" } else { "⬜" });
    }
}
