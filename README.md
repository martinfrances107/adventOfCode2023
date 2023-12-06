# Solutions to AdventOfCode2023

Regarding the [AdventOfCode](https://adventofcode.com/) repository


## A Primer

Here i want to create an assembley of algorithms which are considered best in class.

##  State of the Art Tooling.

"dhat" etc to profile the solutions

## Notes

- Day  1 - using .sum() and str::to_digits(10)
- Day  2 - using str::strip_suffix()
- Day  3 - current soltion has lots of duplicate code
            - The goodness of this solution is that it minimised the memory footprint
              only the last, current and next lines are held.
            - The memory foot print could be further minimised by not cloning slice and
              thinking about lifetimes.
            - For performance only the left and right had edge of the sarch window need to
              be checked. Current I am looping over the whole window.
- Day   4 - The current solution is slow ( 8s (release mode) for part 2 )
              Refactor
                Collect winnings and playable numbers in two HashSets
                precompute and store the result... at present I compute in the
                worst case senario.
                Minimal "Card" should be a list of card offset to "copy"
                Looking using "fold"

Dat 5   5  -   Refator
                Could you take_while to avoid nested looping.
