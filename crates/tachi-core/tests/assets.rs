use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use tachi_core::facade::{cleanup_mislabeled_images, detect_brand_assets, detect_images};

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be monotonic")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{nanos}"))
}

fn write_bytes(path: &Path, bytes: &[u8]) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent directories");
    }
    fs::write(path, bytes).expect("write test file");
}

const PNG_MAGIC: &[u8] = b"\x89PNG\r\n\x1a\n";
const JPEG_MAGIC: &[u8] = b"\xff\xd8\xff\xe0\x00\x10JFIF";

#[test]
fn detect_images_uses_clean_jpeg_without_warnings() {
    let root = unique_temp_dir("tachi-assets-clean-jpeg");
    let template_dir = root.join("templates");
    let target_dir = root.join("target");

    write_bytes(
        &target_dir.join("threat-executive-architecture.jpg"),
        &[JPEG_MAGIC, b"payload"].concat(),
    );

    let images = detect_images(&target_dir, &template_dir);

    assert_eq!(
        images
            .executive_architecture_image_path
            .as_deref()
            .expect("executive architecture image path"),
        "../target/threat-executive-architecture.jpg"
    );
    assert!(!target_dir
        .join("threat-executive-architecture.png")
        .exists());
}

#[test]
fn detect_images_writes_png_sibling_for_mislabeled_jpeg() {
    let root = unique_temp_dir("tachi-assets-mislabeled-png");
    let template_dir = root.join("templates");
    let target_dir = root.join("target");

    write_bytes(
        &target_dir.join("threat-executive-architecture.jpg"),
        &[PNG_MAGIC, b"payload"].concat(),
    );

    let images = detect_images(&target_dir, &template_dir);

    assert_eq!(
        images
            .executive_architecture_image_path
            .as_deref()
            .expect("executive architecture image path"),
        "../target/threat-executive-architecture.png"
    );
    let sibling = target_dir.join("threat-executive-architecture.png");
    assert!(sibling.exists());
    assert!(fs::read(sibling).unwrap().starts_with(PNG_MAGIC));
}

#[test]
fn detect_brand_assets_reports_present_files() {
    let root = unique_temp_dir("tachi-brand-assets");
    let template_dir = root.join("templates");
    let brand_dir = root.join("brand/final");

    write_bytes(&brand_dir.join("tachi-logo-primary.png"), b"logo");
    write_bytes(&brand_dir.join("tachi-logo-horizontal.png"), b"logo");

    let assets = detect_brand_assets(&template_dir, Some(&brand_dir));

    assert!(assets.has_logo_primary);
    assert!(assets.has_logo_horizontal);
    assert_eq!(
        assets.logo_primary_path.as_deref(),
        Some("../brand/final/tachi-logo-primary.png")
    );
    assert_eq!(
        assets.logo_horizontal_path.as_deref(),
        Some("../brand/final/tachi-logo-horizontal.png")
    );
}

#[test]
fn cleanup_removes_an_independent_duplicate_copy() {
    let root = unique_temp_dir("tachi-assets-duplicate-cleanup");
    let bytes = [PNG_MAGIC, b"payload"].concat();
    let jpg = root.join("threat-risk-funnel.jpg");
    let png = root.join("threat-risk-funnel.png");
    write_bytes(&jpg, &bytes);
    write_bytes(&png, &bytes);

    cleanup_mislabeled_images(&root);

    assert!(!jpg.exists());
    assert_eq!(fs::read(png).unwrap(), bytes);
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn cleanup_preserves_candidate_bytes_referenced_by_retained_symlink() {
    use std::os::unix::fs::symlink;

    let root = unique_temp_dir("tachi-assets-symlink-cleanup");
    let bytes = [PNG_MAGIC, b"payload"].concat();
    let jpg = root.join("threat-risk-funnel.jpg");
    let png = root.join("threat-risk-funnel.png");
    write_bytes(&jpg, &bytes);
    symlink(&jpg, &png).expect("create retained PNG symlink");

    cleanup_mislabeled_images(&root);

    assert!(jpg.is_file(), "the symlink target must remain present");
    assert_eq!(fs::read(&png).unwrap(), bytes);
    fs::remove_dir_all(root).unwrap();
}
