# Basics 

So, Astray is a framework to develop parsing functions from Rust type definitions.
It provides 2 basic components, each a separate crate:
1. A [proc-macro](https://github.com/giluis/astray_macro) for generating these functions
2. [Core](https://github.com/giluis/astray_core) functionality that helps this along

The core of Astray is the `Parsable` trait, which can be automatically derived with the `SN` ("Syntax Node") macro.
Just annotating a type with `SN` will auto generate an implementation of `Parsable` , as we'll see in the next chapter.

At the heart of Astray lies the SN macro, a derive-macro that takes a type definition and builds a parsing function for it. We'll cover some basic examples in the next chapter


