use std::fs::File;
use std::io::{Error, Write};

const LOGO: &str = "       _         _                             _____  _         _                     _               \n      | |       | |                           / ____|| |       (_)                   | |              \n      | |  ___  | |__    __ _  _ __   _ __   | (___  | |_  ___  _  _ __    ___   ___ | | __ ___  _ __ \n  _   | | / _ \\ | '_ \\  / _` || '_ \\ | '_ \\   \\___ \\ | __|/ _ \\| || '_ \\  / _ \\ / __|| |/ // _ \\| '__|\n | |__| || (_) || | | || (_| || | | || | | |  ____) || |_|  __/| || | | ||  __/| (__ |   <|  __/| |   \n  \\____/  \\___/ |_| |_| \\__,_||_| |_||_| |_| |_____/  \\__|\\___||_||_| |_| \\___| \\___||_|\\_\\\\___||_| \n";
const FAVOURITES: [&str; 5] = ["Rust", "C", "C++", "Javascript", "Python"];

fn create_readme() -> Result<(), Error> {
    let path = "README.md";

    let mut output = File::create(path)?;

    write!(
        output,
        "```console\njohannsteinecker@gh:~$ make johannsteinecker-github-readme\n"
    )?;
    write!(output, "{}\n", LOGO)?;
    write!(output, "\u{2714} Found .steinrc\n\n")?;

    for fav in FAVOURITES {
        write!(output, "\u{2764} Added {} to favourites\n", fav)?;
    }

    write!(output, "\n\u{1F9EA} Running tests\n")?;
    write!(
        output,
        "  \u{2714} core module passed with a coverage of 100%\n"
    )?;
    write!(
        output,
        "  \u{2714} configuration module passed with a coverage of 100%\n"
    )?;
    write!(
        output,
        "  \u{2714} love module passed with a coverage of 69%\n\n"
    )?;

    write!(
        output,
        "\u{1F382} Johann Steinecker up N runnin' on port 6969"
    )?;
    write!(output, "\n```")?;
    Ok(())
}

fn main() {
    let _readme_md = create_readme();

    println!(
        "{}                                                                                             v16",
        LOGO
    );

    println!("\u{2714} Found .steinrc\n");

    for fav in FAVOURITES {
        println!("\u{2764} Added {} to favourites", fav);
    }

    println!("\n\u{1F9EA} Running tests");
    println!("  \u{2714} core module passed with a coverage of 100%");
    println!("  \u{2714} configruation module passed with a coverage of 100%");
    println!("  \u{2714} love module passed with a coverage of 69%\n");

    println!("\u{1F382} Johann Steinecker up N runnin' on port 6969");
}
