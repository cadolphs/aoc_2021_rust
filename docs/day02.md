# Struct fun

Nothing groundbreakingly new here. One nice thing is the parsing. Anything that has 
FromStr implemented can be parsed directly from a file. So creating the list of commands 
is pretty straightforward.

One thing I'm not 100% sure on is whether the `Add` trait should work on consumed values or 
references. These particular structs here are small so I can just implement `Copy` for them, 
but in general it might be cleaner to work with references.
