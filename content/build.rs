//! Build script for the content crate.
//! 内容 crate 的构建脚本。

fn main() {
    souprune_cauld_ron::build_support::generate_content_registry(&Default::default())
        .expect("failed to generate Cauld-ron content registry");
}
