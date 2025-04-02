# create hello world
- **cargo** automatically creates the project with a binary file

# concepts

```bash
# create project
cargo new hello_world
# run project, use -p
cargo run --package hello_world
```

# lessons
## package vs bin
after reading
```bash
cargo help run
```
- using --package makes more sense over --bin because the hello_world program is implied to be a *package*
