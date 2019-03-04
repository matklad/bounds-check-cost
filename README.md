# Measuring cost of a bounds check

This crate contains a couple benchmarks which measure the cost of bounds
checking. The main hypothesis is that *direct* cost of a bounds check is
negligible, because it's a trivially predicted branch, but indirect costs can be
high due to missed optimization opportunities. 

`sum_indirectly` and `sum_indirectly_unchecked` functions sum `xs` array, using
indices from the helper `indexes` array. Because indexing is indirect, compiler
can't vectorize the summation, so here we measure direct cost of a bounds check.


`sum` and `sum_unchecked` functions sum `xs` array, using `for i in 0..n` style
loop. Here, compiler can auto-vectorize, if it can prove that access is always
in bounds. Because we pass the summation range externally, compiler can't 
prove that even in checked case bounds checks can be elided.

The results on my machine are (smaller is better)

```
sum_indirectly:           63.633349ms
sum_indirectly_unchecked: 64.861459ms

sum:           34.513286ms
sum_unchecked: 17.653021ms
```

That is, there's a significant difference where bounds checks prevent
auto-vectorization, but no significant difference when accesses are indirect.