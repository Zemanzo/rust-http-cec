use warp::reply::WithStatus;
use warp::{Filter, reply};
use warp::http::StatusCode;
use std::process::{Command, Stdio};
use std::io::Write;

fn shell(cmd: &[u8]) -> WithStatus<String> {
    if cfg!(target_os = "windows") {
        println!("Windows is not supported");
        return reply::with_status("INTERNAL_SERVER_ERROR".to_string(), StatusCode::METHOD_NOT_ALLOWED)
    } else {
        // println!("{}", cmd);

        let mut child = Command::new("cec-client")
            .arg("-s")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        let child_stdin = child.stdin.as_mut().unwrap();
        child_stdin.write_all(cmd).expect("failed to execute process");

        // Close stdin to finish and avoid indefinite blocking
        drop(child_stdin);
    };

    reply::with_status("Success!!".to_string(), StatusCode::OK)
}

fn temp() -> WithStatus<String> {
    if cfg!(target_os = "windows") {
        println!("Windows is not supported");
        return reply::with_status("INTERNAL_SERVER_ERROR".to_string(), StatusCode::METHOD_NOT_ALLOWED)
    } else {
        let output = Command::new("vcgencmd")
            .arg("measure_temp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute process");

        let stdout = String::from_utf8(output.stdout).unwrap();

        println!("{}", stdout);

        return reply::with_status(stdout, StatusCode::OK)
    };
}

#[tokio::main]
async fn main() {
    // GET /
    let root = warp::path::end().map(|| "Hello, World at root!");

    // GET /command
    let command = warp::path!("command" / String).map(|command_name: String| {
        match command_name.as_str() {
            "on" =>  shell(b"on 0.0.0.0"),
            "standby" => shell(b"tx 10:44:40"),
            "temp" => temp(),
            _ => reply::with_status("INTERNAL_SERVER_ERROR".to_string(), StatusCode::METHOD_NOT_ALLOWED),
        }
    });

    let routes = warp::get().and(
        root
            .or(command)
    );

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
