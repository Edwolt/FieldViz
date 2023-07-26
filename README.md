# Vector Field Vizualization

A program that let you visualize a vector field
(a math function that have a vector as input
and a vector as output).
It works by sampling particles in the field and moving it
according to the value of the vector in the field in the position

# Execution
To run the program, use the following command:
```sh
cargo run --release
```

During execution, you can interactively change the visualized field using arrows

# Changing Fields
To modify the vector fields, you'll need to edit the source code.
Inside the file `field.rs`, there's an array of fields
containing functions of the following type:
```rs
fn (f64, f64, f64) -> (f64, f64)
```

The first two parameters is the position in field,
and the last parameter is the time from the start of the vizualization,
so it's possible to make field varying by time
```math
\vec{F}(x, y, t) = (x, y)
```

# Example
* $\vec{F}(x, y) = (y, x)$:
![Field F(x, y) = (y, x)](images/yx_field.png)

* $\vec{F}(x, y) = (\frac{y}{\sqrt{x^2 + y^2}}, -\frac{x}{\sqrt{x^2 + y^2}})$
![Field Circle](images/circle_field.png)
