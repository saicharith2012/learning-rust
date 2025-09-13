# Rust Tutorial

## Initializing a project locally

![alt text](image.png)

## Variables, Conditionals, Loops and Functions

![alt text](image-1.png)

![alt text](image-2.png)

![alt text](image-3.png)

## Structs

![alt text](image-4.png)

![alt text](image-5.png)

## Enums

![alt text](image-6.png)

![alt text](image-7.png)

## Pattern Matching

![alt text](image-8.png)

![alt text](image-9.png)

## Option Enum

![alt text](image-10.png)

![alt text](image-11.png)

![alt text](image-12.png)

## Result Enum

![alt text](image-13.png)

![alt text](image-14.png)

## Package Management in Rust

![alt text](image-15.png)

## Memory Management in Rust

![alt text](image-16.png)

![alt text](image-17.png)

When a program runs it is reserved some static memory and during the execution it also keeps asking for some dynamic memory.

![alt text](image-18.png)

## Heap vs Stack

The ```static size``` variables that are identified during compile time are stored inside a ```stack```.

![alt text](image-19.png)

When a function runs, its ```stack frame``` is pushed into the stack and is popped off when its execution gets completed and the memory is freed.

![alt text](image-20.png)

![alt text](image-21.png)

The ```variables whose size could be changed``` during runtime are stored in the ```heap```.

![alt text](image-22.png)

![alt text](image-23.png)

![alt text](image-24.png)

![alt text](image-25.png)

> For all the dynamic sized data that's stored on the heap, the corresponding metadata like identifiers (whose size doesn't change) is stored on the stack along the with a ```pointer``` to the data in the heap.

![alt text](image-26.png)

## Mutability

![alt text](image-27.png)

![alt text](image-29.png)

![alt text](image-28.png)

## Ways of memory management in different languages

Memory management is the process of allocating and deallocating the memory based on the functions that are being executed.

![alt text](image-30.png)

![alt text](image-31.png)

In the above example, the memory leak happens since the memory is not deallocated after the function execution is finished.

![alt text](image-32.png)

### The Rust way

![alt text](image-33.png)

![alt text](image-34.png)

![alt text](image-35.png)

Rust handles the cleaning up of the heap by itself when the stack frame of a function is cleared.

> In Rust, a value can't exist on the heap if it doesn't have a reference on the stack.

## Ownership

### Ownership rules

- **Each value in rust has an owner.** (Each data on the heap has an owner on the stack.)
- **There can only be one owner at a time.** (Solves the ```double free error``` where same heap address is tried to be deallocated twice.)
- **When the owner goes out of scope, the value will be dropped.**

The job of a garbage collector is to figure out what to clean up in the memory from time to time by often interrupting the control flow (like in javascript).

Hence this constraint is not present in ```Rust```, its comparatively faster than js.

## Moving

![alt text](image-36.png)

![alt text](image-37.png)

When the the existing value of a variable a1 is reassigned to new variable a2,

Neither a new copy is created in the heap, nor both the identifier reference the same address.

![alt text](image-39.png)

**Instead, the ownership of the value shifts to a2 such that a1 becomes invalid and no longer point to the value in the heap.**

![alt text](image-38.png)

![alt text](image-40.png)

The same happens in case of passing the **variable a1** as an argument to a function lets say print_str, in place of a **parameter name a2**.

![alt text](image-41.png)

To solve this issue, the value needs to be reassigned or returned to the original variable.

![alt text](image-42.png)

## Borrowing

![alt text](image-43.png)

hence, to prevent this ugly way of returning values to avoid ownership invalidation, we can use the concept of ```borrowing``` with the help of ```references```.

![alt text](image-44.png)

![alt text](image-45.png)

![alt text](image-46.png)

![alt text](image-47.png)

### Rules of References

- At any given time, you can have either one mutable reference or any number of immutable references.

- References must always be valid.

![alt text](image-48.png)

![alt text](image-49.png)

## Collections

![alt text](image-50.png)

## Vectors

![alt text](image-51.png)

### Iterating through a vector using for loop

```rust
for val in vec {
    if val % 2 == 0 {

    }
}
```

### Initialising Vectors using macros

![alt text](image-52.png)

### Defining the type of the vector as generic

## Hashmap

![alt text](image-53.png)

![alt text](image-54.png)

## Iterators

![alt text](image-55.png)

### Iterating using for loops

![alt text](image-56.png)

### Iterating after creating an iterator

![alt text](image-57.png)

> Iterator borrows the values with ```immutable references```, hence the original vector ```nums``` still owns these values.

### iter_mut()

```iter_mut()``` can be used to make ```mutable references``` to the original collection with the iterator.

![alt text](image-58.png)

### Iterating using .next

![alt text](image-59.png)

### into_iter()

![alt text](image-60.png)

![alt text](image-61.png)

### Which Iterator to choose?

![alt text](image-62.png)

Iterating directly on the collection with a for loop is same as using ```into_iter```, hence ownership is tranferred in both the cases.

![alt text](image-63.png)

### Types of Iterators

#### Consuming Adapters

![alt text](image-64.png)

#### Iterator Adapters

![alt text](image-65.png)

- Map

![alt text](image-66.png)

- Filter

![alt text](image-67.png)

![alt text](image-68.png)

![alt text](image-69.png)

## Strings vs slices

![alt text](image-70.png)

### Creating a string

![alt text](image-71.png)

### Mutating a string

![alt text](image-72.png)

### Deleting from a string

![alt text](image-73.png)

#### How string looks like in memory

![alt text](image-74.png)

> Slice doesn't have the ownership

![alt text](image-75.png)

![alt text](image-76.png)

![alt text](image-77.png)

![alt text](image-78.png)

![alt text](image-79.png)

![alt text](image-80.png)

#### String literal

The string literal is literally hard coded in the binary of the rust code.

The string literal is of type ```&str``` and it literally points to that literal in the binary.

![alt text](image-81.png)

![alt text](image-82.png)

## Generics

![alt text](image-83.png)

The main problem in the above code snippet is the ```redundancy```.

To solve this we use ```generics```.

![alt text](image-84.png)

> ```std::cmp::PartialOrd``` restricts values other than those that are comparable from being passed as arguments into the function.
