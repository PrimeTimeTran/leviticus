- Longest Line File
  38,516 ./crates/editor/src/editor_tests.rs

```sh
find . \
  -path ./target -prune -o \
  -path ./node_modules -prune -o \
  -type f -name "*.rs" -print \
| xargs wc -l \
| sort -nr \
| head -n 10
```

- Total lines of code
  1,429,888 total

```sh
find . \
  -path ./target -prune -o \
  -path ./node_modules -prune -o \
  -type f -name "*.rs" -print \
| xargs wc -l \
| tail -n 1
```

- Total # of words
  3,607,408 total

```sh
find . \
  -path ./target -prune -o \
  -path ./node_modules -prune -o \
  -type f -name "*.rs" -print \
| xargs wc -w \
| tail -n 1
```

- Total # of unique words
  490,991 total

```sh
find . \
  -path ./target -prune -o \
  -path ./node_modules -prune -o \
  -type f -name "*.rs" -print \
| xargs cat \
| tr -s '[:space:]' '\n' \
| sort \
| uniq \
| wc -l
```
