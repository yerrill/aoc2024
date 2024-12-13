# Math for solution

## Proof of concept

Example from [Day 13 description](https://adventofcode.com/2024/day/13):

```text
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
```

> The cheapest way to win the prize is by pushing the `A` button `80` times and the `B` button `40` times.

Setting up equations to isolate $X$ and $Y$ movement based on button presses of $a$ and $b$:

1. $X = A_X * a + B_X * b$
2. $Y = A_Y * a + B_Y * b$

Lowercase $a$ and $b$ are the number of button presses for buttons a and b. Uppercase respresent constants from the input like $X$ and $Y$. Subscripted constants like $A_X$ or $B_Y$ are the equation and the $X$ or $Y$ value they change.

Substituting:

1. $8400 = 94 * a + 22 * b$
2. $5400 = 34 * a + 67 * b$

Solve for $a$ with equation 1:

$$ a = \frac{8400 - 22 * b}{94} $$

Solve for $b$ with equation 2:

$$ 5400 = 34 * (\frac{8400 - 22 * b}{94}) + 67 * b $$

$$ b = 40 $$

Solve for $a$:

$$ a = \frac{8400 - 22 * 40}{94} = 80 $$

## Generalizing

Equations for $a$ & $b$ can be simplified to:

$$ a = \frac{X - B_X * b}{A_X} $$

$$ b = \frac{A_X * Y - A_Y * X}{-A_Y * B_X + A_X * B_Y} $$

Substituting the parsed values from the input allows for calculation of the required $a$ & $b$ button pushes. Since this is a system of linear equations there will always be one solution. In the context of the problem, the solution is only valid if the calculated $a$ and $b$ values are both integers.
