
# Assignment 1: Basic theory.

This task aims to check your basic knowledge of Rust and gain experience in submitting tasks.

## Task

Write answers to the questions below. Please, do not overcomplicate answers. 1-3 sentences are enough. You do not need to write an essay for every question.

1. Is Rust single-threaded or multi-threaded? Is it synchronous or asynchronous? 
2. What runtime Rust has? Does it use a GC (garbage collector)?
3. What static typing means? What are the benefits of using it?
4. What is immutability? What is the benefit of using it?
5. What are move semantics? What are borrowing rules? What is the benefit of using them?
6. What are traits? How are they used? How do they compare to interfaces?
7. What are lifetimes? Which problems do they solve?
8. What are macros? Which problems do they solve?
9. What is the difference between `&String` and `&str` types (or between `&Vec` and `&[u8]` types)? Difference between fat and thin pointers?
10. What are static and dynamic dispatches?

## Answers

1. Rust supports multi-threading through packages like std::thread, so basically both. Also, Rust supports asynchronous programming through async and await, streams and futures.

2. Rust has a very lightweight, C-like runtime. No, it does not have a garbage collector.

3. Static typing means that you have to explicitly assign variable's type which are checked at compile-time. Core benefits of it are better performance, early error detection which increase reliability of a software.

4. Values in Rust immutable by default, but you can opt into mutability with 'mut' keyword. Benefits of it are safety, it prevents accidental mutations, prevents data races in concurrency, lets compiler optimize code, since it assumes that values don't change.

5. When you are passing value somewhere, to another variable or function, in Rust this value is not being copied as in other languages, here it transfers ownership of this value, which is called a move. It basically means that in code like:

fn some_function(s: String){
    println!("Passed string: {}", s);
}

fn main(){
    let msg = String::from("Hello, Cargo!");
    some_function(msg);

    println!("{}", msg); // WONT WORK
}

... you won't be able to access a "msg" variable, since it's ownership moved to a function, not copied.

However, there is a way to implement copy semantics into a variable. It can be done by implementing 'Copy' trait, which is, by the way, implemented by default in types like i32, f64, bool, char.

Borrowing is a concept that allows us to access value of another variable temporarily. There is two kinds of borrowing - immutable and mutable. Using immutable borrowing, we can only read value and have as many of them as we want (borrows). But, there can be only one mutable borrowing of a variable's value, which allows us to modify it. 

Core benefits of those concepts is memory safety and data races prevention.

6. Traits is a construct in Rust, which allows us to define shared behaviour or functionality that some type can implement and can share with others. Since my background is in Java, for me traits is something like an interfaces, but some reddit bigheads said that traits is actually "more powerful" (i trust them). Regarding features of traits, I can highlight default static dispatch (since i didn't even know what is this before Rust), which reduces runtime overhead, unlike dynamic one in Java, and the ability to add implementation, which is actually common for Java, but not a thing in other languages.

7. Lifetimes is a way to describe how long reference is valid. They solve so called dangling references or use-after-free problem, when a reference point to a freed or deallocated memory location.

8. Macros is a way of writing code that writes other code. They are used to remove boilerplate and make programs shorter. There are two types of macros - declarative and procedural. Declarative simpler, procedural is more complex, yet more powerful.

9. In my understanding - &String is a pointer to a struct, that contains all the information about the actual string, including it's address, but &str is a type, that contains two values - address of data in memory and it's length. &str is so called fat pointer, as it's stores extra information alongside actual address, while &String is a thin pointer, which only points to a struct, which is a wrapper over actual memory address of a string and other information.

10. Static and Dynamic dispatches is two forms of a method dispatch (process of selecting which implementation of polymorphic operation to use). Rust supports static dispatch, which selects implementation based on provided data type, and fully resolved in compile-time. While in other languages like, again, Java - dynamic dispatch is used, and it resolves implementation dynamically in runtime, using vtables. 