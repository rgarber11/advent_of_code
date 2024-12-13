# Day 13

A claw machine puzzle that is an excuse to solve a system of linear equations. Overall, pretty fun. It might've been a bit more efficient to chunk lines and return a streaming Iterator. That way the entire file wouldn't have to be read into memory. For this AoC problem, however, the inputs are small enough for this to not make a big difference. I've copied inputs with no additional Blank lines at the end, but my parser supports 0 or 1 blank lines at the end.

## Part 1

For Part 1, where a straight solution is needed, I used Cramer's rule to determine the unknowns. As a closed solution, its very easy to code, and pretty easy to parse. Stack Overflow suggested `ans_2 == ans2.trunc()` as a performant way to check whether something is an integer. A question I have, since I haven't seen Part 2 yet, is whether I filtered at the correct point. So, I have the check for `ans_1 > 100` within the per-claw machine code, while I have the token values outside within the `part1` function. Another interesting question that Godbolt could answer is whether `.map().sum()` is equivalent to `.fold()`. Stack Overflow suggests yes.
