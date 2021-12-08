import sys

lines = []
for line in sys.stdin:
    line = line.strip()
    if line:
        parts = []
        for part in line.split(" | "):
            words = ["".join(sorted(word)) for word in part.split(" ")]
            parts.append(words)
        lines.append(tuple(parts))


lookup = [
    ["a", "b", "c", "e", "f", "g"],
    ["c", "f"],
    ["a", "c", "d", "e", "g"],
    ["a", "c", "d", "f", "g"],
    ["b", "d", "c", "f"],
    ["a", "b", "d", "f", "g"],
    ["a", "b", "d", "f", "g", "e"],
    ["a", "c", "f"],
    ["a", "b", "c", "d", "e", "f", "g"],
    ["a", "b", "c", "d", "f", "g"],
]


def union(g):
    out = None
    for x in g:
        if out is None:
            out = set(x)
        else:
            out |= set(x)
    return out


total = 0
for p1, p2 in lines:
    word_set = set(p1 + p2)
    digit_lengths = {i: len(vals) for i, vals in enumerate(lookup)}
    possible_digits = {
        word: set(i for i, length in digit_lengths.items() if length == len(word))
        for word in word_set
    }
    known = {
        next(iter(possib)): set(word)
        for word, possib in possible_digits.items()
        if len(possib) == 1
    }

    # top_left_or_bottom_right = known[8] - known[2]
    top_right_or_bottom_right = known[1]
    top = known[7] - known[1]

    words_with_top = [set(word) for word in word_set if set(word) & top]
    two_six_or_five = [word for word in words_with_top if word & known[1] != known[1]]
    known[6] = max(two_six_or_five, key=len)

    top_right = known[8] - known[6]
    bottom_right = known[1] - top_right
    (known[5],) = [
        word
        for word in two_six_or_five
        if word != known[6] and not top_right.issubset(word)
    ]
    (known[2],) = [
        word
        for word in two_six_or_five
        if word != known[6] and top_right.issubset(word)
    ]
    bottom_left = known[6] - known[5]
    top_left = known[8] - known[2] - bottom_right
    mid = known[4] - top_left - known[1]
    known[0] = known[8] - mid
    (known[9],) = [
        set(word)
        for word in word_set
        if len(word) == 6 and set(word) != known[6] and set(word) != known[0]
    ]
    bottom_left = known[8] - known[9]
    bottom = (
        set("abcdefg") - top - top_left - top_right - mid - bottom_left - bottom_right
    )

    good_to_bad = {
        "a": top,
        "b": top_left,
        "c": top_right,
        "d": mid,
        "e": bottom_left,
        "f": bottom_right,
        "g": bottom,
    }
    bad_to_good = {next(iter(bad)): good for good, bad in good_to_bad.items()}

    def digit_lookup(good):
        for d, vals in enumerate(lookup):
            if good == set(vals):
                return d
        raise ValueError

    current = 0
    for word in p2:
        good = set(bad_to_good[b] for b in word)
        current = current * 10 + digit_lookup(good)
    total += current
print(total)
