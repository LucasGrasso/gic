# Builtin Predicates

## All-purpose

### `Eq(X, Y)`

Checks if `X` is unifiable to `Y`.

## `Diff(X, Y)`

Checks if `X` is not unifiable to `Y`.

### `Var(X)`

Checks if `X` is a variable.

## Integers

### `Add(X, Y, Z)`

Adds two integers `X` and `Y`, and unifies the result with `Z`.
X and Y should be instanciated.

### `Sub(X, Y, Z)`

Subtracts integer `Y` from integer `X`, and unifies the result with `Z`.
X and Y should be instanciated.

### `Mul(X, Y, Z)`

Multiplies two integers `X` and `Y`, and unifies the result with `Z`.
X and Y should be instanciated.

### `Div(X, Y, Z)`

Divides integer `X` by integer `Y`, and unifies the result with `Z`.
X and Y should be instanciated. If `Y` is zero, it will raise an error.

### `Mod(X, Y, Z)`

Calculates the modulus of integer `X` by integer `Y`, and unifies the result with `Z`.
X and Y should be instanciated. If `Y` is zero, it will raise an error.

### `Lt(X, Y)`

Checks if integer `X` is less than integer `Y`.
X and Y should be instanciated.

### `Lt_eq(X, Y)`

Checks if integer `X` is less than or equal to integer `Y`.
X and Y should be instanciated.

### `Gt(X, Y)`

Checks if integer `X` is greater than integer `Y`.
X and Y should be instanciated.

### `Gt_eq(X, Y)`

Checks if integer `X` is greater than or equal to integer `Y`.
X and Y should be instanciated.

### `Eq_int(X, Y)`

Checks if integer `X` is equal to integer `Y`.
X and Y should be instanciated.

### `Diff_int(X, Y)`

Checks if integer `X` is different from integer `Y`.
X and Y should be instanciated.

### `Between(X, Y, Z)`

Checks if integer `Z` is between integers `X` and `Y`, inclusive.
X, Y should be instanciated, and Z should be a variable or an integer.

## Lists

### `Length(?L, ?N)`

True if Length represents the number of elements in List. This predicate is a true relation and can be used to find the length of a list or produce a list (holding variables) of length Length.

### `Is_list(?L)`

Checks if `L` is a list. If `L` is a variable, returns false.
