# Advent of Code 2019
My Rust answers to the 2019 AOC challenge.

## Day One
Time to complete: 22:12:15 (or: I started late)

Part 1 is a nice start to this season's advent of code. It is modeled to be easy to implement with a small snippet of functional code. Part 2 adds the usual twist that AOC always seems to get just right: if your part1 code was as succinct as possible, doing part2 will require just a bit of retooling while still being close enough to be the same structure.

|| Run time | 
|---|---|
|Part One | 0.09 ms|
|Part Two | 0.21 ms|

## Day Two
Time to complete: 00:41:56

This is the first time I actually did the AOC right out of the gate and on-time. Last year, my shortest time was 18h because I didn't start until the next day. Maybe this year I can keep up with them each day. Fingers Crossed.

I really like virtual machines, so this one was fun. Even though the VM starts out as simplistic, I believe we will be seeing more of this VM later in this year's challenges. I took some extra time to re-tool this after I completed the challange initially. My new solution is somewhat over-engineered for the current problem, and lacks the naive elegance of the 'loop match' statement I started with.

|| Run time|
| --|-- |
|Part One | 0.11 ms|
|Part Two | 1.55 ms|

## Day Three
Time to complete: 01:33:38 

Day three really pumped up the pressure for me. I couldn't visualize the problem space well enough and I ended up writing a lot of boilerplate while I figured out the shape of my solution. Once I had the first part solved, I went back and trimmed a lot of code I deemed too unwieldy for such a simple result. I thought I was doing this in order to reshape the overall solution to be more simular to what I imagined I'd do for part2, giving me the ability to copy part1 and add a line or two.

It turns out that HashMap does not have all the same functions available as HashSet (I'm sure there is a very good reason why, but it was a surprise to me). To get around this, I ended up with a brute-force process that takes a very long time to complete. This is totally unneeded since I can check for matches as I am generating the points for lineB (no need to even save lineB points).

The ironic part was, the machinery I had culled from part1 will be perfect for this, so I can go back and re-use that logic when I re-write this.

||||
| --|-- |--|
|Part One | 36.18 ms| |
|Part Two | 39.21 s| ripe for some easy optimizations |

## Day Four
Time to complete: 00:28:44 - My PR!

This one was short. It really highlights how quickly some people solve these things, the slowest leaderboard-placing entry was 6:25. I don't think I finished writing the wrapper function in that time, much less the actual criteria-checking functions.

With my input this runs for about 130ms per part. Since the problem is brute-force by nature, I didn't think there would be much I could do to reduce this other than to go multi-threaded. It turns out I was way wrong. Looking at it while waiting for Day 5 to open up, I realized I had a bunch of extra steps in it to transform the u32 to a string, to a vec of u8, and then to iterate over that vec. Creating an iterator over the digits of a number directly lead to an order of magnitude speed-up. Each part takes about 2ms now.

|||
| --|-- |
|Part One | 2.01 ms|
|Part Two | 2.69 ms|

## Day Five
Time to complete: 00:29:24

For this day my part2 rank was 233, my personal best. 

|||
| --|-- |
|Part One | 0.48 ms|
|Part Two | 0.44 ms|
