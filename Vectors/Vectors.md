Vector and arrays are a lot different when compared to from a memory perspective.

Vectors being dynamic(ability to add elements to a vector) - is stored in heap where as arrays elements are stored directly ontp the stack

When a vector is created, its size and capacity are zero. When we perform operations like the push() function, the vector’s size increments by 1. With the pop() and remove() functions, the size of the vector decrements by 1.

Things are different when it comes to the capacity of the vector. When we use push() the data, the vector checks if its size is equal to its capacity. If they are equal, then the vector updates its capacity by a factor of 2. However, the capacity of the vector remains the same when we are using the pop() or the remove() function.

A vector occupies space even after popping or removing all of its elements since it is able to retain its capacity. This may cause memory shortages for future operations. Let’s see this in the following example.

https://github.com/rust-lang/rust/blob/master/library/std/src/alloc.rs