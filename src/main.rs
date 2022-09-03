#![allow(unused)]

use std::collections::BTreeMap;

// Stop with the unusual orderings.
// Everything should be lexicographic. They just might not superset each
// other.

static BASES: &[(u32, &[char])] = &[
    (10, &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']),
    (16, &[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ]),
    (32, &[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'J', 'K', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z',
    ]),
    (36, &[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ]),
    (64, &[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '-', '_',
    ]),
];

static AUGMENTS: &[(u32, &[char])] = &[
    (0, &[]),
    (4, &[
        '̀', // ̀_ 0x0300 grave
        '́', // ́_ 0x0301 acute
        '̂', // ̂_ 0x0302 circumflex
        '̊', // ̊_ 0x030A ring
    ]),
    (8, &[
        '̀', // ̀_ 0x0300 grave
        '́', // ́_ 0x0301 acute
        '̂', // ̂_ 0x0302 circumflex
        '̊', // ̊_ 0x030A ring
        '̌', // ̌_ 0x030C caron
        '̽', // ̽_ 0x033D x
        '͆', // ͆_ 0x0346 bridge
        '͛', // ͛_ 0x035B zigzag
    ]),
];

static TOWERS: &[u32] = &[16, 16, 16];

fn main() {
    println!("Hello, world!");
}
