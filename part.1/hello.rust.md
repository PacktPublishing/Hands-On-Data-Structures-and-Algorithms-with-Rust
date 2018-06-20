# Hello, Rust

First - thank you for picking up a copy of this book! Many of you will have talked about the topic of "Algorithms and Data Structures" only back in university. In fact, regardless if this is your first endeavour into programming or not, we worked hard to make this book a great learning experience. And Rust isn't that old anyway so how much experience can you have? 

How old is Rust? It started off in 2006 by Graydon Hoare, an engineer at Mozilla and was then quickly adopted by the company and less than a decade later (May 15, 2015) the Rust team announced a stable version 1.0! During its journey there have been many features that have been added and removed again (for example a Garbage Collector, classes, and interfaces) to become the safe, concurrent language that it is today. 

In this chapter we will cover some of Rust's fundamental concepts as a basis for later discussions! Look forward to learning about:

- A quick refresh on Rust and what awaits in 2018
- The latest and greatest about borrowing and ownership 
- How we can leverage concurrency and mutability properly
- References (not pointers!) to where the language lives

It is strongly encouraged to read these first chapters before diving deeper into algorithms and data structures. While experienced programmers will certainly find their ways, the next few parts contain refreshers and cover all the basics required for later. It will be worth the time!


## Rust in 2018 #todo

Is Rust a multi-paradigm language: it features traits and structs but no classes, calls methods functions, it uses many functional concepts, it's modular, and works well with generics.  


As of this writing, Rust comes in two editions: 2015 and 2018. Even if you know where to look, it's often hard to keep up with the core team's six week cycles to know exactly what's the new thing right now and how it improves your life. Where 2015 provides fundamental constructs to make the language useful, 2018 adds many features that could be conceived as convencience additions. Whether this is good or not is left for you to decide, however it shows the maturity stage a language is at.

Let's look at some notable features:

- impl Trait: Program to an interface, not to an implementation! What had to be boxed before can now be done at zero cost, perfect for implementing ... say sorting algorithms!  
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

The reason why Rust can work without a garbage collector and still free the programmer from error-prone memory management is simple: borrowing and ownership. While the particulars are quite complex - the high level view is that the compiler inserts any "allocate X amount of memory" and "free X amount of memory" (somewhat like malloc() and free() for C programmers) statements for the developer. Yet - how can it do that?

This is where Rust's declarative syntax comes into play. By declaring a variable, the compiler knows - at compile time - that a certain amount of memory needs to be allocated. The lifetime is clearly defined too, from beginning to end of a block, function, or as long as the struct instance lives. If the size of a variable is known at compile time, the compiler can provide exactly the required amount of memory to the function for the time required. 

Isn't that also what every other compiler does? The answer is yes - and no. Many modern languages do not distinguish between heap and stack variables - with significant consequences to the internals. Rust's compiler however takes advantage of the nature of stack variables:

<< Stack vs Heap pointer drawing >>


As visible in Figure X (#todo) stack-allocated memory is directly accessible from within the execution block, without the additional jump into the slower heap portion of your computer. In fact stack variables live and die with the function, heap variables can float freely until the operating system (or garbage collector) takes care of freeing that memory. In a way - a function ownes that portion of the stack. 

What happens if a stack variable is passed into a function during its lifetime? In this case the portion of the stack moves into the function. Its lifetime is now that of its new owner. However it's not always a good idea to transfer ownership back and forth, it will lead to confusing code and unnecessary copying. Therefore it's possible to borrow variables to contained scopes! In this case a variable is only passed in by reference (denoted with an &) which allows to remain efficient and stay within the ownership model. A variable can be borrowed multiple times, but it can never exceed its original lifetime! That's where lifetimes get a bit more complicated.

### Exceptional Lifetimes

Some lifetimes are different and Rust denominates them with a '. While this could be 'static, it's equally possible to create your own - something that is often required working with structs. When working with borrowed variables (called references), it's sometimes not guaranteed that the original memory area lives long enough. 

This makes sense when thinking about the underlying memory structure: if an input parameter is passed into the function and returned at the end, its lifetime surpasses the function's. While during its lifetime the function owns this part of the memory, it cannot add borrow a variable for longer than it actually exists. So this snippet could not work:

~~~Rust
fn my_awesome_function(passing_through: AFancyStruct) -> AFancyStruct {
    let x = vec![1,2,3];
    passing_through.try_own(&x);
    return passing_through;
}
~~~


The reason is simply that the variable passing_through outlives the variable x. There are two solutions to this problem: 

- Pass ownership into "passing_through". This way, the struct now owns the variable and it will live as long as the struct.
- Make sure x outlives passing_through (e.g. by passing it in too!) and declare a lifetime that shows the compiler what's up. 



## Concurrency And Mutability

## Deeper Into Rust

# Crates, Cargo, And The Ecosystem

## Cargo
## Crates
## Testing
## Benchmarking
## A Community To Build With