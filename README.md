# Wasp 🐝
a programming language for extremely concise web assembly modules

**warning:** this compiler is very alpha and error messages aren't the best, but it works and language is simple!

```rust
extern console_log(message)

pub fn main(){
  console_log("Hello World!")
}
```

# Features
* [x] encourages immutability
* [x] immutable c-strings, memory manipulation, global variables, imported functions, 1st class functions
* [x] optional standard library runtime
* [x] functions with inline web assembly
* [x] test framework support
* [x] easy project dependency management
* [ ] self hosting

# Quickstart

Wasp depends on `git` and `rust`. Make sure you have them installed before beginning.

```console
cargo install wasp
wasp init myproject
cd myproject
wasp build
python3 -m http.server
```
Open up http://localhost:8000 and look in console. At this point we will have a web assembly module that has access to the [standard libraries](https://github.com/wasplang/std) functions.  More to come in this area!

If you don't have need for the standard library (or want to write your own!). This is also an option.

```console
wasp init myproject --no-std
```

At this point we will have a web assembly module with a single exported main function and nothing else.

If you think your standard library is out of date, just run `wasp vendor`

# Simple Data Structures

Wasp is an extremely basic language and standard library.

## Linked List

```rust
cons(42,nil) // returns the memory location of cons
```
```rust
head(cons(42,nil)) // return the head value 42
```
```rust
tail(cons(42,nil)) // returns the memory location of tail
```

```rust
cons(1,cons(2,cons(3,nil))) // returns a linked list
```

## Structs

Structs are dictionaries

```rust
struct point { :x :y }

pub fn create_point(){
  foo = malloc(size_of(point))
  set(foo,:x,1)
  set(foo,:y,1)
  foo
}
```

# Drawing

Using [web-dom](https://github.com/web-dom/web-dom) we can easily draw something to screen. Loops in wasp work differently than other languages, bbserve how this example uses recursion to rebind variables.

```rust
extern console_log(msg)
extern global_get_window()
extern window_get_document(window)
extern document_query_selector(document,query)
extern htmlcanvas_get_context(element,context)
extern canvas_set_fill_style(canvas,color)
extern canvas_fill_rect(canvas,x,y,w,h)

static colors = ("black","grey","red")

pub fn main(){
    // setup a drawing context
    window = global_get_window()
    document = window_get_document(window)
    canvas = document_query_selector(document,"#screen")
    ctx = htmlcanvas_get_context(canvas,"2d")

    x = 0
    loop {
        // get the offset for the color to use
        color_offset = (colors + (x * size_num))
        // set current color to string at that position
        canvas_set_fill_style(ctx,mem(color_offset))
        // draw the rect
        canvas_fill_rect(ctx,(x * 10),(x * 10),50,50)
        // recur until 3 squares are drawn
        x = (x + 1)
        if (x < 3) {
            recur
        }
    }
}
```

See it working [here](https://wasplang.github.io/wasp/examples/canvas/index.html)

# Mutable Global Data

It's often important for a web assembly modules to have some sort of global data that can be changed.  For instance in a game we might have a high score.

```rust
high_score = 0

fn run_my_game(){
  ...
  mem(high_score,(mem(high_score) + 100))
  ...
}
```

# Project Management
**warning: this may change but it works**
Code dependencies are kept in a special folder called `vendor` which is populated by specific checkouts of git repositories.

For example a `project.wasp` containing:

```
bar git@github.com:richardanaya/bar.git@specific-bar
```

would result in these commands (roughly)

```
rm -rf vendor
mkdir vendor
git clone git@github.com:richardanaya/bar.git@specific-bar vendor/bar
```

when `wasp vendor` is called

Now, when wasp compiles your code, it does a few things.

* In order specified by your `project.wasp`, one folder at a time all files ending in .w are loaded from each `vendor/<dependency-name>` and its subfolders.
* all files in the current directory and sub directories not in `vendor` are loaded
* then everything is compiled in order

Please try to use non conflicting names in meantime while this is fleshed out.

# Technical Details
## Types
It's easiest to think that everything is a `f64` number in wasp.

* **number** - a 64 bit float
* **string** - a number to a location in memory of the start of of a c-string (e.g. `"hello world!"`)
* **symbol** - a number to a location in memory of the start of of a c-string (e.g. `:hello_world`)
* **bool** - a number representing boolean values. True is 1, false is 0. (e.g. `true` `false`)
* **(...)** - a global only type this is a a number pointer to sequence of  values in memory (e.g. `(another_global 1 true :hey (:more-data)`). Use this for embedding raw data into your application memory on startup.

## Globals
* **nil** - a number that represents nothingness (0). Note that it is also the same value as false and the number 0.
* **size_num** - the length of a number in bytes (8). This is a global variable in wasp to cut down in magic numbers floating around in code.

## Functions
* **[pub] fn name (x,...){ ... })** - create a function that executes a list of expressions returning the result of the last one. Optionally provide an export name to make visible to host.
* **function_name(...)** - call a function with arguments
* **mem_byte(x:integer)** - get 8-bit value from memory location x
* **mem_byte(x:integer y)** - set 8-bit value at memory location x to value y
* **mem(x:integer)** - get 64-bit float value from memory location x
* **mem(x:integer y)** - set 64-bit float value at memory location x to value y

* **mem_heap_start()** - get number that represents the start of the heap
* **mem_heap_end()** - get number that represents the end of the heap
* **mem_heap_end(x)** - set number value that represents the end of the heap
* **if x { y } )** - if x is true return expression y otherwise return 0
* **if x { y } else { z })** - if x is true return expression y otherwise return expression z
* **x = y** -  bind the value of an expression y to an identifier x
* **loop { ... x } ** - executes a list of expressions and returns the last expression x. loop can be restarted with a recur.
* **recur** - restarts a loop
* **fn(x,x1 ..)->y** - gets the value of a function signature with inputs x0, x1, etc and output y
* **call(x,f,y0,y1 ...)** call a function with signature x and function handle f with parameters y0, y1, ...

