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
<details>
<summary>Commit 3 Reflection notes</summary>

1. Why Refactoring Was Needed
Initially, every response (either for 200 or 404) got the same page. Therefore, we need to refactor the code to accomodate new logics.
Refactoring the code helps by making the code easier and making future changes simpler.

2. How was the logic split
We introduced a new pattern:
```Rust
let (status_line, contents) = if request_line.contains("GET / ") {
        ("HTTP/1.1 200 OK", fs::read_to_string("hello.html").unwrap())
    } else {
        ("HTTP/1.1 404 NOT FOUND", fs::read_to_string("404.html").unwrap())
    };
```
This logic routes response to their own page.

Screen capture:
![Commit 3 Screen Capture](assets/images/commit3.png)
</details>