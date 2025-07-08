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
- **Prolog-like Syntax**: The language is designed to be similar to Prolog, making it familiar for those who have used Prolog before.
- **SLD Resolution**: GIC implements SLD resolution for query processing, allowing users to perform logical queries on facts and rules.
- **Built-in predicates**: Gic supports built-in predicates: `Eq/2`, `Diff/2`, `Atom/1` and `Var/1` for logical operations and variable handling.

### Syntax

Syntax is FOL-like

```
L-Formulas f ::= P(t1,... tn) | bottom | f and f | f or f | f impl f | not f | exists X. f | forall X. f
// or equivalently:
L-Formulas f ::= P(t1,... tn) | ⊥ | f ∧ f | f ∨ f | f ⇒ f | ¬ f | ∃ X. f | ∀ X. f

```

You can use any of the symbols interchangeably.

### Usage

To use GIC, you can run the interpreter with a file containing your logic program. The program should be written in the GIC syntax.

### Example

You can run the gic interpreter, **igic**, from the command line:

```bash
cd igic
cargo run
igic> load ..\family.gic
igic> query "Exists X. Exists Y. Grandpa(X,Y)"
...
igic> query "∃ X. ∃ Y. Brother(X,Y)"
```

### Current limitations

- **Different Named variables**: GIC does not currently support different named variables for a progam, so please rename accordingly.
- **Only SLD Resolution**: GIC only implements SLD resolution, for horn progams and objective clauses, which may not be suitable for all use cases.
