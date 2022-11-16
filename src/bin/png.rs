use bitvec;
use fiction::micro;
use fiction::png;

fn main() {
    let mut canvas = [false; 4096];
    micro::PrintOptions::default().print("Hello, world!", &mut canvas);

    for (i, bit) in canvas.into_iter().enumerate() {
        if i % 64 == 0 {
            println!();
        }
        print!("{}", if bit { "⬛" } else { "⬜" });
    }
}
