# Type and functions

## Challenges

#### 5. How many different functions are there from Bool to Bool? Can you implement them all?

Bool → Bool : identity (= x)

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

# Categories Great and Small

## Challenges

#### 1. Generate a free category from

##### 1. A graph with one node and no edges


@startuml
hide empty members
rectangle A

A -> A : identity
@enduml

##### 2. A graph with one node and one (directed) edges (hint: this edge can be composed with itself)

Add infinite arrows to represent every number of applications of the directed edge.


##### 3. A graph with two node and a single arrow between them

@startuml
hide empty members
rectangle A
rectangle B

A -> A : identity
A --> B : edge
B -> B : identity
@enduml

##### 4. A graph with a single node and 26 arrows marked with the letters of the alphabet: a, b, c ... z.

Add an identity arrow, and then add infinite arrows, one for every combination of a-z of any length.

#### 3. Considering that Bool is a set of two values True and False, show that it forms two (set-theoretical) monoids with respect to, respectively, operator `&&` (AND) and `||` (OR).

AND and OR are associative.

(A && B) && C = A && (B && C)

(A || B) || C = A || (B || C)

The identity for AND is true.

A && true = A

The identity for OR is false.

B || false = B

#### 4. Represent the Bool monoid with the AND oeprator as a category: List the morphism and their rules of composition.


Bool -> Bool : AND True

Bool -> Bool : AND False (composition)


#### 5. Represent addition modulo 3 as a monoid category.

The single element of this category is N (Natural Number: n [0,2]).

The morphisms are A: add 3n (identity), B: add 1 + 3n, C: add 2 + 3n.

The morphisms in this category are closed under association because B . B is C and both B . C, C . B are A.

