# Day 3 - Binary fun
This would be easy enough to do with "brute force" string processing.

One quick observation: If we assume that the total number of rows is uneven, 
there can't ever be ties for "most common / least common". And in that case, 
we _only_ need to compute the `gamma rate` (i.e. the most common bit of each position);
the `epsilon rate` is then simply the 2s-complement, i.e. a bitwise negate.

Now how to do the "counting". In Python I'd just load the string into a numpy matrix. 
It's actually fine for now to leave it as a string. Then we can accumulate all the 
positions in one go, right?

Trying to do a dead-simple iterator here.

So for part 1 I'm doing a non-iterator accumulation. It's just more idiomatic to 
avoid closures with too much mutability going on.

For part 2, we're using the partition function of iterators. Nothing wild here.