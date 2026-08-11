use godot::prelude::*;

struct EchoBardExt;

#[gdextension]
unsafe impl ExtensionLibrary for EchoBardExt {}

#[derive(GodotClass)]
#[class(init, base=RefCounted)]
struct ExtSample;
