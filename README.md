<details>
<summary>Commit 1 Reflection notes</summary>

1. Handling TCP Streams with BufReader
TcpStream wrapped inside a BufReader to simplify reading the incoming stream line by line. This makes it easier to parse the HTTP request headers since HTTP headers are line-separated text.

2. Collecting the HTTP Request
`.lines()` returns an iterator of each line from the stream.
- The `map(|result| result.unwrap())` part forcefully unwraps each `Result<String, Error>`, assuming the read will succeed.
- The `take_while(|line| !line.is_empty())` stops reading once an empty line is encountered, which corresponds to the end of HTTP headers in a typical request.

3. Printing the HTTP Request
By collecting the lines into a `Vec<String>`, full HTTP request headers are printed out with `println!()`.
</details>

<details>
<summary>Commit 2 Reflection notes</summary>

1. Serving a Static HTML File
By adding `fs::read_to_string("hello.html")`, the program now can serve a static html file.

2. Raw HTTP Response
``` Rust
let status_line = "HTTP/1.1 200 OK";
let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
```
This reinforced how HTTP responses are structured, starting with the status line, followed by the headers, and then the body separated by `\r\n\r\n`.

Screen capture:
![Commit 2 Screen Capture](assets/images/commit2.png)
</details>