# Hello, Rust

First - thank you for picking up a copy of this book! Many of you will have talked about the topic of "Algorithms and Data Structures" only back in university. In fact, regardless if this is your first endeavour into programming or not, we worked hard to make this book a great learning experience. And Rust isn't that old anyway so how much experience can you have? 

How old is Rust? It started off in 2006 by Graydon Hoare, an engineer at Mozilla and was then quickly adopted by the company and less than a decade later (May 15, 2015) the Rust team announced a stable version 1.0! During its journey there have been many features that have been added and removed again (for example a Garbage Collector, classes, and interfaces) to become the safe, concurrent language that it is today. 

In this chapter we will cover some of Rust's fundamental concepts as a basis for later discussions! Look forward to learning about:

- Rust language features in the 2018 edition
- The latest and greatest about borrowing and ownership 
- How we can leverage concurrency and mutability properly
- References (not pointers!) to where the language lives


## Rust in 2018

As of this writing, Rust comes in two editions: 2015 and 2018. Even if you know where to look, it's often hard to keep up with the core team's six week cycles to know exactly what's the new thing right now and how it improves your live. Where 2015 provides very fundamental constructs to make the language useful, 2018 adds many features that could be conceived as convencience additions. Whether this is good or not is left for you to decide, however it shows the maturity stage a language is at.

Let's look at some notable features:

- impl Trait: Program to an interface, not to an implementation! What had to be boxed before can now be done at zero cost, perfect for implementing .. say sorting algorithms!  
- async/await: The C# feature that lets one avoid the callback hell finally made it into Rust. These are easy operators for concurrent programming.
- '_ the anonymous lifetime: Rust sometimes insisted on lifetimes a lot - so often a dummy lifetime was used. This is now gone! 
- Modules: Explicit declarations of external libraries is a thing of the past, as well as the file naming restrictions when using folders.

These things make Rust a lot easier and more convenient to use. Though - what is Rust used for these days? The language (and its standard library) specifically intended for any partP of the stack and while usage of Open Source technology is notoriously difficult to track, there are some known users and use cases, for example:

- Parts of Microsoft's IoT edge engine are written in Rust 
- Mozilla's latest version of Servo rendering engine for Firefox is built in Rust
- Companies are exploring the crypto currency space with Rust
- Amazon and Facebook created tools for license checking and performance analysis respectively
- Games are created in Rust thanks to the easy availability of WebAssembly

Although Rust was created with systems programming in mind, it lends itself to a lot of use cases ranging from device drivers for the Internet of Things (IoT) to web servers, basic tools, and machine learning. With WebAssembly looming on the horizon Rust is a great candidate for making highly performant browser applications too. 




## Borrowing And Ownership

## Concurrency And Mutability

## Deeper Into Rust

# Crates, Cargo, And The Ecosystem

## Cargo
## Crates
## Testing
## Benchmarking
## A Community To Build With