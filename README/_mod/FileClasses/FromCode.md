---
fields:
  - name: SourceStandardCodes
    type: MultiFile
    options:
      dvQueryString: dv.pages().where(p => p.file_type?.contains("StandardCode") && p.file.path.contains(current.file.path.split("/")[0]))
    path: ""
    id: zQ0POs
version: "2.0"
limit: 20
mapWithTag: false
icon: package
tagNames: 
filesPaths: 
bookmarksGroups: 
excludes: 
extends: 
savedViews: []
favoriteView: 
fieldsOrder:
  - zQ0POs
---
