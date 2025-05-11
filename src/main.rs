use figlet_rs::FIGfont;
use qbfrt::config::Config;
use qbfrt::db::{dump_db, save_path, tracker_url, DB};
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", FIGfont::standard().unwrap().convert("qbfrt").unwrap());

    let config = Config::build().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    DB::backup(&config).unwrap_or_else(|err| {
        println!("Could not backup database: {err}");
        process::exit(1);
    });

    let db = DB::connect(&config).unwrap_or_else(|err| {
        println!("Could not connect to database: {err}");
        process::exit(1);
    });

    if let Some(save_path) = &config.save_path {
        save_path::change_save_path(&db, save_path, &config).unwrap_or_else(|err| {
            println!("Could not update save paths: {err}");
            process::exit(1);
        });
    }

    if let Some(tracker_url) = &config.tracker_url {
        tracker_url::change_tracker_url(&db, tracker_url, &config).unwrap_or_else(|err| {
            println!("Could not update tracker URLs: {err}");
            process::exit(1);
        })
    }

    if config.db_to_fastresume {
        dump_db::to_fastresume(&db, &config).unwrap_or_else(|err| {
            println!("Could not dump database to fastresume files: {err}");
            process::exit(1);
        });
    }

    Ok(())
}
