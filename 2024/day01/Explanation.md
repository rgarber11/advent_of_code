# Explanation of Day 1 Code

## Part 1

Going back, I probably shouldn't be hard-coding file paths into my solutions. I also should probably actually check if `charconv` is successful. Beyond aesthetic and usability concerns, this solution is really nice. Using `std::ranges`, we get a very declarative piece of code that accumulates over a thin-view.

## Part 2

At least I make some amount of effort in safety with the assert. Here, the changes I made include moving into the pair. I'm not sure if this is necessary, though I remember the godbolt output looking better. Afterwards, a hashmap keeps track of right occurrences (maybe I should set up cspell), and a similar left fold makes the answer.
