use godot::prelude::*;

//create godot entrypoint
struct SuteraExtension;
#[gdextension]
unsafe impl ExtensionLibrary for SuteraExtension{}

pub mod world;
