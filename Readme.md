Solutions to https://adventofcode.com/ for 2021.

My best shot at attempting to show that Rust is suitable for small algorithmic problems.

Notes:
- You might notice my code is full of `unwrap()`; this is intentional. Although this practice is generally frowned upon on production code,
I consider it *best practice* on run-once programs. On one hand, these invocations to `unwrap()` would only manifest in case of a bug. On
the other hand, the panic stacktrace is really handy (no pun intended) when debugging.
