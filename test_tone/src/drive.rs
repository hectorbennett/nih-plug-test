// https://www.elementary.audio/resources/distortion-saturation-wave-shaping

pub fn drive(x: f32, amount: f32) -> f32 {
    (x * amount).tanh()
}

#[test]
fn test_drive() {
    assert_eq!(drive(0.0, 1.0), 0.0);
}
