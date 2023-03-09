# Novacore

Novacore is a work-in-progress functional programming language designed for simplicity and ease of use. It is based on the idea that functions should be treated as first-class citizens, which means they can be passed around as values, returned as results, and composed together to form more complex functions.

One of the key features of Novacore is its unique syntax, which allows for both postfix and infix styles of programming. This means that you can write code in a way that makes the most sense to you and your problem domain.

While Novacore is still a work in progress and many things still need to be worked on, it already provides a number of built-in functions for common tasks such as filtering, mapping, and folding for lists. Novacore also supports blocks as data, which means that blocks of code can be treated as values and passed around as arguments to functions.

Novacore is designed to make programming fun and experimenting with code easy. Whether you're a beginner or an experienced developer, Novacore can help you explore the world of functional programming and have fun doing it. So why not give it a try and see what you can create?

# Getting Started with Cargo and Novacore

Novacore is built in Rust, which means that you'll need to have Rust installed on your computer in order to run it. If you don't already have Rust installed, you can download it from rust-lang.org.

Once you have Rust installed, you can use Cargo to easily build and run Novacore. Cargo is Rust's package manager and build tool, and it comes bundled with Rust.

# Installing Novacore

To install Novacore using Cargo, follow these steps:

Clone the Novacore repository to your local machine by running the following command in your terminal:

    
```bash
git clone https://github.com/pyrotek45/Novacore.git
```

Change your working directory to the root of the Novacore repository:

```bash

cd novacore
```

Build Novacore using Cargo:

```bash

cargo build --release
```
This may take a few minutes, especially the first time you build Novacore.

Once Cargo has finished building Novacore, you can run it using the following command:

```bash

 ./target/release/novacore
```

This will start the Novacore REPL (Read-Eval-Print Loop), which you can use to interactively run Novacore code.

# Using the Novacore REPL

The Novacore REPL is a command-line interface that allows you to enter Novacore code and see the results immediately. To start the REPL, simply run the following command:

```bash

./target/release/novacore
```

Once you're in the REPL, you can enter any Novacore code you want to test. For example, you can define a function to add two numbers together:


```
Nova $ add = [x y]: { x + y }
```

You can then call this function with two arguments:

```
Nova $ add(3 5)
```

The REPL will immediately print the result of the function call:
```
 ---> [8]
```

You can also call the function in a few other ways
```
Nova $ 5 add(5)
Nova $ 5 5 add()
```

In each case, the two 5's are placed on the stack first and the function `add` is then executed.

# Novacore syntax

Novacore's syntax is unlike any other programming language out there, making it unique and exciting to use. The language is designed around a series of functions that can be combined in various ways to manipulate data.

One of the key features of Novacore's syntax is its ability to use both postfix and infix styles of programming. This means that you can write code in a way that makes the most sense to you. For example, if you want to square a number, you can write it as square(2) or 2 square(). In both cases, the number 2 is placed on the stack first, and then the square() function is executed.

# How it works 

To achieve this flexibility, Novacore uses a shunting yard algorithm. This algorithm allows Novacore to interpret expressions in a way that supports both postfix and infix notations. The shunting yard algorithm works by parsing expressions from left to right, placing numbers and variables on a stack, and operators on an operator stack. When a new operator is encountered, it is compared to the operator on top of the operator stack. If the new operator has higher precedence, it is pushed onto the operator stack. If the new operator has lower precedence, operators on the operator stack are popped off and executed until an operator of lower precedence is encountered.

Novacore's use of the shunting yard algorithm also extends to function calls, with the placement of a function's matching brace indicating where it will be executed. For example, square(2) will get converted to 2 square, since the matching brace for square comes after the 2.

# Quick Start

Here are a few examples to get you started. 

```
when( true { 
  println("this will print!") 
})
```
As you can see, `when` is a function. It takes a bool and a block `{}`. Anything between `{}` are considered blocks. Blocks are like list, but contain 
code that can be stored in variables, passed around, or executed later.

If, is another control flow function, unlike when however, it takes two blocks.

```
if( false { 
  prinlnt("this will not print") 
} { 
  println("this will") 
})
```

If will execute the first block if its first argument is true, otherwise, it will execute the second.

Blocks can be stored as data and passed around. 
```
x = {
  println("im in x") 
}

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


