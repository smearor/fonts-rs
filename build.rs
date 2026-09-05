fn main() {
    glib_build_tools::compile_resources(&["resources"], "resources/nerd-fonts.gresource.xml", "compiled.gresource");
}
