use leviticus::{
    cli::command::{Command, parse},
    daemon::{reload, run::start, status, stop},
    projection::{document, view},
};

#[tokio::main]
async fn main() {
    let cli = parse();

    match cli.command {
        // -------------------------
        // DAEMON LIFECYCLE
        // -------------------------
        Command::Start => {
            start().await;
        }

        Command::Status => {
            status::status().await;
        }

        Command::Stop => {
            stop::stop();
        }

        Command::Reload => reload::reload(),

        // -------------------------
        // VIEW SYSTEM
        // -------------------------
        Command::View { name } => {
            view::set_active(name);
        }

        Command::ViewFork { name } => {
            view::fork(name);
        }
        Command::ViewList => {
            view::list();
        }

        // -------------------------
        // EXPLAIN / DOCS SYSTEM
        // -------------------------
        Command::Explain => {
            document::generate_explain_doc().unwrap();
        }

        Command::ExplainDoc => {
            document::open_explain_doc();
        }

        // fallback
        _ => {
            println!("Unknown command");
        }
    }
}
