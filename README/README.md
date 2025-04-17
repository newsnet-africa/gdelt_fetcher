#  Structure
This library is in charge of everything that relates to the data that comes from the GDELT project. Here the following is supposed to happen:

Source of the Dictionaries:
The dictionaries that store the mappings of the codes of the SubComponents are stored [here](https://www.gdeltproject.org/data.html#documentation). 

- [ ] Fetch Data from the source
	- [x] CSV:
		- [x] [[GDELT-Event_Codebook-V2.0.pdf|Event]] data
		- [x] [[GDELT-Event_Codebook-V2.0.pdf|Mention]] data
		- [x] [[GDELT-Global_Knowledge_Graph_Codebook-V2.1.pdf|GKG]] data
	- [ ] JSON:
		- [ ] [[Announcing The Global Entity Graph (GEG) And A New 11 Billion Entity Dataset – The GDELT Project.html|GEG]] data
		- [ ] [[Global Relationship Graph_ Realtime Verb-Centered NGram Pilot – The GDELT Project.html|GRG]] data
		- [ ] [[GDELT GEO 2.0 API Debuts! – The GDELT Project.html|GGG]] data
		- [ ] [[Announcing The Global Quotation Graph – The GDELT Project.html|GQG]] data
		- [ ] [[Announcing the GDELT Global Difference Graph (GDG)_ Planetary Scale Change Detection For The Global News Media – The GDELT Project.html|GDG]] data 
		> Although I think it might be better to pull the GDG as an RSS Feed, given that it is an option, unlike some of the other graphs which only give us JSON files
- [>] Parse the data into their Serialisable Rust object counterparts
	- [ ] CSV:
		- [ ] [[GDELT-Event_Codebook-V2.0.pdf|Event]] data
		- [ ] [[GDELT-Event_Codebook-V2.0.pdf|Mention]] data
		- [ ] [[GDELT-Global_Knowledge_Graph_Codebook-V2.1.pdf|GKG]] data
	- [ ] JSON:
		- [ ] [[Announcing The Global Entity Graph (GEG) And A New 11 Billion Entity Dataset – The GDELT Project.html|GEG]] data
		- [ ] [[Global Relationship Graph_ Realtime Verb-Centered NGram Pilot – The GDELT Project.html|GRG]] data
		- [ ] [[GDELT GEO 2.0 API Debuts! – The GDELT Project.html|GGG]] data
		- [ ] [[Announcing The Global Quotation Graph – The GDELT Project.html|GQG]] data
		- [ ] [[Announcing the GDELT Global Difference Graph (GDG)_ Planetary Scale Change Detection For The Global News Media – The GDELT Project.html|GDG]] data
	> Note that these objects are composed of other subcomponent or otherwise objects which should be found in the rest of this folder. The structure of these objects will be represented by the relationships that are created in this README Folder and Obsidian will graph them for you. If I'm not lazy, I'll also add details for how each subcomponent can be formed from the raw data.
	
	- [x] Actor
	- [x] Country
	- [x] KnownGroup
	- [x] Ethnicity
	- [x] Religion