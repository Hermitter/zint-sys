#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use image::EncodableLayout;
    use libc::strlen;
    use std::ffi::CString;

    // Tested with Zint v2.12.0
    #[test]
    fn hello_code_128() {
        let encoded_text = CString::new("A12345B").expect("CString::new failed");
        let fg_color = CString::new("001100").expect("CString::new failed");
        let bg_color = CString::new("a0b93d").expect("CString::new failed");

        // Barcode configs
        let symbol = unsafe { ZBarcode_Create().as_mut().unwrap() };
        symbol.symbology = BARCODE_CODE128 as i32;
        symbol.output_options |= BARCODE_QUIET_ZONES as i32 | BARCODE_BIND as i32;
        symbol.height = 50.0;
        symbol.show_hrt = 1;
        symbol.border_width = 5;
        symbol.scale = 1.0;
        symbol.whitespace_width = 10;

        // Generate the barcode
        unsafe {
            symbol
                .fgcolor
                .copy_from(fg_color.as_ptr(), strlen(fg_color.as_ptr()));
            symbol
                .bgcolor
                .copy_from(bg_color.as_ptr(), strlen(bg_color.as_ptr()));

            ZBarcode_Encode_and_Buffer(symbol, encoded_text.as_ptr() as *const u8, 0, 0);
        }

        // Store barcode in an image buffer
        let mut img = image::RgbImage::new(symbol.bitmap_width as u32, symbol.bitmap_height as u32);
        let bitmap_length = symbol.bitmap_height * symbol.bitmap_width * 3;
        let bitmap = unsafe { std::slice::from_raw_parts(symbol.bitmap, bitmap_length as usize) };

        let mut i = 0;
        for row in 0..symbol.bitmap_height as u32 {
            for col in 0..symbol.bitmap_width as u32 {
                let r = bitmap[i];
                let g = bitmap[i + 1];
                let b = bitmap[i + 2];
                img.put_pixel(col, row, image::Rgb::from([r, g, b]));

                i += 3;
            }
        }

        // Verify the result
        assert_eq!(
            blake3::hash(img.as_bytes()).to_string(),
            "a903271fa1eddc137621511ee4762f75240928d0f71acf32325288bbdb858fb4"
        );

        // Free memory
        unsafe { ZBarcode_Delete(symbol) }
    }
}
