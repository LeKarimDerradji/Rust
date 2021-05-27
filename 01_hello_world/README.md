## Anatomy of a Rust Program 

The Rust 'Hello, World!' is composed of two main pieces : 

```rs
fn main() {

}
``` 
First, these lines define a function in Rust. The ```main``` function is special: it is always 
the **first code that runs in every executable Rust program.** The first line declares a function named ```main``` that has no parameters and returns nothing. If there were parameters, they 
would go inside the parentheses, ().

Also, note that the function body is wrapped in curly brackets {}. Rust requires these 
around all function bodies. It's good style to place the opening curly bracket on the same
line as the function declaration, adding one space in between. 

If you want to stick to a standard style accross Rust projects, you can use an automatic 
formatter toool called ```rustfmt``` to format your code in a particulat style. 

The Rust team has included this tool with the standard Rust distribution, like ```rustc```, 
so it should already be installed on your computer when you install Rust. 

Inside the main funciton is the following code:

```    println!("Hello, world!");```

This line does all the work. It prints text to the screen. 
There are **four important details to notice here.**

**First,** Rust style is to indent with four spaces, not tab. 

**Second,** ```println!``` calls a Rust macro. If it called a function instead, it would be 
entered as ```println``` (wihout the !). Using a ```!``` means that you're calling a macro
instead of a normal function. 


**Third**, you see the ```"Hello, world!"```string. We pass this string as an argument to
```println!```, and the string is printed to the screen. 

**Fourth**, we end the line with a semicolon (```;```), which indicates that this expression
is over and the next one is ready to begin. 

Most lines of Rust code end with a semicolon. 
