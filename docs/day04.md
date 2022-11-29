# Day 04
Okay, this is mostly about encoding board state in a nice way. Also the reading. Let me first use this as a scratch pad to go through my thought process.

The "game loop" will be fetching a number and then telling all the boards that they need to update them. There's the _marking_ of the numbers, and then there's 
the _checking_ of the rows / columns. So, how to represent the board? Maybe the idea is to have _two_ low-level data structures. One structure will tell you if 
the board has the number, and if so, in which field. The other structure will keep track of what's marked. So, if we say `board.mark_number(5)` internally the 
`Board` struct would look up (hash-map; though it is integers so no hash required. Anyway. The hashmap tells you then the board cooardinates. Then we tell the 
marked-array that it has been marked. We can then also efficiently figure out if the board is a winner because only that row or col that was just added needs to be checked.

Or, even easier, if we _assume_ correct boards, all we need to do is keep track of _how many entries_ we've had in a given column. Hah. Oh wait. For the check-sum 
computation we need to know which numbers are marked / unmarked. So there. Betterr to _honestly_ represent the state of the game, not optimize prematurely until we 
also know part b) of the challenge.