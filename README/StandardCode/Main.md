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

> [!example]- Now lets imagine some dumb shit
> Let's say we were want to know add a feature that tells us how close the news article is taking place 
> > [!info]- 
> > That the actual real reason this logic is useful is more in the parsing use cases but it involves knowing _exactly_ how we store Provinces which is nested in a country, but [[GDELT-Event_Codebook-V2.0.pdf#page=7&selection=0,0,123,19|GDELT Locations are slightly more complicated than that.]] 
> 
> > [!note] Note
> > For this example, the `Location` object is (poorly) stored as a raw array straight from GDELT (This example also shows us why we even create in memory objects like a **proper** `Location` to begin with):
> >  ```rust
> >  struct Location([String, String, String, String, String, String, String, String]); //Can you see how fucked this looks already dude.
> > ```
> 
> > [!example] The Confusing way to store data
> >  ```rust
> > 	fn is_news_near_me(location: Location) -> bool {
> > 	 	let admin_code: String = Location.0[4];  //This is the wrong index for ADMIN CODE but the compiler will never know because its not checking if its an Admin code
> > 	...
> > }
> > ```
> > In the above example, because all the codes are strings, the compiler will never know if you have pulled the right thing out of the CSV and called it Administrative code 1.
> 
> > [!example] Less confusing way to store data
> > ```rust
> > 	fn is_news_near_me(location: Location) -> bool {
> > 		let admin_code: AdministrativeCode = Location.0[4].to_administrative_code(); // Now the compiler will check to see if the String can actually turn into an Administrative code.
> > 	}
> >```
>> In the above example, the compiler can create rules that determine if a `String` is an administrative code and catch an invalid string at the call. This way debugging is easier and we could also create `AdministrativeCode` related functions
> 
> This is a simple example but very quickly we can get fucked because of the simpleness of the kind of mistake made. These structs are low-key very important

Although it might add to the confusion, storing letter codes as a u8 array that gets translated to and from a UTF-8/16 or ASCII string saves space, preserves simplicity and does bound checking. This can ensure automatic checking of the code size and makes it slightly easier to slice if necessary. This file should simply contain the format of the code and the source of the dictionary that the code definitions stay.