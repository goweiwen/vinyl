fn main() {
    #[cfg(feature = "miyoo")]
    slint_build::compile_with_config(
        "ui/main.slint",
        slint_build::CompilerConfiguration::new()
            .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer),
    )
    .unwrap();

    #[cfg(feature = "simulator")]
    slint_build::compile_with_config("ui/main.slint", slint_build::CompilerConfiguration::new())
        .unwrap();
}
