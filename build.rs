fn main() {
    // todo
    glib_build_tools::compile_resources(
        &["src/resources"],
        "src/resources/resources.gresource.xml",
        "todo_app.gresource",
    );
}
