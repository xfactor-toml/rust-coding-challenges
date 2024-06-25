[Solve Yourself Here](https://www.hackerrank.com/challenges/dynamic-programming-classics-the-longest-common-subsequence/problem?isFullScreen=true)

# Dynamic Programming

## Theorem
```rust
let c[i, j] = if x[i] == y[j] {
    c[i - 1, j - 1] + 1
} else {
    max(c[i - 1, j], c[i, j - 1])
}
```
