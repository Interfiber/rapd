use crate::server::*;

pub fn rebuild() {
    let mut server = RapdServer::new();
    server.connect();

    let cmd = crate::server::RapdCommand::new(String::from("rebuild_database"), vec![]);

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        println!(
            "{}",
            crate::utils::client_error(String::from("Line is empty"))
        );
    } else {
        print!("{}", line);
    }
}

pub fn files() {
    let mut server = RapdServer::new();
    server.connect();

    let cmd = crate::server::RapdCommand::new(String::from("db_get_files"), vec![]);

    server.write_cmd(cmd);

    let line = server.read_line();

    if line.is_empty() {
        println!(
            "{}",
            crate::utils::client_error(String::from("Line is empty"))
        );
    } else {
        print!("{}", line);
    }
}
