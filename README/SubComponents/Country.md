---
file_type: SubComponent, FromCode
SubComponent:
  - "[[Actor]]"
SourceStandardCodes:
  - "[[FIPS]]"
  - "[[CAMEOCountry]]"
---
SubComponent:: [[Actor]]

SourceStandardCodes:: [[FIPS]]

SourceStandardCodes:: [[CAMEOCountry]]

SubComponent:: [[Actor]]

SourceStandardCodes:: [[FIPS]]

SourceStandardCodes:: [[CAMEOCountry]]


# Standard Codes
---
This is a list of the Codes that this object can be created from. The task status represents the progress of the implementation of the translation of the code to this object. If there is documentation for the standards of the codes, they should be provided.

- [ ] [[FIPS.country.txt|FIPS codes]]:
	- [ ] Double Digit Codes
	- [ ] Double char + Double Digit Codes 
		> Note that FIPS comes in two forms and the second also encompasses the Administrative regions (Provinces, States etc) of that country. Ideally, these regions would be sub objects of the Country Enum because every time a region is mention, it belongs to a country and this makes it easier to filter because we don't have to map a separate list of regions to the correct country.
- [ ] [[CAMEO.country.txt| Cameo codes]]

