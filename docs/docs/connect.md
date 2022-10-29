# Connect to the RAPD server


## Default config

| Key         | Value                                |
| ----------- | ------------------------------------ |
| `port`      | ```6702```                           |
| `protocol`  | ```TCP```                            |
| `address`   | ```127.0.0.1```                      |

## Code examples

=== "Rust"

    ``` rust
    use std::io::BufRead;
    use std::io::BufReader;
    use std::io::Write;
    use std::net::TcpStream;

    fn main() {
      // open a tcp stream to the RAPD server, on the default port
      let mut stream = TcpStream::connect("127.0.0.1:6702").expect("Failed to connect");

      // ping the server
      stream
        .write(b"{ \"command\": \"ping\", \"params\": [], \"client_name\": \"e\" }\n")
        .expect("Write failed");

      // Create a buffered reader to read from the stream
      let mut reader = BufReader::new(stream);

      loop {
        let mut line = String::new(); // this will contain the line contents

        reader.read_line(&mut line).expect("Line read failed"); // await the next line sent by the server, and read it into "line"

        // if the line is empty, the server has shutdown
        if line.is_empty() {
          println!("Server sent empty packet, assuming shutdown!");
          std::process::exit(1);
        }

        // print out what we got from the server
        print!("Got from server: {}", &line);
      }
    }

    ```

=== "C++"

    ``` c++
    #include <iostream>

    int main(void) {
      std::cout << "Hello world!" << std::endl;
      return 0;
    }
    ```
