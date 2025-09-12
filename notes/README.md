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
