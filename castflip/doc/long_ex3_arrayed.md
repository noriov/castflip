How to convert between bytes and an array of `struct`s

An array of type `[T; N]` can be processed as one of the following
types depending on the situation:

1. [As a value of type `[T; N]`](../long_example3_1/index.html),
2. [As consecutive values of type `T`](../long_example3_2/index.html), or
3. [As an element of slice `[[T; N]]`](../long_example3_3/index.html).
