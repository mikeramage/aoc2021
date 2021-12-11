# aoc2021
Advent of Code 2021

# Day 1
So here we are again: a new season of Advent of Code, some more horrible code, another stupid README. For the first challenge I actually wrote some Rust. No way! Probably Rust that looks more like Python or C, but Rust nonetheless. Extrapolating based on how long it took me to do this one with all the guddling about in the Rust book and API documentation, wrestling with the borrow checker, etc., I reckon I'll need about 100 hours for day 18. Sigh. Anyone want to wager how long it takes me to revert back to Python? 

# Day 2
My eyeballs are throbbing gently and the gentle tinnitus of exhaustion rings faintly in my ears. It's done. I ended up plagiarising Chris's framework for running the tests. Thanks Chris. I only meant to take a little inspiration from it, but it ended up basically the same. I didn't cheat on the answer in any way - that Rust is mine all mine (as you can probably tell from the quality). At this stage it takes about a minute to solve the problem and 3 hours struggling with the ampersands, mutability, trait bounds, etc. etc. Hopefully that balance will have reversed by Christmas. Though it's a pretty slim hope. Anyway, time for bed.

# Day 3
What vile nastiness is this? Grim code for a grim problem. Back in the mists of time almost before the dawn of memory I can see a distant vision of myself not wrestling with the compiler. But it is done. And I have two stars. Do the ends justify the means? Not really. 

# Day 4
What's with the damn rust compiler? It's patronising arrogance makes me feel like a simpleton. Every time I try to compile the code, up it pops:"You think you're clever? Ha, ha, ha. You think you can write code? Oh, no no no no, my dear boy! Let me explain to you what you did wrong. I will demonstrate the many reasons why you should not be writing code". 

After a few minutes of consideration I came up with what I thought was a decent enough design. Turned out it was, more or less. But would Rust let me express it in code in fewer than 4 hours? Nope, just one "tut tut - you've borrowed an &&String when *obviously* you need a mut &str, moron" after another. 

Python beckons me down from the ivory tower with its siren call of cheap and filthy simplicity. "With me you can just bash out a simple little script that just does the job any way you like. I won't criticise. I won't judge. Come to me. You know you want to." No, Python, *no*!  I will satisfy the borrow checker. I will satsify the trait bounds. I will appease the demands of the compiler. I will make clippy happy. I will *not* give in to your cheap and superficial charms! I will achieve the purity of unsullied Rust or die in the attempt!

# Day 5
My solution is, frankly, disgusting. I do not recommend reading this code if you value your eyesight or want to keep your lunch down. Ironically, the Rust compiler was pretty forgiving this time - maybe it just went off in a supercilious huff when it saw the state of what I was trying to run past it. "Yeah, mate, whatever, I'll compile it. Just don't ask me to look at your filthy code ever again. Ever." I couldn't even be bothered to write it in a way that produced an answer to part 1 and part 2 in the same pass. 

But it worked, you hear me!? It worked!

# Day 6
Turns out iterating through an array of billions of entries is quite time-consuming. So I had to write two programs. Perhaps the word "exponential" should have set off some alarms. Ah well. At least the compiler didn't moan too much. Am I getting better, or did I just get lucky? 

# Day 7
Meh. I'm sure there's a more elegant algorithm to calculate this, but things to do, people to see. And hey, Rust, why do you have to make simple maths so awkward?

# Day 8
There are probably several ways to do implement a solution to this problem well. This is not one of them.