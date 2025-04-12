---
fields:
  - name: SuperComponent
    type: MultiFile
    options:
      dvQueryString: dv.pages().where(p => p.file_type?.contains("SubComponent") && p.file.path.contains(current.file.path.split("/")[0]))
    path: ""
    id: 8qouEh
version: "2.1"
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
  - 8qouEh
---