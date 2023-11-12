use image::imageops::FilterType;
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn resize_image(input: *const c_char, output: *const c_char, new_width: u32) {
    let input = unsafe { CStr::from_ptr(input).to_str().unwrap() };
    let output = unsafe { CStr::from_ptr(output).to_str().unwrap() };

    let img = match image::open(input) {
        Ok(img) => img,
        Err(_) => return,
    };

    let resized = img.resize(new_width, new_width, FilterType::Nearest);

    let _ = match resized.save(output) {
        Ok(_) => (),
        Err(_) => return,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GenericImageView, ImageBuffer, Rgb};

    #[test]
    fn test_resize_image() {
        let input = "test_input.jpg";
        let output = "test_output.jpg";
        let new_width = 800;

        // Create a test image
        let img: ImageBuffer<Rgb<u8>, _> = ImageBuffer::new(new_width, new_width);
        img.save(input).unwrap();

        // Test the resize_image function
        let input_cstring = CString::new(input).unwrap();
        let output_cstring = CString::new(output).unwrap();
        resize_image(input_cstring.as_ptr(), output_cstring.as_ptr(), new_width);

        // Check that the output image exists and has the correct dimensions
        let output_img = image::open(output).unwrap();
        assert_eq!(output_img.dimensions(), (new_width, new_width));

        // Clean up
        std::fs::remove_file(input).unwrap();
        std::fs::remove_file(output).unwrap();
    }
}