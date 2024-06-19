# TUP Problem 
_Reinout Annaert & Brecht Van de Sijpe_


This repository contains a Java and Rust implementation to solve the Traveling Umpire Problem optimally.


__Implemented algorithm:__

Toffolo, T. Â. M. (2017). *Branch-and-bound with decomposition-based lower bounds*. In *Decomposition-based algorithms for optimization problems* (Chapter 2.4).


### Implemented Components
**Implementation**
- Branch and Bound Tree search with pruning based on Q1, Q2 & global constraints. Best-first search & fixation to prevent solution rotation.

- Lowerbound calculation:
    - 2-round matching: (2 implementations)
        - 2-deep branch and bound
        - using a matching algorithm
    - Lowerbound strengthening with incremental sub-trees

- Partial matching: (only in the Java version)
    - using a matching algorithm
    - memoization scheme

- Matching algorithms: (check classes for source)
    - Jonker-Volgenant 
    - Hungarian / Munkres


Rust-version: final_version_rust
Java-version: final_version_java


