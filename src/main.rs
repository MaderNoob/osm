mod html;
mod osm;
mod tmp_dir;
mod ui;

use anyhow::Context;

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {}", error)
    }
}

fn run() -> anyhow::Result<()> {
    tmp_dir::init_tmp_dir().context("failed to initialize tmp dir")?;
    ui::run()?;
    Ok(())
}