### Common Operators
These oprators work pretty much how you'd expect if you've used C

* **(x + y)** - sums a list of values and returns result
* **(x - y)** - subtracts a list of values and returns result
* **(x \* y)** - multiplies a list of values and returns result
* **(x / y)** - divides a list of values and returns result
* **(x % y)** - modulos a list of values and returns result
* **(x == y)** - returns true if values are equal, false if otherwise
* **(x != y)** - returns true if values are not equal, false if otherwise
* **(x < y)** -  returns true if x is less than y, false if otherwise
* **(x > y)** - returns true if x is greater than y, false if otherwise
* **(x <= y)** - returns true if x is less than or equal y, false if otherwise
* **(x >= y)** - returns true if x is greater than or equal y, false if otherwise
* **(x and y)** - returns true if x and y are true, false if otherwise
* **(x or y)** - returns true if x or y are true, false if otherwise
* **(x & y)** - returns bitwise and of x and y
* **(x | y)** - returns bitwise or of x and y
* **!x** - returns true if zero and false if not zero
* **^x** - bitwise exclusive or of x
* **~x** - bitwise complement of x
* **(x << y)** - shift x left by y bits
* **(x >> y)** - shift x right by y bits

## Testing
```rust
pub test_addition(){
  assert(4,(2+2),"2 + 2 should be 4")
  assert(7,(3+4),"3 + 4 should be 7")
}
```
See it working [here](https://wasplang.github.io/wasp/examples/testing/index.html)

## Why so few functions?
Wasp prefers to keep as little in the core functionality as possible, letting the [standard library](https://github.com/wasplang/std) evolve faster and more independent community driven manner. This project currently follows a principle that if a feature can be implemented with our primitive functions, don't include it in the core compiled language and let the standard library implement it. Also that no heap based concepts be added to the core language.

## Notes
<p align="center">
<img src="static_heap.svg" width="400">
</p>

* all functions (including extern functions) return a value, if no obvious return, it returns ()
* Web assembly global 0 is initialized to the end of the static data section (which might also be the start of a heap for a memory allocator). This value is immutable.
* Web assembly global lobal 1 also is initialized to the end of the static data section. This value is mutable and might be used to represent the end of your heap. Check out the [simple allocator example](https://github.com/richardanaya/wasp/blob/master/examples/malloc/main.w).
* Literal strings create initialize data of a c-string at the front of your memory, and can be passed around as pointers to the very start in memory to your text. A \0 is automatically added at compile time, letting you easily have a marker to denote the end of your text.
