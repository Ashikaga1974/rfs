use winresource::WindowsResource;

// App Icon setzen
fn main() {
    WindowsResource::new()
        .set_icon("resources/strategy_goal_progress_grow_icon_262694.ico")
        .compile()
        .unwrap();
}
