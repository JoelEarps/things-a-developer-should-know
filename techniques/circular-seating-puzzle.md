# Circular Seating Puzzle — Method & Solution

**Question:** Eight people A–H sit around a circular table facing the centre. Given the constraints below, who is sitting to the **immediate right of C**?

**Constraints:**
1. G is not an immediate neighbour of C.
2. A is third to the right of C.
3. E is second to the left of C.
4. C is second to the left of B.
5. F is second to the left of D.
6. A is second to the left of F.

---

## How to work it out

### 1. Fix one person and choose a direction

- Number the seats **0, 1, 2, …, 7** going **clockwise** (or all anti-clockwise — just stay consistent).
- Put **C** at position **0**. “Right of C” = next seat clockwise; “left of C” = previous seat anti-clockwise.

### 2. Place people relative to C

- **A is third to the right of C** → from 0, move 3 steps clockwise → **A at 3**.
- **E is second to the left of C** → from 0, move 2 steps anti-clockwise → **E at 6** (i.e. 0−2 mod 8).
- **C is second to the left of B** → B is second to the *right* of C → **B at 2**.

So far: **0 = C**, **2 = B**, **3 = A**, **6 = E**.

### 3. Use A and F to place F, then D

- **A is second to the left of F** → F is second to the *right* of A → A=3 ⇒ **F at 5**.
- **F is second to the left of D** → D is second to the *right* of F → F=5 ⇒ **D at 7**.

So: **5 = F**, **7 = D**.

### 4. Fill the gaps and apply “G not next to C”

- Filled: **0 C, 2 B, 3 A, 5 F, 6 E, 7 D**.
- Empty: **1** and **4** → only **G** and **H** left.
- **G is not an immediate neighbour of C** → neighbours of C (0) are seats **1** and **7**. So G cannot be at **1**. Hence **1 = H** and **4 = G**.

### 5. Final order (clockwise from C)

| Position | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|----------|---|---|---|---|---|---|---|---|
| Person   | C | H | B | A | G | F | E | D |

**Immediate right of C** = position 1 = **H**.

---

## Answer

**H** is sitting to the immediate right of C.

---

## Summary of the method

1. **Fix one person** (here, C) at a seat and number seats in one direction.
2. **Translate “left/right of X”** into “X is N steps left/right of Y” and place people step by step.
3. **Use “second to the left/right”** by inverting: “A is second to the left of F” ⇒ F is second to the right of A.
4. **Fill remaining seats** and apply any “not neighbour” (or other) rules to resolve the last unknowns.
