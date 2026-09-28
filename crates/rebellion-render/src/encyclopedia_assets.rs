//! Bounded, CPU-only inspection of encyclopedia content bytes.

use std::io::Cursor;

use image::{ImageFormat, ImageReader, Limits};
use sha2::{Digest, Sha256};

/// Maximum retained byte length for one encyclopedia asset.
pub const MAX_ENCYCLOPEDIA_IMAGE_BYTES: usize = 32 * 1024 * 1024;

/// Maximum decoded pixel count for one encyclopedia image.
pub const MAX_ENCYCLOPEDIA_IMAGE_PIXELS: u64 = 16_000_000;

/// Facts derived from the exact retained bytes of one encyclopedia asset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspectedBytes {
    pub sha256: String,
    pub byte_len: u64,
    pub format: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// Inspect retained encyclopedia bytes without filesystem or GPU access.
pub fn inspect_encyclopedia_bytes(
    bytes: &[u8],
    format: Option<&str>,
) -> Result<InspectedBytes, String> {
    if bytes.len() > MAX_ENCYCLOPEDIA_IMAGE_BYTES {
        return Err(format!(
            "encyclopedia asset exceeds byte limit of {MAX_ENCYCLOPEDIA_IMAGE_BYTES} bytes"
        ));
    }

    let byte_len = u64::try_from(bytes.len())
        .map_err(|_| "encyclopedia asset byte length does not fit u64".to_owned())?;

    let Some(requested_format) = format else {
        return Ok(InspectedBytes {
            sha256: sha256_hex(bytes),
            byte_len,
            format: None,
            width: None,
            height: None,
        });
    };

    let image_format = match requested_format {
        "bmp" => ImageFormat::Bmp,
        "png" => ImageFormat::Png,
        other => {
            return Err(format!(
                "unsupported encyclopedia image format {other:?}; expected bmp or png"
            ));
        }
    };

    let detected = image::guess_format(bytes)
        .map_err(|error| format!("encyclopedia image format detection failed: {error}"))?;
    if detected != image_format {
        return Err(format!(
            "encyclopedia image format mismatch: requested {requested_format}, detected {}",
            image_format_name(detected)
        ));
    }

    let (header_width, header_height) = match image_format {
        ImageFormat::Bmp => bmp_dimensions(bytes)?,
        ImageFormat::Png => png_dimensions(bytes)?,
        _ => unreachable!("requested formats are restricted above"),
    };
    let max_decode_bytes = checked_decode_budget(header_width, header_height)?;
    if image_format == ImageFormat::Png {
        validate_png_stream(bytes)
            .map_err(|error| format!("encyclopedia png decode failed: {error}"))?;
    }

    let mut limits = Limits::default();
    limits.max_image_width = Some(header_width);
    limits.max_image_height = Some(header_height);
    limits.max_alloc = Some(max_decode_bytes);
    let mut reader = ImageReader::with_format(Cursor::new(bytes), image_format);
    reader.limits(limits);
    let decoded = reader
        .decode()
        .map_err(|error| format!("encyclopedia {requested_format} decode failed: {error}"))?;
    let decoded_width = decoded.width();
    let decoded_height = decoded.height();
    if (decoded_width, decoded_height) != (header_width, header_height) {
        return Err(format!(
            "encyclopedia {requested_format} decoded dimensions {decoded_width}x{decoded_height} do not match bounded header dimensions {header_width}x{header_height}"
        ));
    }
    drop(decoded);

    Ok(InspectedBytes {
        sha256: sha256_hex(bytes),
        byte_len,
        format: Some(requested_format.to_owned()),
        width: Some(decoded_width),
        height: Some(decoded_height),
    })
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn image_format_name(format: ImageFormat) -> &'static str {
    match format {
        ImageFormat::Bmp => "bmp",
        ImageFormat::Png => "png",
        _ => "another format",
    }
}

fn checked_decode_budget(width: u32, height: u32) -> Result<u64, String> {
    let pixels = u64::from(width)
        .checked_mul(u64::from(height))
        .ok_or_else(|| "encyclopedia image pixel count overflow".to_owned())?;
    // PNG can decode to 16-bit RGBA (eight bytes per pixel); BMP needs no more.
    let decoded_bytes = pixels
        .checked_mul(8)
        .ok_or_else(|| "encyclopedia image decoded byte size overflow".to_owned())?;
    if pixels > MAX_ENCYCLOPEDIA_IMAGE_PIXELS {
        return Err(format!(
            "encyclopedia image exceeds pixel limit of {MAX_ENCYCLOPEDIA_IMAGE_PIXELS} pixels"
        ));
    }
    Ok(decoded_bytes)
}

