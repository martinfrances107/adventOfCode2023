# Solutions to AdventOfCode2023

Regarding the [AdventOfCode](https://adventofcode.com/) repository


## A Primer

Here i want to create an assembley of algorithms which are considered best in class.

##  State of the Art Tooling.

"dhat" etc to profile the solutions

## Notes

- Day One - using .sum() and str::to_digits(10)
- Day Two - using str::strip_suffix()
- Day Three - current soltion has lots of duplicate code
            - The goodness of this solution is that it minimised the memory footprint
              only the last, current and next lines are held.
            - The memory foot print could be further minimised by not cloning slice and
              thinking about lifetimes.
            - For performance only the left and right had edge of the sarch window need to
              be checked. Current I am looping over the whole window.
