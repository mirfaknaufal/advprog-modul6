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