# Basics 

As you've seem by now, Astray is a framework to develop parsing functions from Rust type definitions.
When is annotated with the `SN` macro, the Parsable<T> trait is automatically implemented for it.

Any type may derive SN, or implement Parsable<T>, as long as it complies with its trait bounds.

Parsable<T> is generic over T, which represents a Token. 
A Token is the base case for parsing, a leaf in an Abstract Syntax Tree. We'll see further examples of what this means, in practice.

Tokens can be composed into more complex parsable types using:
1. structs, which (generally) represent tokens being parsed in sequence
2. enum, which (generally) represent tokens being parsed exclusivelly.

"Generally" is used here because you can actually override parsing behaviour for any type, be it a struct or an enum, though that is a more advanced topic.

For now, let's take a look at structs and how they may be used to parse tokens in sequence!
Keep going
