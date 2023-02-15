use rand::random;

pub type Field = fn(f64, f64, f64) -> (f64, f64);

pub const FIELDS: [Field; 12] = [
    // Fall
    |_x, _y, _| (0.0, -1.0),
    // Disperse
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
    |x, y, t| {
        let f1 = (y, x);
        let f2 = (y / (x * x + y * y).sqrt(), -x / (x * x + y * y).sqrt());
        (
            f1.0 * t.cos() + f2.0 * t.sin(),
            f1.1 * t.cos() + f2.1 * t.sin(),
        )
    },
    |x, y, t| {
        let f1 = (y, x);
        let f2 = (y / (x * x + y * y).sqrt(), -x / (x * x + y * y).sqrt());
        (
            f1.0 * t.cos() + f2.0 * t.sin(),
            f1.1 * t.sin() + f2.1 * t.cos(),
        )
    },
    |x, y, t| {
        let f1 = (y / (x * x + y * y).sqrt(), -x / (x * x + y * y).sqrt());
        let f2 = (
            y * t.cos() / (x * x + y * y).sqrt(),
            -x * t.cos() / (x * x + y * y).sqrt(),
        );

        (
            f1.0 * t.cos() + f2.0 * t.sin(),
            f1.1 * t.cos() + f2.1 * t.sin(),
        )
    },
    |x, y, t| {
        let f1 = (y / (x * x + y * y).sqrt(), -x / (x * x + y * y).sqrt());
        let f2 = (
            y * t.cos() / (x * x + y * y).sqrt(),
            -x * t.cos() / (x * x + y * y).sqrt(),
        );

        (
            f1.0 * t.cos() + f2.0 * t.sin(),
            f1.1 * t.sin() + f2.1 * t.cos(),
        )
    },
    |x, y, t| {
        let f1 = (y / (x * x + y * y).sqrt(), -x / (x * x + y * y).sqrt());
        let f2 = (
            y * t.cos() / (x * x + y * y).sqrt(),
            -x * t.cos() / (x * x + y * y).sqrt(),
        );

        (
            f1.0 * t.cos() + f2.0 * t.sin(),
            f1.0 * t.sin() + f2.0 * t.cos(),
        )
    },
    |_x, _y, _| (random(), random()),
    |_x, _y, _| (random::<f64>() * 2.0 - 1.0, random::<f64>() * 2.0 - 1.0),
];
