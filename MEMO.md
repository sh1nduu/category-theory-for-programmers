# Type and functions

## Challenges

#### 5. How many different functions are there from Bool to Bool? Can you implement them all?

Bool → Bool : identify (= x)

Bool →  Bool : not (= !x)

#### 6. Draw a picture of a category whose only objects are the types Void, () (unit), and Bool; with arrows corresponding to all possible functions between these types. Label the arrows with the names of the functions.

@startuml
hide empty members

Void => Void : absurd or id?
Void ==> Unit : absurd
Void ==> Bool : absurd

Unit => Unit : id
Unit ==> Bool : true
Unit ==> Bool : false

Bool => Bool : id
Bool ==> Unit : discard
Bool ==> Unit : discard
@enduml

