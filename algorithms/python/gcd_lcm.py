def gcd(a: int, b: int) -> int:
    while b:
        a, b = b, a % b
    return abs(a)


def lcm(a: int, b: int) -> int:
    if a == 0 or b == 0:
        return 0
    return abs(a * b) // gcd(a, b)


def pairwise(values: list[tuple[int, int]]) -> list[tuple[int, int]]:
    result = []
    for x, y in values:
        result.append((gcd(x, y), lcm(x, y)))
    return result


if __name__ == "__main__":
    print(pairwise([(12, 18), (7, 5), (21, 14)]))
