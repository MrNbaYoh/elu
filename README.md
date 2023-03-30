# ELU (EVAL-LINK-UPDATE)
This crate provides traits to describe operations on EVAL-LINK-UPDATE data structures (similar to operations defined by Tarjan in ["Applications of Path Compression on Balanced Trees.”](https://doi.org/10.1145/322154.322161)).  
It also provides implementations of basic EVAL-LINK-UPDATE structures such as forest with path compression on evaluation (see [`CompressedForest`]).

## EVAL-LINK-UPDATE Operations
Suppose we have an associative operation ⊕. The three operations made available on forests are:
- [`EVAL`](EvalLinkUpdate::try_eval)`(n)`: find the root of the tree that contains the node `n`, let say `r`, and compute the product of all values on the path from `r` to `n` (i.e `value(r)` ⊕ ... ⊕ `value(n)`)
- [`LINK`](EvalLinkUpdate::try_link)`(n, m)`: find the root of the tree that contains the node `m`, let say `r`, and link it to the node `n` (i.e `r` becomes a child of `n`)
- [`UPDATE`](EvalLinkUpdate::try_update)`(n, v)`: find the root of the tree that contains the node `n`, let say `r`, and replace its value by `v`
