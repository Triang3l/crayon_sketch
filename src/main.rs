mod crayon;

use std::{arch::x86_64::*, io::Write};

fn print_tile_sizes() {
    let mut products = [0_i64; 2];
    unsafe {
        let products_vector =
            _mm_mul_epi32(_mm_set_epi32(0, 32, 0, 80), _mm_set_epi32(0, 32, 0, 16));
        _mm_storeu_si128(products.as_mut_ptr() as *mut __m128i, products_vector);
    }
    println!("Products: {}, {}", products[0], products[1]);
}

fn save_bgra_framebuffer(
    framebuffer: &[u32],
    framebuffer_width: u16,
    framebuffer_height: u16,
) -> std::io::Result<()> {
    let mut file = std::fs::File::create("framebuffer.tga")?;
    file.write_all(&[
        0u8,
        0u8,
        2u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        framebuffer_width as u8,
        (framebuffer_width >> 8) as u8,
        framebuffer_height as u8,
        (framebuffer_height >> 8) as u8,
        32u8,
        0u8,
    ])?;
    let row_length_bytes = framebuffer_width as usize * 4usize;
    for row_index in 0u16..framebuffer_height {
        let row_start =
            (framebuffer_height - 1u16 - row_index) as usize * framebuffer_width as usize;
        let row = &framebuffer[row_start..row_start + framebuffer_width as usize];
        unsafe {
            file.write_all(std::slice::from_raw_parts(
                row.as_ptr() as *const u8,
                row_length_bytes,
            ))?;
        }
    }
    Ok(())
}

fn main() -> std::process::ExitCode {
    print_tile_sizes();

    let framebuffer_width = 1280u32;
    let framebuffer_height = 720u32;
    let mut framebuffer = vec![0u32; framebuffer_width as usize * framebuffer_height as usize];

    framebuffer[0] = 0xFFFF0000u32;
    framebuffer[1] = 0xFF0000FFu32;
    framebuffer[framebuffer_width as usize] = 0xFF00FF00u32;

    let _ = save_bgra_framebuffer(
        &framebuffer,
        framebuffer_width as u16,
        framebuffer_height as u16,
    );

    std::process::ExitCode::SUCCESS
}
