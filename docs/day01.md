# Iterator fun
This would be so easy with Python :)

But we're using Rust and we try to do it idiomatically. Already I learned a few things.

For reading files where each line is just one particular value, I wanted to make a nice function that 
does the parsing for me. Rusts type inference can be quite powerful here: Whatever you can parse into 
a string, you can now read from a file on a one-item-per-line basis. 

I was considering returning an iterator instead of a vec but decided against it. The memory footprint 
would be smaller, but only if we'd also _read_ from the file line by line instead of loading the 
whole file into memory as a string in the first place.

Fun was had trying to use the generic error. This `Ok(line.parse()?)` idiom is worth remembering.

The part 2 of day 1 could have been written much more easily by just iterating over size-3-windows 
operating on a slice. But that involves a lot of inefficient extra copying. To compute the 
cumulative windowed triplet sum, you really only need to compute the sum of the first three elements, 
and then at each new step you need to just drop the element from one end and attach the element from 
the other end. Just for fun I wanted to see how one would write a custom iterator for this.

Then some extra fun was had trying to have this triplet sum creation written as a trait rather than 
a standalone function that would take a vector or slice as argument. Initially I implemented this 
for `&[i32]` but somehow that made it so the test module was confused and considered the trait 
not implemented for the test data, which was of type `Vec<i32>`. Using a generic type with the 
`AsRef<[i32]>` type bound worked like a charm,
