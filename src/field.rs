pub type Field = fn(f64, f64, f64) -> (f64, f64);

pub const FIELDS: [Field; 5] = [
    // Fall
    |_x, _y, _| (0.0, -1.0),
    //
    |x, y, _| (x, y),
    // enter from one diagonal and go out in other
    |x, y, _| (y, x),
    // Circle
    |x, y, _| (y / (x * x + y * y).sqrt(), -x / (x * x + y * y).sqrt()),
    |x, y, t| {
        (
            y * t.cos() / (x * x + y * y).sqrt(),
            -x * t.cos() / (x * x + y * y).sqrt(),
        )
    },
];
