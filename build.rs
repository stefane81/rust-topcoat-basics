fn main() {
    // topcoat::tailwind::BuildConfig::new().render().unwrap();
    topcoat::tailwind::BuildConfig::new()
        .input("input.css")
        .render()
        .unwrap();
}
