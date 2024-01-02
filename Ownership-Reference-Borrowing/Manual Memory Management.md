MMM is used in languages such as C
- `malloc` to allocate memeory
- `free` to release or "de-allocate"

Manual management of memory results in unprecedented bugs.

Double Free (CWE-415)—calling the free() function multiple times, resulting in a memory leak. This might allow an attacker to write values to arbitrary memory spaces and create an interactive shell with elevated privileges.

Use After Free (CWE 416)—occurs when a program continues to use a memory pointer after it has been freed. If an attacker manages to overwrite one of those pointers with an address to shellcode, they could execute arbitrary code.

Thus, Rust uses a scope based heurisitc to handle memory.Rust does NOT have a Garbage collector but a concept of ownership based on scopeof a variable.