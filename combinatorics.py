"""
combinatorics.py - Generator-based combinatorial primitives.
All functions are lazy (yield-based) to handle large state spaces efficiently.
"""

from typing import Generator, TypeVar, Sequence

T = TypeVar("T")


# ------------Permutations---------------------------------------------------
def permutations(items: Sequence[T], r: int = None) -> Generator[tuple, None, None]:
    """
    Yield all r-length permutations of items (default: full-length).
    >>> list(permutations([1,2,3], 2))
    [(1, 2), (1, 3), (2, 1), (2, 3), (3, 1), (3, 2)]
    """
    items = list(items)
    n = len(items)
    if r is None:
        r = n
    if r > n or r < 0:
        return

    indices = list(range(n))
    cycles = list(range(n, n - r, -1))

    yield tuple(items[i] for i in indices[:r])

    while True:
        for i in range(r - 1, -1, -1):
            cycles[i] -= 1
            if cycles[i] == 0:
                # Rotate indices
                indices[i:] = indices[i + 1:] + indices[i:i + 1]
                cycles[i] = n - i
            else:
                j = cycles[i]
                # Swap
                indices[i], indices[-j] = indices[-j], indices[i]
                yield tuple(items[k] for k in indices[:r])
                break
        else:
            return


# -------Combinations-----------------------------------------------------
def combinations(items: Sequence[T], r: int) -> Generator[tuple, None, None]:
    """
    Yield all r-length combinations (without replacement).
    >>> list(combinations([1, 2, 3, 4], 2))
    [(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)]
    """
    items = list(items)
    n = len(items)
    if r > n or r < 0:
        return
    if r == 0:
        yield ()
        return

    indices = list(range(r))
    yield tuple(items[i] for i in indices)

    while True:
        for i in range(r - 1, -1, -1):
            if indices[i] != i + n - r:
                break
            else:
                return
        else:
            return

        indices[i] += 1
        for j in range(i + 1, r):
            indices[j] = indices[j - 1] + 1
        yield tuple(items[i] for i in indices)


# -------Combinations with Replacement------------------------------------
def combinations_with_replacements(
    items: Sequence[T], r: int
) -> Generator[tuple, None, None]:
    """
    Yield all r-length combinations with replacement.
    >>> list(combinations_with_replacements([1, 2], 2))
    [(1, 1), (1, 2), (2, 2)]
    """
    items = list(items)
    n = len(items)
    if not n or r < 0:
        return
    if r == 0:
        yield ()
        return

    indices = [0] * r
    yield tuple(items[i] for i in indices)

    while True:
        for i in range(r - 1, -1, -1):
            if indices[i] != n - 1:
                break
            else:
                return
        else:
            return

        indices[i] += 1
        for j in range(i + 1, r):
            indices[j] = indices[i]
        yield tuple(items[k] for k in indices)


# ---------Integer Partitions---------------------------------------------
def integer_partitions(n: int, max_part: int = None) -> Generator[tuple, None, None]:
    """
    Yield all integer partitions of n (non-increasing order).
    Optionally restrict the maximum part size.
    >>> list(integer_partitions(4))
    [(4,), (3, 1), (2, 2), (2, 1, 1), (1, 1, 1, 1)]
    """
    if max_part is None:
        max_part = n
    if n == 0:
        yield ()
        return

    def _partition(remaining, max_val, current):
        if remaining == 0:
            yield tuple(current)
            return
        for part in range(min(remaining, max_val), 0, -1):
            yield from _partition(remaining - part, part, current + [part])

    yield from _partition(n, max_part, [])


# ---------Set Partitions-------------------------------------------------
def set_partitions(items: Sequence[T]) -> Generator[list, None, None]:
    """
    Yield all partitions of a set (Bell number B(n) total).
    Each partition is a list of lists (blocks).
    >>> list(set_partitions([1, 2, 3]))  # B(3) = 5
    [[[1, 2, 3]], [[1, 2], [3]], [[1, 3], [2]], [[1], [2, 3]], [[1], [2], [3]]]
    """
    items = list(items)
    if not items:
        yield []
        return

    def _partitions(remaining, blocks):
        if not remaining:
            yield [list(b) for b in blocks]
            return
        first, *rest = remaining
        # Add to an existing block
        for i, block in enumerate(blocks):
            new_blocks = blocks[:i] + [block + [first]] + blocks[i + 1:]
            yield from _partitions(rest, new_blocks)
        # Start a new block
        yield from _partitions(rest, blocks + [[first]])

    yield from _partitions(items, [])


# --------Power Set-------------------------------------------------------
def power_set(items: Sequence[T]) -> Generator[tuple, None, None]:
    """
    Yield all subsets of items (2^n total).
    >>> list(power_set([1, 2, 3]))
    [(), (1,), (2,), (3,), (1, 2), (1, 3), (2, 3), (1, 2, 3)]
    """
    items = list(items)
    n = len(items)
    for r in range(n + 1):
        yield from combinations(items, r)


# --------Cartesian Product-----------------------------------------------
def cartesian_product(*sequences, repeat: int = 1) -> Generator[tuple, None, None]:
    """
    Yield the cartesian product of input sequences.
    >>> list(cartesian_product([0, 1], repeat=3))
    [(0, 0, 0), (0, 0, 1), (0, 1, 0), (0, 1, 1), (1, 0, 0), (1, 0, 1), (1, 1, 0), (1, 1, 1)]
    """
    pools = [list(s) for s in sequences] * repeat
    result = [()]
    for pool in pools:
        result = [x + (y,) for x in result for y in pool]
    yield from result
