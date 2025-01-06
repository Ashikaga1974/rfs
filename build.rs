use windres::Build;

fn main() {
    Build::new().compile("trayicons.rc").unwrap();
}
