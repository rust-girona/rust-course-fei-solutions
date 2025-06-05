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

1. [Factorial](01/exercises/tests/01_factorial.rs) (basic syntax)
2. [Find maximum in an array](01/exercises/tests/02_array_max.rs) (cycles)
3. [3D vector](01/exercises/tests/03_struct.rs) (structs)
4. [Luhn's algorithm](01/exercises/tests/04_luhn_algorithm.rs) (arithmetics)
5. [Match parentheses](01/exercises/tests/05_match_parentheses.rs) (pattern matching)
6. [Write tests for a sanitization function](01/exercises/tests/06_write_tests.rs) (writing tests)
7. [Perform simple calculation](02/exercises/tests/01_calc.rs) (enums)
8. [Match exercises](02/exercises/tests/02_match_exercises.rs) (pattern matching)
9. [Computer state transition (structs)](02/exercises/tests/03_state_transition_struct.rs) (structs)
10. [Computer state transition (enums)](02/exercises/tests/04_state_transition_enum.rs) (enums, pattern matching)
11. [Simple URL validator](02/exercises/tests/05_srl_validator.rs) (newtype pattern, encapsulation, parsing)
12. [Brainfuck interpreter](02/exercises/tests/06_brainfuck_interpreter.rs) (enums, parsing)
13. [Encryption & decryption](03/exercises/src/encrypt_decrypt.rs) (ownership)
14. [Longest string](03/exercises/tests/01_longest.rs) (lifetimes)
15. [Strip prefix](03/exercises/tests/02_strip_prefix.rs) (lifetimes, strings)
16. [Interleave strings](03/exercises/tests/03_interleave.rs) (strings)
17. [Merge sorted slices](03/exercises/tests/04_merge_slices.rs) (slices)
18. [Bubble sort](03/exercises/tests/05_bubble_sort.rs) (slices)
19. [Zero-copy parsing](03/exercises/tests/06_zerocopy_parsing.rs) (lifetimes in structs, parsing)
20. [Shape interface](04/exercises/tests/01_shape.rs) (traits)
21. [Case-insensitive comparator](04/exercises/tests/02_case_insensitive_cmp.rs) (traits, iterators)
22. [Fibonacci iterator](04/exercises/tests/03_fibonacci.rs) (iterators)
23. [Iterator exercises](04/exercises/tests/04_iter_exercises.rs) (iterators)
24. [Split items](04/exercises/tests/05_split_items.rs) (iterators)
25. [Cumulative sum](04/exercises/tests/06_cumulative_sum.rs) (traits, iterators)
26. [Adjacent diff](04/exercises/tests/07_adjacent_diff.rs) (iterators)
27. [1D Range](04/exercises/tests/08_range.rs) (structs, iterators)
28. [Poker hand value](04/exercises/tests/09_poker_hand_value.rs) (traits)
29. [Ring buffer](04/exercises/tests/10_ringbuffer.rs) (generics, data structure implementation)
30. [Binary tree](05/exercises/tests/01_binary_tree.rs) (smart pointers)
31. [Parser combinators](05/exercises/tests/02_parser_combinator.rs) (closures)
32. [Directed graph](05/exercises/tests/03_graph.rs) (smart pointers, interior mutability)
33. [Newtype wrapper](06/exercises/assignments/tests/01_newtype_wrapper.rs) (declarative macros)
34. [Hashmap constructor](06/exercises/assignments/tests/02_hashmap_constructor.rs) (declarative macros)
35. [Display trait derive](06/exercises/assignments/tests/03_displayme.rs) (procedural macros)
36. [Memory map](06/exercises/assignments/tests/04_memory_map.rs) (BTreeMap, algorithmization)
37. [Parallel sum](07/exercises/tests/01_parallel_sum.rs) (threads, atomics)
38. [Worker queue](07/exercises/tests/02_worker_queue.rs) (threads, channels)
39. [Factorio](07/exercises/tests/03_factorio.rs) (threads, channels)
40. [TCP/IP network chat using blocking I/O](08/exercises/src/lib.rs) (threads)
41. [TCP/IP network chat using non-blocking I/O](09/exercises/src/lib.rs) (epoll, event loop)
42. [TCP/IP network chat using async/await](10/exercises/src/lib.rs) (async/await)

## Gratitude

We appreciate [Jakub Beránek](https://github.com/Kobzol) for creating and releasing under an Open
Source license this Rust Course.
with exercises.
