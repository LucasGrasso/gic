<p align="center">
  <img src="https://github.com/LucasGrasso/gic/blob/master/assets/logo_150px.png" />
</p>

<p align="center">
The progamming language for First Order Logic
</p>

This project was made for educational pourposes, to learn about the implementation of a programming language.

### Features

- **First Order Logic Parsing**: GIC can parse first order logic expressions.
- **SLD Resolution**: GIC implements SLD resolution for query processing, allowing users to perform logical queries on facts and rules.
- **Built-in predicates**: Gic supports built-in predicates: Check `Builtins.md` for more information.

### Syntax

Syntax is FOL-like: If P is a predicate (Uppercase) and t1,...,tn terms such as Variables or function applications(Lowercase), then the syntax for L-Formulas is as follows:

```
L-Formulas f ::= P(t1,... tn) | bottom | f and f | f or f | f impl f | not f | exists X. f | forall X. f
// or equivalently:
L-Formulas f ::= P(t1,... tn) | ⊥ | f ∧ f | f ∨ f | f ⇒ f | ¬ f | ∃ X. f | ∀ X. f
```

You can use any of the symbols interchangeably.
A .gic file consists of a set of L-Formulas separated by `.`.
foralls may be left implicit.

### Usage

To use GIC, you can run the **igic** interpreter and load a file containing your logic program. The program should be written in the GIC syntax.

### Example

```
Welcome to the IGIC REPL! Type 'exit' or 'quit' to leave.
igic> load ..\examples\family.gic
loaded.
igic> query "exists X. exists Y. Grandpa(X,Y)"
X := juan(), Y := maria()
igic> query "∃ X. ∃ Y. Brother(X,Y)"
X := luis(), Y := pepe()
Continue? (Y/N) y
X := pepe(), Y := luis()
Continue? (Y/N) y
false.
igic> query "Length(XS,6) and Reverse(XS,XS)"
XS := [H_78, H_66, H_54, H_54, H_66, H_78]
```

## License

This project is licensed under the terms of the [GNU General Public License v3.0](LICENSE.md).

© 2025 Lucas Grasso
