//! Build script for the content crate.
//! 内容 crate 的构建脚本。

fn main() {
    souprune_vessel::build_support::generate_content_registry(&Default::default())
        .expect("failed to generate Vessel content registry");
}
