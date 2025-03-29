use crate::Errors::{KeyMissing, WrongFormat};
use clap::Parser;
use regashii::{Key, KeyName, Registry, Value, ValueName};
use std::error::Error;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
enum Errors {
    #[error("Key or value is missing!")]
    KeyMissing,

    #[error("Value is wrong format")]
    WrongFormat,
}

const REG_FILE: &str = "user.reg";

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    dpi: Option<u32>,

    #[arg(short, long)]
    retina: Option<bool>,

    #[arg(long, short, action)]
    write_to_test: bool,

    bottle_directory: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let (dpi, retina, test, bottle_directory) = (
        args.dpi,
        args.retina,
        args.write_to_test,
        args.bottle_directory,
    );

    let mut reg_path = PathBuf::from(bottle_directory).canonicalize()?;
    reg_path.push(REG_FILE);

    let registry = Registry::deserialize_file(&reg_path)?;

    match (dpi, retina) {
        (None, None) => {
            let DisplayState { dpi, retina_mode } = get_display_state(&registry)?;
            println!("DPI: {dpi}\nRetina: {retina_mode}")
        }
        (dpi, retina) => {
            set_display_state(registry, dpi, retina, test, reg_path)?;
        }
    }

    Ok(())
}

struct DisplayState {
    dpi: u32,
    retina_mode: String,
}

fn read_val(registry: &Registry, key: &str, val: &str) -> Result<Value, Box<dyn Error>> {
    Ok(registry
        .keys()
        .get(&KeyName::new(key))
        .ok_or(KeyMissing)?
        .values()
        .get(&ValueName::Named(val.to_string()))
        .ok_or(KeyMissing)?
        .clone())
}

fn bool_to_yes(b: bool) -> String {
    match b {
        true => "Y".to_string(),
        false => "N".to_string(),
    }
}

fn get_display_state(registry: &Registry) -> Result<DisplayState, Box<dyn Error>> {
    let Value::Dword(dpi_wine_fonts) = read_val(registry, "Software\\Wine\\Fonts", "LogPixels")?
    else {
        Err(WrongFormat)?
    };

    let Value::Dword(dpi_control_panel) =
        read_val(registry, "Control Panel\\Desktop", "LogPixels")?
    else {
        Err(WrongFormat)?
    };

    let Value::Sz(retina_mode) = read_val(registry, "Software\\Wine\\Mac Driver", "RetinaMode")?
    else {
        Err(WrongFormat)?
    };

    assert_eq!(dpi_control_panel, dpi_wine_fonts);

    Ok(DisplayState {
        dpi: dpi_control_panel,
        retina_mode: retina_mode.to_owned(),
    })
}

fn set_display_state(
    mut registry: Registry,
    dpi: Option<u32>,
    retina: Option<bool>,
    test: bool,
    reg_path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    if let Some(retina) = retina {
        registry = registry.with(
            r"Software\\Wine\\Mac Driver",
            Key::new().with("RetinaMode", Value::Sz(bool_to_yes(retina))),
        );
    };

    if let Some(dpi) = dpi {
        registry = registry
            .with(
                r"Control Panel\\Desktop",
                Key::new().with("LogPixels", Value::Dword(dpi)),
            )
            .with(
                r"Software\\Wine\\Fonts",
                Key::new().with("LogPixels", Value::Dword(dpi)),
            );
    };

    if test {
        registry.serialize_file("test.reg")?;
    } else {
        registry.serialize_file(reg_path)?;
    }

    Ok(())
}
