// https://doc.rust-lang.org/book/first-edition/getting-started.html

/// Anatomy of a Rust Program
/// Now, let’s go over what just happened in your "Hello, world!" program in detail. Here's the
/// first piece of the puzzle:
///
/// ```
/// fn main() {
///
/// }
/// ```
///
/// These lines define a function in Rust. The main function is special: it's the beginning of every
/// Rust program. The first line says, “I’m declaring a function named main that takes no arguments
/// and returns nothing.” If there were arguments, they would go inside the parentheses (( and )),
/// and because we aren’t returning anything from this function, we can omit the return type
/// entirely.
///
/// Also note that the function body is wrapped in curly braces ({ and }). Rust requires these
/// around all function bodies. It's considered good style to put the opening curly brace on the
/// same line as the function declaration, with one space in between.
///
/// Inside the `main()` function:
///
/// ```
/// println!("Hello, world!");
/// ```
///
/// This line does all of the work in this little program: it prints text to the screen. There
/// are a number of details that are important here. The first is that it’s indented with four
/// spaces, not tabs.
///
/// The second important part is the println!() line. This is calling a Rust macro, which is how
/// metaprogramming is done in Rust. If it were calling a function instead, it would look like
/// this: println() (without the !). We'll discuss Rust macros in more detail later, but for now
/// you only need to know that when you see a ! that means that you’re calling a macro instead
/// of a normal function.
///
/// Next is "Hello, world!" which is a string. Strings are a surprisingly complicated topic in a
/// systems programming language, and this is a statically allocated string. We pass this string
/// as an argument to println!, which prints the string to the screen. Easy enough!
///
/// The line ends with a semicolon (`;`). Rust is an expression-oriented language, which means
/// that most things are expressions, rather than statements. The ; indicates that this
/// expression is over, and the next one is ready to begin. Most lines of Rust code end with a
/// `;`.

fn main() {
        println!("Hello, world!");
}