fn bmp_dimensions(bytes: &[u8]) -> Result<(u32, u32), String> {
    const CORE_HEADER_SIZE: u32 = 12;
    const SUPPORTED_INFO_HEADER_SIZES: [u32; 5] = [40, 52, 56, 108, 124];

    if bytes.len() < 26 {
        return Err("encyclopedia bmp header is truncated".to_owned());
    }

    let dib_size = u32::from_le_bytes(bytes[14..18].try_into().expect("fixed BMP DIB-size slice"));
    if dib_size == CORE_HEADER_SIZE {
        let width = u32::from(u16::from_le_bytes(
            bytes[18..20]
                .try_into()
                .expect("fixed BMP core-width slice"),
        ));
        let height = u32::from(u16::from_le_bytes(
            bytes[20..22]
                .try_into()
                .expect("fixed BMP core-height slice"),
        ));
        return nonzero_dimensions(width, height, "bmp");
    }
    if !SUPPORTED_INFO_HEADER_SIZES.contains(&dib_size) {
        return Err(format!(
            "encyclopedia bmp uses unsupported DIB header size {dib_size}"
        ));
    }

    let signed_width = i32::from_le_bytes(
        bytes[18..22]
            .try_into()
            .expect("fixed BMP info-width slice"),
    );
    let signed_height = i32::from_le_bytes(
        bytes[22..26]
            .try_into()
            .expect("fixed BMP info-height slice"),
    );
    let width = u32::try_from(signed_width)
        .map_err(|_| "encyclopedia bmp width must be positive".to_owned())?;
    let height = signed_height
        .checked_abs()
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(|| "encyclopedia bmp height is invalid".to_owned())?;
    nonzero_dimensions(width, height, "bmp")
}

fn png_dimensions(bytes: &[u8]) -> Result<(u32, u32), String> {
    if bytes.len() < 33 {
        return Err("encyclopedia png header is truncated".to_owned());
    }
    let ihdr_len = u32::from_be_bytes(
        bytes[8..12]
            .try_into()
            .expect("fixed PNG IHDR-length slice"),
    );
    if ihdr_len != 13 || &bytes[12..16] != b"IHDR" {
        return Err("encyclopedia png does not begin with a valid IHDR".to_owned());
    }
    let width = u32::from_be_bytes(bytes[16..20].try_into().expect("fixed PNG width slice"));
    let height = u32::from_be_bytes(bytes[20..24].try_into().expect("fixed PNG height slice"));
    nonzero_dimensions(width, height, "png")
}

fn validate_png_stream(bytes: &[u8]) -> Result<(), String> {
    let mut offset = 8_usize;

    loop {
        if offset == bytes.len() {
            return Err("encyclopedia png does not contain an IEND chunk".to_owned());
        }
        if bytes.len() - offset < 8 {
            return Err(format!(
                "encyclopedia png chunk header is truncated at byte {offset}"
            ));
        }

        let chunk_len = usize::try_from(u32::from_be_bytes(
            bytes[offset..offset + 4]
                .try_into()
                .expect("checked PNG chunk-length slice"),
        ))
        .map_err(|_| "encyclopedia png chunk length does not fit usize".to_owned())?;
        let chunk_type = &bytes[offset + 4..offset + 8];
        let data_start = offset
            .checked_add(8)
            .ok_or_else(|| "encyclopedia png chunk offset overflow".to_owned())?;
        let data_end = data_start
            .checked_add(chunk_len)
            .ok_or_else(|| "encyclopedia png chunk length overflow".to_owned())?;
        if data_end > bytes.len() {
            return Err(format!(
                "encyclopedia png chunk data is truncated at byte {offset}"
            ));
        }
        let crc_end = data_end
            .checked_add(4)
            .ok_or_else(|| "encyclopedia png chunk CRC offset overflow".to_owned())?;
        if crc_end > bytes.len() {
            return Err(format!(
                "encyclopedia png chunk CRC is truncated at byte {offset}"
            ));
        }

        let expected_crc = u32::from_be_bytes(
            bytes[data_end..crc_end]
                .try_into()
                .expect("checked PNG chunk-CRC slice"),
        );
        let actual_crc = png_crc32(&bytes[offset + 4..data_end]);
        if actual_crc != expected_crc {
            return Err(format!(
                "encyclopedia png chunk CRC mismatch at byte {offset}"
            ));
        }

        if chunk_type == b"IEND" {
            if chunk_len != 0 {
                return Err("encyclopedia png IEND chunk must be empty".to_owned());
            }
            if crc_end != bytes.len() {
                return Err("encyclopedia png has trailing bytes after IEND".to_owned());
            }
            return Ok(());
        }

        offset = crc_end;
    }
}

