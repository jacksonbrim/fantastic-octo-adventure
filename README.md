# Purpose
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

##### Playing around with mermaidjs
![Image of class diagram](./mermaid_diagrams/class_uml.svg?sanitize=true)

To run tests
```
cargo test
```

