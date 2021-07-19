fn main() {
    let green_check = '\u{2714}';
    let test_tube = '\u{1F9EA}';
    let cake = '\u{1F382}';
    let heart = '\u{2764}';

    println!("       _         _                             _____  _         _                     _               \n      | |       | |                           / ____|| |       (_)                   | |              \n      | |  ___  | |__    __ _  _ __   _ __   | (___  | |_  ___  _  _ __    ___   ___ | | __ ___  _ __ \n  _   | | / _ \\ | '_ \\  / _` || '_ \\ | '_ \\   \\___ \\ | __|/ _ \\| || '_ \\  / _ \\ / __|| |/ // _ \\| '__|\n | |__| || (_) || | | || (_| || | | || | | |  ____) || |_|  __/| || | | ||  __/| (__ |   <|  __/| |   \n  \\____/  \\___/ |_| |_| \\__,_||_| |_||_| |_| |_____/  \\__|\\___||_||_| |_| \\___| \\___||_|\\_\\\\___||_|   \n                                                                                                      \n                                                                                                      ");

    println!("{} Found .steinrc\n", green_check);

    println!("{} Added Rust to favourites", heart);
    println!("{} Added C to favourites", heart);
    println!("{} Added C++ to favourites", heart);
    println!("{} Added Javascript to favourites", heart);
    println!("{} Added Python to favourites\n", heart);

    println!("{} Running tests", test_tube);
    println!(
        "  {} core module passed with a coverage of 100%",
        green_check
    );
    println!(
        "  {} configruation module passed with a coverage of 100%",
        green_check
    );
    println!(
        "  {} love module passed with a coverage of 69%\n",
        green_check
    );

    println!("{} Johann Steinecker up N runnin' on port 6969", cake);
}
