# General Stable Vec

A stable vector implementation that allows insertions and deletions in amortised O(1), and uses memory linear in the maximum number of elements that the vector contained.

A stable vector is a vector that keeps its indices stable, i.e. the index of an element is assigned upon insertion and not changed until removal.
This works much like a hash map, except that in this implementation, the index is assigned by the stable vector, and cannot be chosen by the user.
