Sub components are the most granular components of the objects collected from GDELT. These are the object form of the codes that GDELT stores the Actor Event information. The general rule here is "If there is a list of possible choices, it is probably a subcomponent." These are probably going to be represented as enums and stored in memory. 

> [!note] Managing large dictionaries
> It could be that it is impractical to do so (like with 4000+ Administration codes) and those _can_ be embedded into a file or database, but do your best to avoid this because it adds latency and parsing logic. Enums are great because in rust, they optimise suuuuuper fucking well so their cost to memory is genuinely not that bad.
> 
> Ideally, if you can write a python script or some shit that can parse the list of items to an enumeration, do so because we can also update the code accordingly when the list gets updated externally by gdelt or the organisation that manages the codes.
> 
> If i can find the time, I'll add some rust macros that do this for us, but thats a tiny but complex because I'd also need to manage update logic and integration first, whereas i can simply tack it onto an existing skeleton that we know works. This does mean that the intention will be to rewrite that aspect of the parsing section, but honestly that's just the price of the game. 

