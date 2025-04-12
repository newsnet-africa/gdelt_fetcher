---
file_type: StandardCode
---
# Actor Type Codes
These codes identify actors and their roles to provide context about the actors in question. These roles are useful for filtering out the nature of an event.

> [!example] Why would this type be useful?
> Let's say we were want to add a feature that tells us is our president is involved in the news
>  
>  > [!example] A bad way to do this
>  > ```rust
>  > fn involves_president(event: Event) -> bool {
>  > 	if event.actor1.name.contains("Ramaphosa") || event.actor2.name.contains("Ramaphosa") {
>  > 		return true	
>  > 	} else {
>  > 		return false
>  > 	}
>  > }
>  > ```
>  > The above example might return true even when ramaphosa is no longer a president
>  
>  > [!example] A better way to do this
>  > ```rust
>  > fn involves_president(event: Event) -> bool {
>  > 	if (event.actor1.type1.equals(President) || event.actor1.type2.equals(President) || event.actor2.type1.equals(President) || event.actor2.type2.equals(President)) &&
>  > 	event.country.equals(SouthAfrica) {
>  > 		return true
>  > 	} else {
>  > 		return false
>  > 	}
>  > }
>  > ```
>  
>  This code keeps the filter logic the same across presidential administrations. This is again a pretty shit example but for simplicity's sake, it shows you how types can be useful.
