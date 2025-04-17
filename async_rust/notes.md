# async rust
- essential for running multiple processes at the same time, it's not better than threads ... but i prefer specializing it over threads

# lessons
- tokio is a runtime, mostly used in async projects
- fs useful for cli apps and file systems dependent stuffs
- task useful for algorithms and cpu threads
- process, create multiple processes
- test, used in testings

# challenges
- tokio::task::spawn_blocking(||{}) and sometimes "move"
- select!
- revisit multithreading to understand how spawned threads can share data
