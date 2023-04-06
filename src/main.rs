use clap::{App, Arg, SubCommand};
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use dirs::home_dir;
use prettytable::{Table, row, cell};

fn main() -> Result<()> {
    let matches = App::new("Note Taker")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("A simple CLI for taking notes")
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a new note")
                .arg(Arg::with_name("note").required(true).index(1)),
        )
        .subcommand(SubCommand::with_name("list").about("List all notes"))
        .subcommand(SubCommand::with_name("delete").about("Delete all notes").arg(Arg::with_name("idx").required(false).index(1)))
        .get_matches();

    let home_dir = home_dir().expect("Couldn't find the user's home directory");
    let db_path = PathBuf::from(home_dir).join("notes.db");
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            note TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    match matches.subcommand() {
        Some(("add", add_matches)) => {
            let note = add_matches.value_of("note").unwrap();
            add_note(&conn, note)?;
        }
        Some(("list", _ )) => {
            //list_notes(&conn)?;
            list_notes_in_a_table(&conn)?;
        }
        // add a command for deleting all notes
        Some(("delete", note )) => {
            let idx = note.value_of("idx");
            match idx {
                Some(idx) => {
                    let idx = idx.parse::<i64>().unwrap();
                    let query = format!("DELETE FROM notes where id = {}", idx);
                    conn.execute(&query, [])?;
                    println!("Note {} deleted.", idx);       
                }
                None => {
                    // delete all notes
                    conn.execute("DELETE FROM notes", [])?;
                    println!("All notes deleted.");
                }
            }
            
            
        }
        _ => {
            eprintln!("Invalid command. Use -h or --help for more information.");
        }
    }

    Ok(())
}

fn add_note(conn: &Connection, note: &str) -> Result<()> {
    conn.execute("INSERT INTO notes (note) VALUES (?1)", [note])?;
    println!("Note added.");
    Ok(())
}

fn list_notes(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, note, created_at FROM notes")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?
        ))
    })?;

    println!("Notes:");
    for row in rows {
        let (id, note, date) = row?;
        println!("[{}] {} -- creation date: {}", id, note, date);
    }
    Ok(())
}

fn list_notes_in_a_table(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, note, created_at FROM notes")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?
        ))
    })?;

    println!("Notes:");
    let mut table = Table::new();
    table.add_row(row!["ID", "Note", "Creation Date"]);
    for row in rows {
        let (id, note, date) = row?;
        table.add_row(row![id, note, date]);
    }
    table.printstd();
    Ok(())
}