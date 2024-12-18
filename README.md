# Solutions to AdventOfCode2023

Regarding the [AdventOfCode](https://adventofcode.com/) repository

## A Primer

Here i want to create an assembley of algorithms which are considered best in class.

##  State of the Art Tooling.

"dhat" Criterion etc to profile the solutions

## Notes

- Day  1 - Trebuchet - corrupt calibration data.
          using .sum() and str::to_digits(10)
           part 2
           I delayed until a good solution could be found.
           A clean implementation uses .find and .rfind() -- reverse find.
           Also of intrest I collected into a BTree so I could extract the
           last and the first.

- Day  2 - RGB cubes game
            using str::strip_suffix()

- Day  3 - Gear Ratios
            Current soltion has lots of duplicate code
            - The goodness of this solution is that it minimised the memory footprint
              only the last, current and next lines are held.
            - The memory foot print could be further minimised by not cloning slice and
              thinking about lifetimes.
            - For performance only the left and right had edge of the sarch window need to
              be checked. Current I am looping over the whole window.

- Day  4 - Scratch Cards
          The current solution is slow ( 8s (release mode) for part 2 )
           Needs Refactor
                Collect winnings and playable numbers in two HashSets
                precompute and store the result... at present I compute in the worst case senario.
                Minimal "Card" should be a list of card offset to "copy"
                Looking using "fold"

- Day  5 - Almanac as a multi-map planting shedule
              Needs Refator
              Could you take_while to avoid nested looping.
              part 2 is tricky
              Good use of .chunks(2) to split the seeds int pairs of numbers.
              - The only optimisation currently is to treat the seed blocks
              as a "batch lot" compute the min and then push that min forward and then find
              the mins of all the candiate mins. Otherwise the memory footprint get out of hand.
              Currently runs in 3m41sec ( --release )
               -- must remove all .collect() calls and remove memory usage.
               -- Use rayon .. and compute all seed blocks in parallel?

- Day  6 - Toy Racing game ( button hold duration )
            Part 1: An good example of a small memory footprint.
            Two iterators are ".zip()ed' and fed into a .product() call with out any
            intermediate value ever being ".collected()".
            Part 2: Just demonstates that u32 overflow.

- Day  7 - Camel cards
          Part1 Refactor PartialOrd is verbose
           I think I can find a 5 lines replacement for it.
           Itertools.counts() creates a histogram I clumsily implemented
           the histogram.

- Day  8 - L System

- Day  9 - Data Extrapolation extercise.

           Part1 Refactor,
           For these toy examples in it not a problem, but I am consuming way too much memory. [ it does not scale for large lines of numbers.]
           IterTools -- I could use a tuple_window() to have a sliding window
           of value(i) and value(i-1) useful when creating the diff.
           Then I can store only the latest diff.

Day 10 - Pipe puzzle

        Needs major recator to get part1!

        |
        FJ

        have identified this as the pathalogical case
        from J to F should be downwards, but up is taken

        Need to add a method to at the start inspect the s tile
        and return the ports...

        then refactor the massive switch statement.

Day 11 - Galaxy map
        Good use of filter_map
         Refactor -- .expand() had a double loop I could remove.

Day 13 - Ash Mirrors

        part 1 is failing ...
        There are some more tests that I can implement:-
        I need to alter rows and columns at the edges which are not checked
        for mirror becasue the other "walker" has already met an edge.

        Not sure, as my current answer is too low and this would
        only drop the score further.

        I think the next step is to display the horizontal/vertical brackets
        on the puzzle as they are processed.

Day 14 - Rolling rocks
      part 1 passes.
      part 2 operated over 10^9 spin cycles.
      I need to make the solution generic for the N, W, S, E
      antilock-wise rolling.
      Can I merge the four steps into one?

Day 15 - Hash algorithrm
      part 2 in a good examples of CRUD operations when using a VecDeque

      Good example of using Wrapping<u8> to create a hash.
## Observations

Using "Itertools" as a example of good library code

functions that use std function like HashMap should signal to the compiler that they do. "use_std"

```rustlang
    #[cfg(feature = "use_std")]
    fn counts(self) -> HashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        let mut counts = HashMap::new();
        self.for_each(|item| *counts.entry(item).or_default() += 1);
        counts
    }
```

How easy it is, using std functions to create permulations and combinations from a list.

".boundaries" is a useful thing.