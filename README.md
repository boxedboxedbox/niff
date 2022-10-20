
A toy programming language for me to implement some weird compiler optimizations and such.

# Hello world example

```
fn main() {
    println("Hello, World!");
}
```

Read the code if you want to know more.

# Implementation checklist

Not in any specific order.

' ' = not done,
x = done,
k = kinda (done, but thinking if I want to add more features)

### Lexer

- [x] Comments
- [x] Keywords
- [x] Keywords
- [ ] Macros/preprocessors, or something like that??
- [x] Do something with whitespaces??
- [ ] Proper error handling
- [ ] Optimizations
- [ ] Make more readable? (l.src[l.i] looks kinda bad, but it works)
- [ ] Documentation
- [ ] Clean up unused things

### Parser

- [k] Functions
- [ ] Type system
- [ ] Control flow
- [ ] Proper error handling
- [ ] Optimizations
- [ ] Clean up unused things (if some)
- [ ] Make more readable and call functions instead of "p.tokens[p.i].kind"
