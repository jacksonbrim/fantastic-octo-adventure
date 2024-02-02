# Unit Conversion & Dice Game Simulation
Cli usage:


Convert 10 meters to inches
Convert 10 hours to minutes 
```sh
cargo run -- -u 10 m in
cargo run -- -u 10 hr min
```

Run Dice Game Simulation 100 times
***Dice Game Simulation will save results to ./output/<n
results>_simulations/%YYYY-%MM-%DD_%h_%m_%s.txt***
```sh
cargo run -- -n 100
```
With logs
```sh
cargo run -- -n 100 -v
```
Results output to custom filepath
```sh
cargo run -- -n 100 -o ./hello_world.txt
```
If the program is ended early with Ctrl^C...
The output file will be modified such that the early termination is
noted and the filename includes the number of simulations completed.
```sh
cargo run -- -n 100 -o ./hello_world.txt
ls ./hello*
>>> hello_world_early_exit_992_simulations_completed.txt
```


### Purpose
I came across a [video of a Jane Street Mock Interview](https://www.youtube.com/watch?v=V8DGdPkBBxg), and was curious
to implement it in Rust. The purpose of the problem wasn't to show off
DSA, but to demonstrate talking through a code exercise with an
interviewer. 

### Problem
Given the facts, write a program to correctly answer the following
queries. 
'''
example facts:
* m = 3.28 ft
* ft = 12 in
* hr = 60 min
* min = 60 sec
example queries:
* 2m = ? in   ---> answer = 78.72
* 13 in = ? m ---> answer = 0.330 (roughly)
* 13 in = ? hr ---> "not convertible!"
'''

#####Dice Game Strategy Simulation
Implementation of part 1 of this problem on [Jane Street's
youtube](https://www.youtube.com/watch?v=NT_I1MjckaU).
######Strategy 1:
Roll if the value of the current die-result is less than the sum of the
expected values of all die results greater than the current result for
the current turn.
![Dice Game: Strategy 1 State Flow](./mermaid_diagrams/dice_game_strat_1.svg?sanitize=true)
######Strategy 2:
Roll if the value is less than 20.
![Dice Game: Strategy 2: 20s Only State Flow](./mermaid_diagrams/dice_game_strat_2.svg?sanitize=true)
####Unit Conversion Class Diagram
![Unit Conversion Class Diagram](./mermaid_diagrams/class_uml.svg?sanitize=true)



To run tests
```
cargo test
```