fn png_crc32(bytes: &[u8]) -> u32 {
    const NIBBLE_TABLE: [u32; 16] = [
        0x0000_0000,
        0x1db7_1064,
        0x3b6e_20c8,
        0x26d9_30ac,
        0x76dc_4190,
        0x6b6b_51f4,
        0x4db2_6158,
        0x5005_713c,
        0xedb8_8320,
        0xf00f_9344,
        0xd6d6_a3e8,
        0xcb61_b38c,
        0x9b64_c2b0,
        0x86d3_d2d4,
        0xa00a_e278,
        0xbdbd_f21c,
    ];

    let mut crc = u32::MAX;
    for byte in bytes {
        crc ^= u32::from(*byte);
        crc = (crc >> 4) ^ NIBBLE_TABLE[(crc & 0x0f) as usize];
        crc = (crc >> 4) ^ NIBBLE_TABLE[(crc & 0x0f) as usize];
    }
    !crc
}

fn nonzero_dimensions(width: u32, height: u32, format: &str) -> Result<(u32, u32), String> {
    if width == 0 || height == 0 {
        return Err(format!(
            "encyclopedia {format} dimensions must both be nonzero"
        ));
    }
    Ok((width, height))
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use image::{DynamicImage, ImageFormat, RgbImage};

    use super::*;

    fn encoded_rgb(format: ImageFormat, width: u32, height: u32) -> Vec<u8> {
        let mut pixels = Vec::with_capacity(width as usize * height as usize * 3);
        for y in 0..height {
            for x in 0..width {
                pixels.extend_from_slice(&[x as u8, y as u8, (x + y) as u8]);
            }
        }
        let image = RgbImage::from_raw(width, height, pixels).expect("valid fixture dimensions");
        let mut output = Cursor::new(Vec::new());
        DynamicImage::ImageRgb8(image)
            .write_to(&mut output, format)
            .expect("synthetic image should encode");
        output.into_inner()
    }

    fn one_bit_bmp(width: u32, height: u32, include_pixels: bool) -> Vec<u8> {
        let row_bytes = width.div_ceil(32) * 4;
        let pixel_bytes = row_bytes
            .checked_mul(height)
            .expect("fixture pixel bytes fit u32");
        let pixel_offset = 14_u32 + 40 + 8;
        let file_size = pixel_offset
            .checked_add(pixel_bytes)
            .expect("fixture file bytes fit u32");
        let capacity = if include_pixels {
            file_size as usize
        } else {
            pixel_offset as usize
        };
        let mut bytes = Vec::with_capacity(capacity);
        bytes.extend_from_slice(b"BM");
        bytes.extend_from_slice(&file_size.to_le_bytes());
        bytes.extend_from_slice(&0_u32.to_le_bytes());
        bytes.extend_from_slice(&pixel_offset.to_le_bytes());
        bytes.extend_from_slice(&40_u32.to_le_bytes());
        bytes.extend_from_slice(&(width as i32).to_le_bytes());
        bytes.extend_from_slice(&(height as i32).to_le_bytes());
        bytes.extend_from_slice(&1_u16.to_le_bytes());
        bytes.extend_from_slice(&1_u16.to_le_bytes());
        bytes.extend_from_slice(&0_u32.to_le_bytes());
        bytes.extend_from_slice(&pixel_bytes.to_le_bytes());
        bytes.extend_from_slice(&0_i32.to_le_bytes());
        bytes.extend_from_slice(&0_i32.to_le_bytes());
        bytes.extend_from_slice(&2_u32.to_le_bytes());
        bytes.extend_from_slice(&0_u32.to_le_bytes());
        bytes.extend_from_slice(&[0, 0, 0, 0, 255, 255, 255, 0]);
        if include_pixels {
            bytes.resize(file_size as usize, 0);
        }
        bytes
    }

    fn png_header(width: u32, height: u32) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"\x89PNG\r\n\x1a\n");
        bytes.extend_from_slice(&13_u32.to_be_bytes());
        bytes.extend_from_slice(b"IHDR");
        bytes.extend_from_slice(&width.to_be_bytes());
        bytes.extend_from_slice(&height.to_be_bytes());
        bytes.extend_from_slice(&[8, 2, 0, 0, 0]);
        let crc = png_crc32(&bytes[12..]);
        bytes.extend_from_slice(&crc.to_be_bytes());
        bytes
    }

    #[test]
    fn none_format_hashes_arbitrary_bytes_stably_without_image_facts() {
        let inspected = inspect_encyclopedia_bytes(b"synthetic encyclopedia bytes", None)
            .expect("ordinary bytes should be hashable");

        assert_eq!(
            inspected.sha256,
            "b3a9e3ec75a8923c11d854e33f2b102ec8396fe0a0ee654529d7217c9fd01ef6"
        );
        assert_eq!(inspected.byte_len, 28);
        assert_eq!(inspected.format, None);
        assert_eq!(inspected.width, None);
        assert_eq!(inspected.height, None);
    }

    #[test]
    fn bmp_and_png_are_fully_decoded_and_report_actual_facts() {
        for (format, name) in [(ImageFormat::Bmp, "bmp"), (ImageFormat::Png, "png")] {
            let bytes = encoded_rgb(format, 3, 2);
            let inspected = inspect_encyclopedia_bytes(&bytes, Some(name))
                .expect("complete synthetic image should inspect");

            assert_eq!(inspected.byte_len, bytes.len() as u64);
            assert_eq!(inspected.format.as_deref(), Some(name));
            assert_eq!(inspected.width, Some(3));
            assert_eq!(inspected.height, Some(2));
            assert_eq!(inspected.sha256.len(), 64);
        }
    }

    #[test]
    fn requested_format_must_match_the_actual_payload() {
        let png = encoded_rgb(ImageFormat::Png, 2, 2);
        let error = inspect_encyclopedia_bytes(&png, Some("bmp"))
            .expect_err("a PNG must not pass as a BMP");
        assert!(
            error.contains("format mismatch: requested bmp, detected png"),
            "unexpected error: {error}"
        );

        let bmp = encoded_rgb(ImageFormat::Bmp, 2, 2);
        let error = inspect_encyclopedia_bytes(&bmp, Some("png"))
            .expect_err("a BMP must not pass as a PNG");
        assert!(
            error.contains("format mismatch: requested png, detected bmp"),
            "unexpected error: {error}"
        );

        let unsupported = inspect_encyclopedia_bytes(&png, Some("jpeg"))
            .expect_err("only the contract formats are supported");
        assert!(
            unsupported.contains("unsupported encyclopedia image format"),
            "unexpected error: {unsupported}"
        );
    }

    #[test]
    fn header_valid_truncated_images_fail_full_decode() {
        for (format, name) in [(ImageFormat::Bmp, "bmp"), (ImageFormat::Png, "png")] {
            let mut bytes = encoded_rgb(format, 8, 8);
            bytes.truncate(bytes.len() / 2);

            let error = inspect_encyclopedia_bytes(&bytes, Some(name))
                .expect_err("a truncated image must fail inspection");
            assert!(error.contains("decode failed"), "unexpected error: {error}");
        }
    }

    #[test]
    fn deceptive_headers_do_not_count_as_decoded_images() {
        let bmp = one_bit_bmp(3, 2, false);
        let bmp_error = inspect_encyclopedia_bytes(&bmp, Some("bmp"))
            .expect_err("a plausible BMP header without pixels must fail");
        assert!(
            bmp_error.contains("decode failed"),
            "unexpected error: {bmp_error}"
        );

        let png = png_header(3, 2);
        let png_error = inspect_encyclopedia_bytes(&png, Some("png"))
            .expect_err("a plausible PNG header without image data must fail");
        assert!(
            png_error.contains("does not contain an IEND chunk"),
            "unexpected error: {png_error}"
        );
    }

    #[test]
    fn bmp_header_length_boundary_is_checked_without_panicking_or_skipping_decode() {
        let mut one_byte_short = one_bit_bmp(1, 1, false);
        one_byte_short.truncate(25);
        let error = inspect_encyclopedia_bytes(&one_byte_short, Some("bmp"))
            .expect_err("a 25-byte BMP cannot contain complete dimensions");
        assert!(
            error.contains("header is truncated"),
            "unexpected error: {error}"
        );

        let mut exact_header_length = one_bit_bmp(1, 1, false);
        exact_header_length.truncate(26);
        let error = inspect_encyclopedia_bytes(&exact_header_length, Some("bmp"))
            .expect_err("a 26-byte BMP has dimensions but no complete payload");
        assert!(error.contains("decode failed"), "unexpected error: {error}");
    }

    #[test]
    fn png_requires_both_the_exact_ihdr_length_and_chunk_name() {
        let mut wrong_length = png_header(2, 2);
        wrong_length[8..12].copy_from_slice(&12_u32.to_be_bytes());
        let error = inspect_encyclopedia_bytes(&wrong_length, Some("png"))
            .expect_err("an IHDR with the wrong length must fail");
        assert!(error.contains("valid IHDR"), "unexpected error: {error}");

        let mut wrong_name = png_header(2, 2);
        wrong_name[12..16].copy_from_slice(b"XHDR");
        let error = inspect_encyclopedia_bytes(&wrong_name, Some("png"))
            .expect_err("the first PNG chunk must be IHDR");
        assert!(error.contains("valid IHDR"), "unexpected error: {error}");
    }

    #[test]
    fn either_zero_image_dimension_is_rejected_before_decode() {
        for bytes in [png_header(0, 1), png_header(1, 0)] {
            let error = inspect_encyclopedia_bytes(&bytes, Some("png"))
                .expect_err("zero image dimensions must fail before decode");
            assert!(
                error.contains("dimensions must both be nonzero"),
                "unexpected error: {error}"
            );
        }
    }

    #[test]
    fn byte_budget_accepts_the_exact_limit_and_rejects_one_byte_more() {
        let exact = vec![0_u8; MAX_ENCYCLOPEDIA_IMAGE_BYTES];
        let inspected = inspect_encyclopedia_bytes(&exact, None)
            .expect("the exact byte limit should be accepted");
        assert_eq!(inspected.byte_len, MAX_ENCYCLOPEDIA_IMAGE_BYTES as u64);

        let above = vec![0_u8; MAX_ENCYCLOPEDIA_IMAGE_BYTES + 1];
        let error = inspect_encyclopedia_bytes(&above, None)
            .expect_err("one byte over the limit must be rejected");
        assert!(error.contains("byte limit"), "unexpected error: {error}");
    }

    #[test]
    fn pixel_budget_accepts_the_exact_limit_and_rejects_one_pixel_more() {
        let exact = one_bit_bmp(4_000, 4_000, true);
        let inspected = inspect_encyclopedia_bytes(&exact, Some("bmp"))
            .expect("the exact pixel limit should fully decode");
        assert_eq!(inspected.width, Some(4_000));
        assert_eq!(inspected.height, Some(4_000));

        let above = one_bit_bmp(16_000_001, 1, false);
        let error = inspect_encyclopedia_bytes(&above, Some("bmp"))
            .expect_err("one pixel over the limit must be rejected before decode");
        assert!(error.contains("pixel limit"), "unexpected error: {error}");
    }

    #[test]
    fn checked_dimension_arithmetic_rejects_hostile_png_dimensions() {
        let bytes = png_header(u32::MAX, u32::MAX);
        let error = inspect_encyclopedia_bytes(&bytes, Some("png"))
            .expect_err("hostile dimensions must be rejected before decode");
        assert!(
            error.contains("decoded byte size overflow"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn png_with_truncated_iend_crc_is_rejected() {
        let mut bytes = encoded_rgb(ImageFormat::Png, 2, 2);
        assert_eq!(&bytes[bytes.len() - 8..bytes.len() - 4], b"IEND");
        bytes.truncate(bytes.len() - 4);

        let error = inspect_encyclopedia_bytes(&bytes, Some("png"))
            .expect_err("a PNG with a truncated IEND CRC must fail inspection");
        assert!(
            error.contains("CRC is truncated"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn png_with_corrupt_iend_crc_is_rejected() {
        let mut bytes = encoded_rgb(ImageFormat::Png, 2, 2);
        let last = bytes.len() - 1;
        bytes[last] ^= 1;

        let error = inspect_encyclopedia_bytes(&bytes, Some("png"))
            .expect_err("a PNG with a corrupt IEND CRC must fail inspection");
        assert!(error.contains("CRC mismatch"), "unexpected error: {error}");
    }

    #[test]
    fn png_with_corrupt_nonterminal_chunk_crc_is_rejected() {
        let mut bytes = encoded_rgb(ImageFormat::Png, 2, 2);
        bytes[32] ^= 1;

        let error = inspect_encyclopedia_bytes(&bytes, Some("png"))
            .expect_err("a PNG with a corrupt IHDR CRC must fail inspection");
        assert!(error.contains("CRC mismatch"), "unexpected error: {error}");
    }

    #[test]
    fn png_with_truncated_chunk_header_is_rejected() {
        let mut bytes = encoded_rgb(ImageFormat::Png, 2, 2);
        bytes.truncate(bytes.len() - 12);
        bytes.push(0);

        let error = inspect_encyclopedia_bytes(&bytes, Some("png"))
            .expect_err("a PNG with an incomplete chunk header must fail inspection");
        assert!(
            error.contains("chunk header is truncated"),
            "unexpected error: {error}"
        );
    }
}
