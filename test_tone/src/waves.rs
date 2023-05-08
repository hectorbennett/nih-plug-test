use std::f32::consts::{PI, TAU};

pub fn sine(phase: f32) -> f32 {
    (phase * TAU).sin()
}

#[test]
fn test_sine() {
    assert_eq!(sine(0.0), 0.0);
    assert_eq!(sine(0.25), 1.0);
    assert_eq!(sine(0.5), PI.sin()); // effectively 0.0 but can't write that because of floating point error
    assert_eq!(sine(0.75), -1.0);
}

pub fn sawtooth(phase: f32) -> f32 {
    phase * 2.0 - 1.0
}

#[test]
fn test_sawtooth() {
    assert_eq!(sawtooth(0.0), -1.0);
    assert_eq!(sawtooth(0.25), -0.5);
    assert_eq!(sawtooth(0.5), 0.0);
    assert_eq!(sawtooth(0.75), 0.5);
    assert_eq!(sawtooth(1.0), 1.0);
}

pub fn triangle(phase: f32) -> f32 {
    if phase < 0.5 {
        return phase * 4.0 - 1.0;
    } else {
        return (1.0 - phase) * 4.0 - 1.0;
    }
}

#[test]
fn test_triangle() {
    assert_eq!(triangle(0.0), -1.0);
    assert_eq!(triangle(0.125), -0.5);
    assert_eq!(triangle(0.25), 0.0);
    assert_eq!(triangle(0.375), 0.5);
    assert_eq!(triangle(0.5), 1.0);

    assert_eq!(triangle(0.625), 0.5);
    assert_eq!(triangle(0.75), 0.0);
    assert_eq!(triangle(0.875), -0.5);
    assert_eq!(triangle(1.0), -1.0);
}

pub fn square(phase: f32) -> f32 {
    if phase < 0.5 {
        return -1.0;
    } else {
        return 1.0;
    }
}

#[test]
fn test_square() {
    assert_eq!(square(0.0), -1.0);
    assert_eq!(square(0.25), -1.0);
    assert_eq!(square(0.499), -1.0);

    assert_eq!(square(0.5), 1.0);
    assert_eq!(square(0.75), 1.0);
    assert_eq!(square(1.0), 1.0);
}
