# Rust Course FEI solutions

This repository contains the exercises and solutions that some members of the Rust Girona community
have solved of the [Rust Course FEI VŠB-TUO](https://github.com/Kobzol/rust-course-fei).

##  Directory organization

Each directory mimic the [original lesson structure](https://github.com/Kobzol/rust-course-fei/tree/main/lessons).
The sub-directory _exercises_ contains the original non-solved exercises, the sub-directory
_solutions_ (when exists) contains a sub-directory with different solutions solved by some members
of the Rust Girona community.

To keep the content easily accessible this is a copy of the original exercises index with links into
this repository:

1. [01/01 - Factorial](01/exercises/tests/01_factorial.rs) (basic syntax)
2. [01/02 - Find maximum in an array](01/exercises/tests/02_array_max.rs) (cycles)
3. [01/03 - 3D vector](01/exercises/tests/03_struct.rs) (structs)
4. [01/04 - Luhn's algorithm](01/exercises/tests/04_luhn_algorithm.rs) (arithmetics)
5. [01/05 - Match parentheses](01/exercises/tests/05_match_parentheses.rs) (pattern matching)
6. [01/06 - Write tests for a sanitization function](01/exercises/tests/06_write_tests.rs) (writing tests)
7. [02/01 - Perform simple calculation](02/exercises/tests/01_calc.rs) (enums)
8. [02/02 - Match exercises](02/exercises/tests/02_match_exercises.rs) (pattern matching)
9. [02/03 - Computer state transition (structs)](02/exercises/tests/03_state_transition_struct.rs) (structs)
10. [02/04 - Computer state transition (enums)](02/exercises/tests/04_state_transition_enum.rs) (enums, pattern matching)
11. [02/05 - Simple URL validator](02/exercises/tests/05_srl_validator.rs) (newtype pattern, encapsulation, parsing)
12. [02/06 - Brainfuck interpreter](02/exercises/tests/06_brainfuck_interpreter.rs) (enums, parsing)
13. [03/01 - Encryption & decryption](03/exercises/src/encrypt_decrypt.rs) (ownership)
14. [03/02 - Longest string](03/exercises/tests/01_longest.rs) (lifetimes)
15. [03/03 - Strip prefix](03/exercises/tests/02_strip_prefix.rs) (lifetimes, strings)
16. [03/04 - Interleave strings](03/exercises/tests/03_interleave.rs) (strings)
17. [03/05 - Merge sorted slices](03/exercises/tests/04_merge_slices.rs) (slices)
18. [03/06 - Bubble sort](03/exercises/tests/05_bubble_sort.rs) (slices)
19. [03/06 - Zero-copy parsing](03/exercises/tests/06_zerocopy_parsing.rs) (lifetimes in structs, parsing)
20. [04/01 - Shape interface](04/exercises/tests/01_shape.rs) (traits)
21. [04/02 - Case-insensitive comparator](04/exercises/tests/02_case_insensitive_cmp.rs) (traits, iterators)
22. [04/03 - Fibonacci iterator](04/exercises/tests/03_fibonacci.rs) (iterators)
23. [04/04 - Iterator exercises](04/exercises/tests/04_iter_exercises.rs) (iterators)
24. [04/05 - Split items](04/exercises/tests/05_split_items.rs) (iterators)
25. [04/06 - Cumulative sum](04/exercises/tests/06_cumulative_sum.rs) (traits, iterators)
26. [04/07 - Adjacent diff](04/exercises/tests/07_adjacent_diff.rs) (iterators)
27. [04/08 - 1D Range](04/exercises/tests/08_range.rs) (structs, iterators)
28. [04/09 - Poker hand value](04/exercises/tests/09_poker_hand_value.rs) (traits)
29. [04/10 - Ring buffer](04/exercises/tests/10_ringbuffer.rs) (generics, data structure implementation)
30. [05/01 - Binary tree](05/exercises/tests/01_binary_tree.rs) (smart pointers)
31. [05/02 - Parser combinators](05/exercises/tests/02_parser_combinator.rs) (closures)
32. [05/03 - Directed graph](05/exercises/tests/03_graph.rs) (smart pointers, interior mutability)
33. [06/01 - Newtype wrapper](06/exercises/assignments/tests/01_newtype_wrapper.rs) (declarative macros)
34. [06/02 - Hashmap constructor](06/exercises/assignments/tests/02_hashmap_constructor.rs) (declarative macros)
35. [06/03 - Display trait derive](06/exercises/assignments/tests/03_displayme.rs) (procedural macros)
36. [06/04 - Memory map](06/exercises/assignments/tests/04_memory_map.rs) (BTreeMap, algorithmization)
37. [07/01 - Parallel sum](07/exercises/tests/01_parallel_sum.rs) (threads, atomics)
38. [07/02 - Worker queue](07/exercises/tests/02_worker_queue.rs) (threads, channels)
39. [07/03 - Factorio](07/exercises/tests/03_factorio.rs) (threads, channels)
40. [08/01 - TCP/IP network chat using blocking I/O](08/exercises/src/lib.rs) (threads)
41. [09/01 - TCP/IP network chat using non-blocking I/O](09/exercises/src/lib.rs) (epoll, event loop)
42. [10/01 - TCP/IP network chat using async/await](10/exercises/src/lib.rs) (async/await)

## Gratitude

We appreciate [Jakub Beránek](https://github.com/Kobzol) for creating and releasing under an Open
Source license this Rust Course.
with exercises.
