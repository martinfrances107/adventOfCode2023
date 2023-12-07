# Solutions to AdventOfCode2023

Regarding the [AdventOfCode](https://adventofcode.com/) repository

## A Primer

Here i want to create an assembley of algorithms which are considered best in class.

##  State of the Art Tooling.

"dhat" etc to profile the solutions

## Notes

- Day  1 - using .sum() and str::to_digits(10)
           part 2
           I delayed until a good solution could be found.
           A clean implementation uses .find and rfind() -- reverse find.
           Also of intrest I collected into a BTree so I could extract the
           last and the first.

- Day  2 - using str::strip_suffix()

- Day  3 - Current soltion has lots of duplicate code
            - The goodness of this solution is that it minimised the memory footprint
              only the last, current and next lines are held.
            - The memory foot print could be further minimised by not cloning slice and
              thinking about lifetimes.
            - For performance only the left and right had edge of the sarch window need to
              be checked. Current I am looping over the whole window.

- Day  4 - The current solution is slow ( 8s (release mode) for part 2 )
           Needs Refactor
                Collect winnings and playable numbers in two HashSets
                precompute and store the result... at present I compute in the
                worst case senario.
                Minimal "Card" should be a list of card offset to "copy"
                Looking using "fold"

- Day  5 - Needs Refator
              Could you take_while to avoid nested looping.
              part 2 is tricky
              Good use of .chunks(2) to split the seeds int pairs of numbers.
              - The only optimisation currently is to treat the seed blocks
              as a "batch lot" compute the min and then push that min forward and then find
              the mins of all the candiate mins. Otherwise the memory footprint get out of hand.
              Currently runs in 3m41sec ( --release )
               -- must remove all .collect() calls and remove memory usage.
               -- Use rayon .. and compute all seed blocks in parallel?

- Day  6 - Part 1:  An good example of a small memory footprint.
            Two iterators are ".zip()ed' and fed into a .product() call with out any
            intermediate value ever being ".collected()".
            Part 2: Just demonstates that u32 overflow.
