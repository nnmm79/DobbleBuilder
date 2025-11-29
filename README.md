# The Mathematics Behind Dobble

Dobble is based on a beautiful mathematical structure called a **finite projective plane**. Here's how it works:

## The Key Properties

In standard Dobble:
- Each card has **8 symbols**
- Any two cards share **exactly 1 symbol**
- There are **57 different symbols** total
- The complete mathematical set has **57 cards** (though commercial versions may use fewer)

## The Mathematical Structure

Dobble uses a **projective plane of order $n = 7$**. The mapping is:
- **Cards** → lines in the projective plane
- **Symbols** → points in the projective plane

A projective plane of order $n$ has these properties:

1. Each line contains exactly $n + 1$ points
2. **Any two distinct lines intersect at exactly one point** ✨
3. Total number of points = $n^2 + n + 1$
4. Total number of lines = $n^2 + n + 1$

For $n = 7$:
- Cards have $7 + 1 = 8$ symbols
- Each symbol appears on $8$ cards
- Total symbols = $7^2 + 7 + 1 = 57$
- Total cards = $57$

## Why It Works

The crucial property is **#2 above**: any two lines meet at exactly one point.

Since cards are lines and symbols are points, **any two cards must share exactly one symbol**. This is guaranteed by the mathematical structure!

## Simple Example: $n = 2$

Here's a smaller example with $n = 2$ (3 symbols per card, 7 cards total):

| Card | Symbols |
|------|---------|
| 1 | {A, B, C} |
| 2 | {A, D, E} |
| 3 | {A, F, G} |
| 4 | {B, D, F} |
| 5 | {B, E, G} |
| 6 | {C, D, G} |
| 7 | {C, E, F} |

Try it! Any pair of cards shares exactly one symbol.

## Construction Method

For $n = 7$, one construction uses **modular arithmetic** (mod 7):

**Symbols:**
- 49 symbols labeled $(x, y)$ where $x, y \in \{0,1,2,3,4,5,6\}$
- 7 symbols labeled $(\infty, k)$ where $k \in \{0,1,2,3,4,5,6\}$  
- 1 symbol labeled $\infty$

**Cards include:**
- Lines with equation $y = mx + b$ (symbols where this holds, plus $(\infty, m)$)
- Vertical lines $x = c$ (all $(c, y)$ plus $\infty$)
- The "line at infinity" (all $(\infty, k)$ plus $\infty$)

This construction guarantees the projective plane properties hold!
