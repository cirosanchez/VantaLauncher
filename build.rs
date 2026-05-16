fn main() {
    let config = slint_build::CompilerConfiguration::default()
        .with_include_paths(vec!["ui".into()]);

    slint_build::compile_with_config("ui/main.slint", config).expect("Slint build failed");
}
