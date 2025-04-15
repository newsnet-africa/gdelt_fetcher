---
file_type: SuperComponent
SuperComponent:
  - "[[Appeal]]"
  - "[[ConsultationAndDiplomacy]]"
  - "[[CooperationAndIntent]]"
  - "[[MaterialAidProvision]]"
  - "[[PublicStatement]]"
  - "[[Assault]]"
  - "[[Coersion]]"
  - "[[Conflict]]"
  - "[[Disapproval]]"
  - "[[ForcePosture]]"
  - "[[InvestigationAndDemands]]"
  - "[[MassViolence]]"
  - "[[Protest]]"
  - "[[ReducedRelation]]"
  - "[[Rejection]]"
  - "[[Threats]]"
  - "[[YieldingAndSanctions]]"
---
An Event is one of the main components for us. This object describes (quite well imo) what is happening in a coded manner which can be quantified. This is useful because there also exist the Goldstien scale which can tell us how significant an event it.
There are 17 Main events, each with their own sub events and these can sometimes have nested events that further describe the super event. Unlike Actors, Events are simply one code and the variation of specificity is not a function of the combination of codes used (kinda, like at least it doesn't look like it's designed that way?), but rather a function of how far down the tree we are:

> [!example] Let's say that an event occurs: Nzuzo disapproves of Randon's actions:
> 
> ```mermaid
> flowchart TD
> 	Nzuzo --> Disapproves --> Randon
> 	subgraph Disapproves
> 		UnspecifiedDisapproval
> 	end
> ```
> > [!note]
> > You might be wondering why use the `UnspecifiedDissaproval` block at all. The answer is:
> > 1. Rust enums (and I think enums in general actually) always need to be the most specifc as they can be:
> > 2. There are other more specific events
> > Say that `Dissaporval` looked like this:
> > ```rust
> > enum Disapproval {
> > 	Unspecified,
> > 	Criticize,
> > 	Accuse(Accusation)
> > 	...
> > }
> > ```
> > The nested `Accusation` type would look like this
> > ```rust
> > enum Accusation {
> >  	CrimeOrCorruption,
> >  	HumanRightsAbuses,
> >  	Agression,
> >  	Espianoge
> > }
> > ```
> > Because it is possible for there not to be any further disapproval than the general way, a separate unspecified type is necessary. This is not that difficult to understand BTW so if you feel like you're missing something, you're not. It is still important to remember, though, because many decisions about things like filtering and categorizing events will be affected by this. 

With the above example in mind, let's look at how the sub categories can define more specific actions:


> [!example] Let's say that an event occurs: Nzuzo disapproves of Randon's **crimes**:
> 
> ```mermaid
> flowchart TD
> 	Nzuzo --creates action--> Disapproves 
> 	Randon --receives action --> Disapproves
> 	Accuse..Accusation  --SubCategory--> Accusation
> 	subgraph Disapproves
> 		Accuse..Accusation
> 	end
> 	subgraph Accusation
> 		AccuseOfCrime
> 	end
> ```
> Here, we get more specific by introducing a more specific group to the Top Level Action. 
>  > [!note]
>  > There are at most *three (3)* levels of action. If at some point more exist, there is no problem adding them, but we need to come up with better naming conventions that don't rely on hierarchies, in the event they are changed 

