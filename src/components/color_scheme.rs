use core::num::ParseIntError;
use lazy_static::lazy_static;
use maplit::hashmap;
use palette::{Blend, LinSrgb, LinSrgba, Srgb};
use std::collections::HashMap;

lazy_static! {
    static ref LIGHT_COLOR_SCHEME: ColorScheme = ColorScheme {
        foreground: "#111",
        background: "#fff",
        pages: hashmap! {
            "me" => "#daabbc",
            "people" => "#969f5a",
            "projects" => "#768036",
            "articles" => "#3f2310",
        },
    };
    static ref DARK_COLOR_SCHEME: ColorScheme = ColorScheme {
        foreground: "#fff",
        background: "#191919",
        pages: hashmap! {
            "me" => "#daabbc",
            "people" => "#969f5a",
            "projects" => "#768036",
            "articles" => "#eebf58",
        },
    };
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PreferredColorScheme {
    Dark,
    Light,
    Unspecified,
}

#[derive(Debug)]
enum FromHexError {
    ParseIntError(ParseIntError),
    HexFormatError(&'static str),
}

impl From<ParseIntError> for FromHexError {
    fn from(err: ParseIntError) -> FromHexError {
        FromHexError::ParseIntError(err)
    }
}

impl From<&'static str> for FromHexError {
    fn from(err: &'static str) -> FromHexError {
        FromHexError::HexFormatError(err)
    }
}

struct ColorScheme {
    foreground: &'static str,
    background: &'static str,
    pages: HashMap<&'static str, &'static str>,
}

fn parse_hex_str(hex: &str) -> Result<Srgb<u8>, FromHexError> {
    let hex_code = if hex.starts_with('#') { &hex[1..] } else { hex };
    match hex_code.len() {
        3 => {
            let red = u8::from_str_radix(&hex_code[..1], 16)?;
            let green = u8::from_str_radix(&hex_code[1..2], 16)?;
            let blue = u8::from_str_radix(&hex_code[2..3], 16)?;
            let col: Srgb<u8> = Srgb::new(red * 17, green * 17, blue * 17);
            Ok(col)
        }
        6 => {
            let red = u8::from_str_radix(&hex_code[..2], 16)?;
            let green = u8::from_str_radix(&hex_code[2..4], 16)?;
            let blue = u8::from_str_radix(&hex_code[4..6], 16)?;
            let col: Srgb<u8> = Srgb::new(red, green, blue);
            Ok(col)
        }
        _ => Err("invalid hex code format".into()),
    }
}

fn generate_color_scheme_css(scheme: &ColorScheme) -> String {
    let foreground: Srgb<u8> = parse_hex_str(scheme.foreground).unwrap();
    let background: Srgb<u8> = parse_hex_str(scheme.background).unwrap();

    format!(
        "body {{ background-color: #{:x}; color: #{:x}; }} {}",
        background,
        foreground,
        scheme
            .pages
            .iter()
            .fold(String::new(), |mut result, (name, value)| -> String {
                let color = parse_hex_str(value).unwrap();
                let linear_foreground_alpha: LinSrgba<f32> =
                    LinSrgba::new(foreground.red, foreground.green, foreground.blue, 255)
                        .into_format();
                let linear_color_alpha: LinSrgba<f32> =
                    LinSrgba::new(color.red, color.green, color.blue, 8).into_format();
                let linear_text_color: LinSrgb<f32> =
                    linear_color_alpha.over(linear_foreground_alpha).color;
                let text_color: Srgb<u8> = Srgb::from_format(Srgb::from_linear(linear_text_color));

                result.push_str(&format!("body.{} {{ color: #{:x}; }}", name, text_color));
                result.push_str(&format!("body.{} a {{ color: #{:x}; }}", name, color));
                result.push_str(&format!(".menu > li a.{} {{ color: #{:x}; }}", name, color));
                result
            })
    )
}

pub fn generate_css(color_scheme: PreferredColorScheme) -> String {
    match color_scheme {
        PreferredColorScheme::Dark => format!(
            ".theme-selector .dark {{ display: none; }} {}", generate_color_scheme_css(&DARK_COLOR_SCHEME)),
        PreferredColorScheme::Light => format!(
            ".theme-selector .light {{ display: none; }} {}", generate_color_scheme_css(&LIGHT_COLOR_SCHEME)),
        PreferredColorScheme::Unspecified => format!(
            ".theme-selector .light {{ display: none; }} {} @media screen and (prefers-color-scheme: dark) {{ .theme-selector .dark {{ display: none; }} .theme-selector .light {{ display: inline; }} {} }}",
            generate_color_scheme_css(&LIGHT_COLOR_SCHEME),
            generate_color_scheme_css(&DARK_COLOR_SCHEME)
        ),
    }
}
