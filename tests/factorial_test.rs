use puruspe::utils::factorial;
use approx::assert_ulps_eq;

#[test]
fn test_factorial() {
    for i in 0..90 {
        assert_ulps_eq!(
            factorial(i),
            (2..=i).map(|j| j as f64).product()
        );
    }

    // Largest factorial that fits in a f64.
    assert_eq!(factorial(170), 7257415615307998967396728211129263114716991681296451376543577798900561843401706157852350749242617459511490991237838520776666022565442753025328900773207510902400430280058295603966612599658257104398558294257568966313439612262571094946806711205568880457193340212661452800000000000000000000000000000000000000000.0)
}