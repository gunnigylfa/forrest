# Forrest

A collection of tree implementations

## Design decisions

### Index calculus

#### Counting from one instead of zero

Using 1 as the index of the first node makes all the index calculations possible. Therefore, when instantiating the Binary Treee we mark off the first item in the vector to occupy the 0th index and start from there.

#### Storing node values as optionals

Because we are marking off the first value it needs to be clear that this value is not a value. Using optionals is one of the best if not the best way to do that.
