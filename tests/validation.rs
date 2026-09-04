#![cfg(feature = "validation")]

use rss::validation::Validate;
use rss::ImageBuilder;

fn image_with_height(height: &str) -> rss::Image {
    ImageBuilder::default()
        .url("https://example.com/image.png".to_string())
        .link("https://example.com/".to_string())
        .title("title".to_string())
        .height(height.to_string())
        .build()
}

fn image_with_width(width: &str) -> rss::Image {
    ImageBuilder::default()
        .url("https://example.com/image.png".to_string())
        .link("https://example.com/".to_string())
        .title("title".to_string())
        .width(width.to_string())
        .build()
}

#[test]
fn image_height_allows_up_to_400() {
    // The RSS 2.0 specification sets the maximum image height to 400.
    assert!(image_with_height("31").validate().is_ok());
    assert!(image_with_height("200").validate().is_ok());
    assert!(image_with_height("400").validate().is_ok());
}

#[test]
fn image_height_above_400_is_invalid() {
    assert!(image_with_height("401").validate().is_err());
}

#[test]
fn image_width_max_is_still_144() {
    // Width keeps its specification maximum of 144.
    assert!(image_with_width("144").validate().is_ok());
    assert!(image_with_width("145").validate().is_err());
}
