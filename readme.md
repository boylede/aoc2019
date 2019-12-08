# Advent of Code 2019
My Rust answers to the 2019 AOC challenge.

## Day One: Fuel Economy
Time to complete: 22:12:15 (or: I started late)

Part 1 is a nice start to this season's advent of code. It is modeled to be easy to implement with a small snippet of functional code. Part 2 adds the usual twist that AOC always seems to get just right: if your part1 code was as succinct as possible, doing part2 will require just a bit of retooling while still being close enough to be the same structure.

|| Run time | 
|---|---|
|Part One | 0.09 ms|
|Part Two | 0.21 ms|

## Day Two: VM #1
Time to complete: 00:41:56

This is the first time I actually did the AOC right out of the gate and on-time. Last year, my shortest time was 18h because I didn't start until the next day. Maybe this year I can keep up with them each day. Fingers Crossed.

I really like virtual machines, so this one was fun. Even though the VM starts out as simplistic, I believe we will be seeing more of this VM later in this year's challenges. I took some extra time to re-tool this after I completed the challange initially. My new solution is somewhat over-engineered for the current problem, and lacks the naive elegance of the 'loop match' statement I started with.

|| Run time|
| --|-- |
|Part One | 0.11 ms|
|Part Two | 1.55 ms|

## Day Three: Tangled Lines
Time to complete: 01:33:38 

Day three really pumped up the pressure for me. I couldn't visualize the problem space well enough and I ended up writing a lot of boilerplate while I figured out the shape of my solution. Once I had the first part solved, I went back and trimmed a lot of code I deemed too unwieldy for such a simple result. I thought I was doing this in order to reshape the overall solution to be more simular to what I imagined I'd do for part2, giving me the ability to copy part1 and add a line or two.

It turns out that HashMap does not have all the same functions available as HashSet (I'm sure there is a very good reason why, but it was a surprise to me). To get around this, I ended up with a brute-force process that takes a very long time to complete. This is totally unneeded since I can check for matches as I am generating the points for lineB (no need to even save lineB points).

The ironic part was, the machinery I had culled from part1 will be perfect for this, so I can go back and re-use that logic when I re-write this.

||||
| --|-- |--|
|Part One | 36.18 ms| |
|Part Two | 39.21 s| ripe for some easy optimizations |

## Day Four: Password Rules
Time to complete: 00:28:44 - My PR!

This one was short. It really highlights how quickly some people solve these things, the slowest leaderboard-placing entry was 6:25. I don't think I finished writing the wrapper function in that time, much less the actual criteria-checking functions.

With my input this runs for about 130ms per part. Since the problem is brute-force by nature, I didn't think there would be much I could do to reduce this other than to go multi-threaded. It turns out I was way wrong. Looking at it while waiting for Day 5 to open up, I realized I had a bunch of extra steps in it to transform the u32 to a string, to a vec of u8, and then to iterate over that vec. Creating an iterator over the digits of a number directly lead to an order of magnitude speed-up. Each part takes about 2ms now.

|||
| --|-- |
|Part One | 2.01 ms|
|Part Two | 2.69 ms|

## Day Five: VM #2 Immediates & Conditionals
Time to complete: 00:29:24

Return to VMs! For this day my part2 rank was 233, my personal best. This one was mostly just adding to the VM and validating that it functions as advertised. I found a bug after completing the challenge, fortunately it didn't affect my answers but I wasn't using the new parameter mode code in one place so it always treated it as an address rather than an immediate.

|||
| --|-- |
|Part One | 0.12 ms|
|Part Two | 0.11 ms|

## Day Six: Orbital Pathfinding
Time to complete: 00:27:47

This one was fun but it took me longer than needed because I got messed up near the end trying to combine the two orbital paths and I couldn't remember if they were going need me to trim them before combining or re-add a removed element. It was essentially a mental off-by-one error. 

My part1 time was my best yet, at 00:12:28.

|||
| --|-- |
|Part One | 3.71 ms| 
|Part Two | 0.29 ms|

## Day Seven: Connected VMs
Time to complete: >24h

Returning to my love of VMs, I was excited to see we would be hooking them together today. There was a reddit post the day before which had predicted we would end up networking the VMs, so I had already gone ahead and make the machines block when they didn't have input that they needed. The next step was just to add some flexibility to allow each machine's IO queue to be passed on to the next machine.

In theory, this could work just by popping the last value but I wasn't sure that the code running on the machines would output exactly one number for every one number of input. I decided to wrap my queues in an Option<> so that I can make use of the .take() method to switch the queue from one machine to the next. This added a lot of checking all throughout the code which in hindsight probably wasn't worth the time. I also switched my queues to use a VecDeque instead of a plain Vec, because it provides both front/back push and pops. 

In a future iteration of this VM code I'd like to add better (automatic?) networking, so that you have a Network object which handles the queue passing.

I started this challenge the following day, and it took much longer than I expected. The VM part was relatively straightforward, but I had a bug in the code which generated my phase inputs and this affected my part2 results. My part1 results didn't need the missing values so I didn't notice for a while that I was missing 50% of the possible input space. I struggled for a while to make it work and eventually fell back on a solution that works but isn't as pretty as I was hoping for.

|||
| --|-- |
|Part One | 0.42 ms|
|Part Two | 0.83 ms|

## Day Eight: Satellite imagery
Time to complete: 02:01:34

This one was refreshing. It was relatively simple, but I had a bug in my part2 code which gave me a skewed output image. I couldn't find the bug and ended up re-writing the code from scratch. For each challenge I've gone back afterwards and cleaned up my code, usually writing it in a more functional style if possible. It is a lot easier to read functional code but it isn't always easy to visualize the problem this way so I often write the code that makes most sense and then go back and clean up the extra steps I inevitably took.

In this case, I did the re-write before completing the challenge and it managed to fix my bug. I think I was using the .windows() method incorrectly which I later fixed by applying .step_by(). 

|||
| --|-- |
|Part One | 0.15 ms|
|Part Two | 0.92 ms|

