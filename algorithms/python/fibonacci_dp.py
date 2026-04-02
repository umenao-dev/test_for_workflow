def fibonacci(n: int) -> int:
    if n <= 1:
        return n

    dp = [0] * (n + 1)
    dp[1] = 1

    for i in range(2, n + 1):
        dp[i] = dp[i - 1] + dp[i - 2]

    return dp[n]


def series(limit: int) -> list[int]:
    return [fibonacci(i) for i in range(limit)]


if __name__ == "__main__":
    print(series(10))
