// Test `exec` function

--- exec-success eval ---
// Test success
#let out = exec("echo 'Hello, World!'")
#test(out, "Hello, World!\n")

--- exec-failure eval ---
// Test failure (error)
// Error: 7-14 script failed with exit status exit status: 1
#exec("false")

--- exec-stderr paged ---
// Test stderr (warning)

// Warning: 7-32 script produced stderr: test warning\n
#exec("echo 'test warning' >&2")

--- exec-raw eval ---
// Test raw content (script in triple backticks)
#let out = exec(```
echo 'Hello from raw!'
```)
#test(out, "Hello from raw!\n")
