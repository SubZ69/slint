// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

//! Compares nearest-neighbor against bilinear (`image-rendering: smooth`) sampling cost for
//! scaling an `Rgba` texture.
//!
//! Run with:
//! ```sh
//! cargo bench -p i-slint-renderer-software --features testing
//! ```

use divan::Bencher;
use i_slint_renderer_software::{PremultipliedRgbaColor, bench_draw_rgba_texture_line};

fn main() {
    divan::main();
}

const SRC: u16 = 64;
const DST_WIDTH: i16 = 640;
const DST_HEIGHT: i16 = 480;

fn source_data() -> Vec<u8> {
    (0..SRC as usize * SRC as usize * 4)
        .map(|i| ((i as u32).wrapping_mul(37) % 256) as u8)
        .collect()
}

#[divan::bench(args = [false, true])]
fn scale_rgba(bencher: Bencher, smooth: bool) {
    let data = source_data();
    let mut line_buffer = vec![PremultipliedRgbaColor::default(); DST_WIDTH as usize];

    bencher.bench_local(|| {
        for line in 0..DST_HEIGHT {
            bench_draw_rgba_texture_line(
                &data,
                SRC,
                SRC,
                DST_WIDTH,
                DST_HEIGHT,
                line,
                smooth,
                &mut line_buffer,
            );
        }
    });
}
