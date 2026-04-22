fn main() {
    souprune_vessel::build_support::generate_content_registry(&Default::default())
        .expect("failed to generate Vessel content registry");
}
