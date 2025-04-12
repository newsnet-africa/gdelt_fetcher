# Code Format
---
This section describes the format of the codes that are used to identify specific objects. The reason this is a separate object is because the codes very quickly get confusing because they may share properties (like character types or formats) with other codes. To prevent confusion, these codes will be wrapped in a transparent struct tuple that prevents dumb shit from happening:

e.g 
```rust
#[repr(transparent)] 
SomeCode(u8) // A single digit (or letter) code

#[repr(transparent)]
SomeCode([u8; 4]) // A 4 digit (or letter) code
```

Now let's imagine some dumb shit:

Let's say company `A` is coded by a 4 letter code combined with a 2 letter Country 2 letter Administration code like so: `SHRTSF01` and company `B` only has a country: `PNPYSF`. Let's say we were reading these codes for parsing as arguments for a function foo:

```
fn foo(country_code)
```

Although it might add to the confusion, storing letter codes as a u8 array that gets translated to and from a UTF-8/16 or ASCII string saves space, preserves simplicity and does bound checking. This can ensure automatic checking of the code size and makes it slightly easier to slice if necessary. This file should simply contain the format of the code and the source of the dictionary that the code definitions stay.