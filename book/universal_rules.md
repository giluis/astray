# Astray Universal Rules

This is a complex project, so below are some rules/axioms/definitions that *must* be upheld throughout. If at any point the code/docs fail to a single one of these, then there's a bug either in the code/docs, or in the rules. 

Throughout the code and docs, AUR::N will be the way to reference the Nth rule, as specified below:

1. Given any type `T` and a `TokenIterator<T>`, where `T: Parsable<T>`, `T` will referred to as a `Token`.
2. Given any type `P: Parsable<T>, T: Parsable<T>` P may be parsed from a TokenIterator<T>. P will be called a "parsable type", or just a "parsable"
3. Since T: Parsable<T>, for all T, T may always be parsed from TokenIterator<T>.
    - This means a Token will always be parsable from a TokenIterator<T> of itself.
    - All T: Parsable<T> 
4. Parsable types are either Tokens, or composed of other parsable types through sum types or product types. 
5. Parsing may fail, so it always produces a Result<P, ParseError<T>>, where P: Parsable<T> 
6. A failed parsing will always leave the TokenIterator at the place it was before parsing was attempted. This is true for all structs, enums and [default implementations](./additional_types.md) in Astray and should remain true for any custom types the user implements.
7. A successful parsing can either leave the iterator in the same place it was before parsing was attempted, or move the iterator along according to the length of the type it is parsing. Check [here](./additional_types.md#option) if confusing
