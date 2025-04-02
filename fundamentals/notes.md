# fundamentals
- making some cookie-cutter functions to increase my understanding of various concepts

# concepts
- variables
- data types (+ vectors)
- loops
- control flow (+ match & if let)
- functions (+ closures)

# lessons
- use consts when the value shouldn't change (preferrably as a global variable)
- **.len()** = print the number of characters or items in a compound type
- **.parse::<u8>()** = converts the type of a value to the explicit type (requires **.expect()**)
- chars include emojis
- statements don't return value, expressions do
- fn's that have a return value typically assigned to a variable
- loops can be stopped with **break** and continued by **continue**
- nested loops have labels
- default to using while loops (requires condition), when iterating through a collection, the go with for loop
- idk the application of aliasing (uses the **type** key word), but it's good to know that you can change the data types, not change per say btw
- in **match** statement, you can use if
- use **if let** when matching some enums (**option**)

# challenges
- how to return multiple types and values returned from a function
- disable clippy warnings (tried reading the clippy book, but i don't get it)
- understanding of From and Into (from **String::From("")**) requires some basics of traits and generics
- destructuring tuples, arrays, enums, pointers and structs with match
- **while let**, probably will make more sense after some more practice with **if let**
- learn pattern and matching from book
