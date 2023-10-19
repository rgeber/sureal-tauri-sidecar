// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri::api::process::{Command as TauriCommand, CommandEvent};
use tauri::{Manager, Runtime};

fn main() {

    /// Allow setting a password for surrealdb using the `SURREALDB_PASSWORD` environment variable.
    /// Other options might be cli args, etc. Just make sure to protect your database somehow from
    /// unauthorized access on localhost.
    let surreal_db_password: String = match std::env::var_os("SURREALDB_PASSWORD") {
        Some(p) => p.into_string().expect("Unable to convert value of env var `SURREALDB_PASSWORD` to String."),
        None => "topSecret".into()
    };

    /// Spawn the sidecar process. Only use the bin name, not the whole path.
    /// Args are optional.
    ///
    /// Keep in mind that the process will not be errorous if surreal fails to stay up.
    /// For example if something is wrong with the args surreal will exit but not at this stage.
    /// We catch this with the termination handling further down.
    ///
    /// The args are set to provide a good baseline security. Don't forget that surreal will
    /// talk to ANY client on localhost not just the Tauri app.
    let (mut rx, mut child) = TauriCommand::new_sidecar("surreal")
        .expect("failed to create `surreal` binary command")
        .args([
            "start",
            "--bind",
            "127.0.0.1:8877",
            "--user",
            "root",
            "--pass",
            &surreal_db_password,
            "--auth",
            "--deny-guests",
            "--deny-scripting",
            "--deny-funcs",
            "\"http::*\"",
            "--no-banner",
            "--log",
            "debug",
            "file:/tmp/test.db",
        ])
        .spawn()
        .expect("Failed to spawn sidecar: `surreal`");

    dbg!("Spawned Tauri sidecar process `surreal` with PID: {}", child.pid());
    dbg!("Surreal Root password: {}", &surreal_db_password);

    tauri::async_runtime::spawn(async move {
        // read events such as stdout
        while let Some(event) = rx.recv().await {
            /// Re-print the stdout. A good place to put the output to a dedicated output
            /// file or something along those lines.
            if let CommandEvent::Stdout(line) = &event {
                println!(" ++-> surreal: {line}");
            }

            /// Print pring logs, errors, etc. from surreal to the console
            /// A smarter solution would of course be to dump them into some dedicated
            /// log file. This is the place to do it :)
            if let CommandEvent::Stderr(line) = &event {
                println!(" ++-> surreal: {line}");
            }

            /// React to the tasks' termination. In our case we can't live without the database.
            /// The process panics and that's it. However this could also be used to set some
            /// flag used to inform the frontend about the loss of connection and/or trying to
            /// restart.
            /// This will also capture a failed start (e.g. bad args)
            if let CommandEvent::Terminated(line) = &event {
                println!(" ++-> surreal: terminated.");

                /// Set `panic = 'abort'` in your build profile to make the panic global
                /// If set to unwind it will simply terminate the async task but not the entire application.
                ///
                /// Still looking for a solution that will work with the `unwind` setting.
                ///
                /// Relevant resources:
                /// * https://users.rust-lang.org/t/panic-in-tokio-task-does-not-end-the-program-execution/45731/6
                /// * https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html

                panic!("Surreal went away :'(");
            }
        }
    });

    /// Preparing a state object for the tauri app.
    /// In this case all it stores is the database password.
    ///
    /// More on managed state in Tauri:
    /// https://tauri.app/v1/guides/features/command/#accessing-managed-state
    struct AppState {
        surreal_db_password: String,
    }

    let app_state = Arc::new(
        AppState {
            surreal_db_password: surreal_db_password
        }
    );

    /// Enable front end to access the random database password
    /// Pro tip: Use plugins instead of single commands when working on a real project.
    ///          Keeps everything a lot more organized :)
    ///          More: https://tauri.app/v1/guides/features/plugin/
    #[tauri::command]
    async fn get_surrealdb_password<R: Runtime>(
        app: tauri::AppHandle<R>
    ) -> String {
        let app_state = (*app.state::<Arc<AppState>>()).clone();
        app_state.surreal_db_password.to_owned()
    }

    /// Just running a default Tauri window. Nothing fancy :)
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![get_surrealdb_password])
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
