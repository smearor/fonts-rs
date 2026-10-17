fn main() -> miette::Result<()> {
    fonts_rs_generator::IconSetBuild::from_file("icons.toml")?.run()?;
    Ok(())
}
