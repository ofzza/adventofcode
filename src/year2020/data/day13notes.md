## Day13, Puzzle #2 Notes

### Test input

```
7,13,x,x,59,x,31,19
```

### Implies

---

```
A x 7  = T
B x 13 = T + 1
...
...
C x 59 = T + 4
...
D x 31 = T + 6
E x 19 = T + 7
```

---

```
(T + 0) % 7  = 0
(T + 1) % 13 = 0
(T + 4) % 59 = 0
(T + 6) % 31 = 0
(T + 7) % 19 = 0
```

---

```
T % 7  = 0
T % 13 = 13 - 1 = 12
T % 59 = 59 - 4 = 55
T % 31 = 31 - 6 = 25
T % 19 = 19 - 7 = 12
```

### Solution (Chinese remainder theorem)

- Find T where `T % 7 = 0`, starting at `0` in step of `1` => `7`
- Find T where `T % 13 = 12`, starting at `7` in steps of `1 x 7` => `77`
- Find T where `T % 59 = 55`, starting at `77` in steps of `1 x 7 x 13` => `350`
- Find T where `T % 31 = 25`, starting at `350` in steps of `1 x 7 x 13 x 59` => `70147`
- Find T where `T % 19 = 12`, starting at `70147` in steps of `1 x 7 x 13 x 59 x 31` => `1068781`
