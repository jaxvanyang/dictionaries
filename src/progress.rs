use std::sync::LazyLock;

use indicatif::ProgressStyle;

pub const STYLE_PROGRESS: LazyLock<ProgressStyle> = LazyLock::new(|| {
    ProgressStyle::with_template("[{bar:40.cyan/blue}] {pos}/{len} {msg}").unwrap()
});

pub const STYLE_DOWNLOAD: LazyLock<ProgressStyle> = LazyLock::new(|| {
    ProgressStyle::with_template(
        "[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
    )
    .unwrap()
});
