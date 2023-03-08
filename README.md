# Novacore

Novacore is a work-in-progress functional programming language designed for simplicity and ease of use. It is based on the idea that functions should be treated as first-class citizens, which means they can be passed around as values, returned as results, and composed together to form more complex functions.

One of the key features of Novacore is its unique syntax, which allows for both postfix and infix styles of programming. This means that you can write code in a way that makes the most sense to you and your problem domain.

While Novacore is still a work in progress and many things still need to be worked on, it already provides a number of built-in functions for common tasks such as filtering, mapping, and folding for lists. Novacore also supports blocks as data, which means that blocks of code can be treated as values and passed around as arguments to functions.

Novacore is designed to make programming fun and experimenting with code easy. Whether you're a beginner or an experienced developer, Novacore can help you explore the world of functional programming and have fun doing it. So why not give it a try and see what you can create?

# Getting started

Make sure to have rust installed, I recomend reading the rust book [here](https://doc.rust-lang.org/book/ch01-01-installation.html).

With rust installed, it should be as simple as a quick `git clone` and a `cargo run`.

If everything compiles ( and I hope it does! ), Novacores Repl should open. You can use this to test out the syntax, watch the stack, and have fun!

# Using Novacore

Novacore is a programming language that utilizes a functional programming paradigm. Its syntax is based on a series of functions that can be used to manipulate data in various ways. Novacore functions are defined using square brackets that indicate the function's arguments, followed by a colon and the function body enclosed in curly braces. For example, to define a function that squares a number, you could write: ``` square = [x]:{x*x}```. The language can create higher order functions for common tasks such as mapping, filtering, and folding lists, as well as math functions like sum, product, and power. Novacore also includes control flow functions like when, if, and for, which can be used to conditionally execute code and iterate over lists. Overall, Novacore offers a concise and powerful syntax for functional programming that is easy to learn and use.

Here are a few examples to get you started. 

```
when( true { println("this will print!") } )
```
As you can see, `when` is a function. it takes a bool and a block {}. anything between {} are considered blocks. blocks are like list but contain 
code that can be stored in variables, passed around, or executed later.

If, is another control flow function, unlike when however, it takes two blocks.

```
if( false { prinlnt("this will not print") } { println("this will") } )
```

If will execute the first block if its first argument is true, otherwise, it wille execute the second.

Blocks can be stored as data and passed around. 
```
x = {println("im in x")}
when( true x )
```

This will output: `im in x`. 

Creating a for loop is simple, it too is just a function. 
```
mylist = [ 1 2 3 4 ]
for( i mylist {
  println(i)
})
```

Try this one out for yourself!

