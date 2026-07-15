use std::arch::x86_64::*;

#[target_feature(enable = "sse4.1")]
fn print_tile_sizes() {
    let mut products = [0_i64; 2];
    let products_vector = _mm_mul_epi32(_mm_set_epi32(0, 32, 0, 80), _mm_set_epi32(0, 32, 0, 16));
    unsafe {
        _mm_storeu_si128(products.as_mut_ptr() as *mut __m128i, products_vector);
    }
    println!("Products: {}, {}", products[0], products[1]);
}

fn main() -> std::process::ExitCode {
    if !is_x86_feature_detected!("sse4.1") {
        eprintln!("SSE4.1 is not supported");
        return std::process::ExitCode::FAILURE;
    }
    unsafe {
        print_tile_sizes();
    }
    std::process::ExitCode::SUCCESS
}
