```
_____/\\\\\\\\\\\\__/\\\\\\\\\\\________/\\\\\\\\\_
____/\\\//////////__\/////\\\///______/\\\////////__
___/\\\_________________\/\\\_______/\\\/___________
____\/\\\____/\\\\\\\_____\/\\\______/\\\_____________
_____\/\\\___\/////\\\_____\/\\\_____\/\\\_____________
______\/\\\_______\/\\\_____\/\\\_____\//\\\____________
_______\/\\\_______\/\\\_____\/\\\______\///\\\__________
________\//\\\\\\\\\\\\/___/\\\\\\\\\\\____\////\\\\\\\\\_
__________\////////////____\///////////________\/////////__

```

# GIC - The progamming language for FOL

This project was made for educational pourposes, to learn about the implementation of a programming language.

### Features

- **First Order Logic Parsing**: GIC can parse first order logic expressions.
- **SLD Resolution**: GIC implements SLD resolution for query processing, allowing users to perform logical queries on facts and rules.
- **Built-in predicates**: Gic supports built-in predicates: `Eq/2`, `Diff/2`, and `Var/1` for logical operations and variable handling.

### Syntax

Syntax is FOL-like: If P is a predicate (Uppercase) and t1,...,tn terms such as Variables or function applications(Lowercase), then the syntax for L-Formulas is as follows:

```
L-Formulas f ::= P(t1,... tn) | bottom | f and f | f or f | f impl f | not f | exists X. f | forall X. f
// or equivalently:
L-Formulas f ::= P(t1,... tn) | ⊥ | f ∧ f | f ∨ f | f ⇒ f | ¬ f | ∃ X. f | ∀ X. f

```

You can use any of the symbols interchangeably.
A .gic file consists of a set of L-Formulas separated by `.`.

### Usage

To use GIC, you can run the **igic** interpreter and load a file containing your logic program. The program should be written in the GIC syntax.

### Example

```bash
cd igic
cargo run
Welcome to the IGIC REPL! Type 'exit' or 'quit' to leave.
igic> load ..\examples\family.gic
igic> query "Exists X. Exists Y. Grandpa(X,Y)"
Goal Clause: {¬Grandpa(X, Y)}
✔ Solution found!
X := juan(), Y := maria()
Continue? (Y/N) Y
✘ No solution found.
igic> query "∃ X. ∃ Y. Brother(X,Y)"
Goal Clause: {¬Brother(X, Y)}
✔ Solution found!
X := luis(), Y := pepe()
Continue? (Y/N) Y
✔ Solution found!
X := pepe(), Y := luis()
Continue? (Y/N) Y
✘ No solution found.
```

### Current limitations

- **Different Named variables**: GIC does not currently support different named variables for a progam, so please rename accordingly.
- **Only SLD Resolution**: GIC only implements SLD resolution, for horn progams and objective clauses, which may not be suitable for all use cases.
