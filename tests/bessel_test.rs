use approx::assert_relative_eq;
use puruspe::{
    besselik, besseljy, CachedBesselIK, CachedBesselJY, CachedInuKnu, CachedJnuYnu, In, Inu_Knu,
    Jn, Jnu_Ynu, Kn, Yn,
};

// epsilon in the assertion has been set to the smallest magnitude for which the tests pass.

#[test]
fn jn_test() {
    for (n, x, ans) in J_TABLE {
        let result = Jn(n, x);
        let abs_eps = f64::EPSILON;
        let rel_eps = 1e-14;
        assert_relative_eq!(result, ans, epsilon = abs_eps, max_relative = rel_eps);
    }
}

#[test]
fn yn_test() {
    for (n, x, ans) in Y_TABLE {
        let result = Yn(n, x);
        let abs_eps = f64::EPSILON;
        let rel_eps = 1e-13;
        assert_relative_eq!(result, ans, epsilon = abs_eps, max_relative = rel_eps);
    }
}

#[test]
fn kn_test() {
    for (n, x, ans) in K_TABLE {
        let result = Kn(n, x);
        let abs_eps = f64::EPSILON;
        let rel_eps = 1e-14;
        assert_relative_eq!(result, ans, epsilon = abs_eps, max_relative = rel_eps);
    }
}

#[test]
fn in_test() {
    for (n, x, ans) in I_TABLE {
        let result = In(n, x);
        let abs_eps = f64::EPSILON;
        let rel_eps = 1e-14;
        assert_relative_eq!(result, ans, epsilon = abs_eps, max_relative = rel_eps);
    }
}

#[test]
fn test_jnu_ynu() {
    for &(nu, x, expected_jnu, expected_ynu) in JNU_YNU_TABLE.iter() {
        let (jnu, ynu) = Jnu_Ynu(nu, x);
        let abs_eps = f64::EPSILON;
        let rel_eps = 1e-12;

        assert_relative_eq!(jnu, expected_jnu, epsilon = abs_eps, max_relative = rel_eps);
        assert_relative_eq!(ynu, expected_ynu, epsilon = abs_eps, max_relative = rel_eps);
    }
}

#[test]
fn cached_jnuynu_test() {
    const ABS_EPS: f64 = f64::EPSILON;
    const REL_EPS: f64 = 1e-12;

    let mut cache = CachedJnuYnu::with_capacity(JNU_YNU_TABLE.len());
    for (nu, x, expected_jnu, expected_ynu) in JNU_YNU_TABLE {
        let (jnu, ynu) = cache.Jnu_Ynu(nu, x);

        assert_relative_eq!(jnu, expected_jnu, epsilon = ABS_EPS, max_relative = REL_EPS);
        assert_relative_eq!(ynu, expected_ynu, epsilon = ABS_EPS, max_relative = REL_EPS);

        assert!(cache.contains(nu, x));
        let &(cached_jnu, cached_ynu) = cache.get(nu, x).unwrap();

        assert_eq!(cached_jnu, jnu);
        assert_eq!(cached_ynu, ynu);
    }
}

#[test]
fn test_inu_knu() {
    for &(nu, x, expected_inu, expected_knu) in INU_KNU_TABLE.iter() {
        let (inu, knu) = Inu_Knu(nu, x);
        let abs_eps = f64::EPSILON;
        let rel_eps = 1e-13;

        assert_relative_eq!(inu, expected_inu, epsilon = abs_eps, max_relative = rel_eps);
        assert_relative_eq!(knu, expected_knu, epsilon = abs_eps, max_relative = rel_eps);
    }
}

#[test]
fn cached_inuknu_test() {
    const ABS_EPS: f64 = f64::EPSILON;
    const REL_EPS: f64 = 1e-12;

    let mut cache = CachedInuKnu::with_capacity(INU_KNU_TABLE.len());
    for (nu, x, expected_inu, expected_knu) in INU_KNU_TABLE {
        let (inu, knu) = cache.Inu_Knu(nu, x);

        assert_relative_eq!(inu, expected_inu, epsilon = ABS_EPS, max_relative = REL_EPS);
        assert_relative_eq!(knu, expected_knu, epsilon = ABS_EPS, max_relative = REL_EPS);

        assert!(cache.contains(nu, x));
        let &(cached_inu, cached_knu) = cache.get(nu, x).unwrap();

        assert_eq!(cached_inu, inu);
        assert_eq!(cached_knu, knu);
    }
}

#[test]
fn test_besseljy() {
    for &(nu, x, expected_j, expected_y, expected_jp, expected_yp) in BESSELJY_TABLE.iter() {
        let (j, y, jp, yp) = besseljy(nu, x);
        let abs_eps = f64::EPSILON;
        let rel_eps = 1e-12;
        assert_relative_eq!(j, expected_j, epsilon = abs_eps, max_relative = rel_eps);
        assert_relative_eq!(y, expected_y, epsilon = abs_eps, max_relative = rel_eps);
        assert_relative_eq!(jp, expected_jp, epsilon = abs_eps, max_relative = rel_eps);
        assert_relative_eq!(yp, expected_yp, epsilon = abs_eps, max_relative = rel_eps);
    }
}

#[test]
fn test_cached_besseljy() {
    const ABS_EPS: f64 = f64::EPSILON;
    const REL_EPS: f64 = 1e-12;

    let mut cache = CachedBesselJY::with_capacity(BESSELJY_TABLE.len());

    for &(nu, x, expected_j, expected_y, expected_jp, expected_yp) in BESSELJY_TABLE.iter() {
        let (j, y, jp, yp) = cache.besseljy(nu, x);

        assert_relative_eq!(j, expected_j, epsilon = ABS_EPS, max_relative = REL_EPS);
        assert_relative_eq!(y, expected_y, epsilon = ABS_EPS, max_relative = REL_EPS);
        assert_relative_eq!(jp, expected_jp, epsilon = ABS_EPS, max_relative = REL_EPS);
        assert_relative_eq!(yp, expected_yp, epsilon = ABS_EPS, max_relative = REL_EPS);

        assert!(cache.contains(nu, x));
        let &(cached_j, cached_y, cached_jp, cached_yp) = cache.get(nu, x).unwrap();
        assert_eq!(cached_j, j);
        assert_eq!(cached_jp, jp);
        assert_eq!(cached_y, y);
        assert_eq!(cached_yp, yp);
    }
}

#[test]
fn test_besselik() {
    for &(nu, x, expected_i, expected_k, expected_ip, expected_kp) in BESSELIK_TABLE.iter() {
        let (i, k, ip, kp) = besselik(nu, x);
        let abs_eps = f64::EPSILON;
        let rel_eps = 1e-13;
        assert_relative_eq!(i, expected_i, epsilon = abs_eps, max_relative = rel_eps);
        assert_relative_eq!(k, expected_k, epsilon = abs_eps, max_relative = rel_eps);
        assert_relative_eq!(ip, expected_ip, epsilon = abs_eps, max_relative = rel_eps);
        assert_relative_eq!(kp, expected_kp, epsilon = abs_eps, max_relative = rel_eps);
    }
}

#[test]
fn test_cached_besselik() {
    const ABS_EPS: f64 = f64::EPSILON;
    const REL_EPS: f64 = 1e-12;

    let mut cache = CachedBesselIK::with_capacity(BESSELIK_TABLE.len());

    for &(nu, x, expected_i, expected_k, expected_ip, expected_kp) in BESSELIK_TABLE.iter() {
        let (i, k, ip, kp) = cache.besselik(nu, x);

        assert_relative_eq!(i, expected_i, epsilon = ABS_EPS, max_relative = REL_EPS);
        assert_relative_eq!(k, expected_k, epsilon = ABS_EPS, max_relative = REL_EPS);
        assert_relative_eq!(ip, expected_ip, epsilon = ABS_EPS, max_relative = REL_EPS);
        assert_relative_eq!(kp, expected_kp, epsilon = ABS_EPS, max_relative = REL_EPS);

        assert!(cache.contains(nu, x));
        let &(cached_i, cached_k, cached_ip, cached_kp) = cache.get(nu, x).unwrap();

        assert_eq!(cached_i, i);
        assert_eq!(cached_ip, ip);
        assert_eq!(cached_k, k);
        assert_eq!(cached_kp, kp);
    }
}

// ┌─────────────────────────────────────────────────────────┐
//  Tables from scripts/bessel_test.py
// └─────────────────────────────────────────────────────────┘
const J_TABLE: [(u32, f64, f64); 42] = [
    (0, 1.00000000000000e-01, 9.97501562066040e-01),
    (0, 2.00000000000000e-01, 9.90024972239576e-01),
    (0, 5.00000000000000e-01, 9.38469807240813e-01),
    (0, 1.00000000000000e+00, 7.65197686557967e-01),
    (0, 1.50000000000000e+00, 5.11827671735918e-01),
    (0, 2.00000000000000e+00, 2.23890779141236e-01),
    (0, 2.50000000000000e+00, -4.83837764681979e-02),
    (0, 3.00000000000000e+00, -2.60051954901933e-01),
    (0, 4.00000000000000e+00, -3.97149809863847e-01),
    (0, 5.00000000000000e+00, -1.77596771314338e-01),
    (0, 2.50000000000000e-01, 9.84435929295853e-01),
    (0, 7.50000000000000e-01, 8.64242275166649e-01),
    (0, 1.00000000000000e+01, -2.45935764451348e-01),
    (0, 2.00000000000000e+01, 1.67024664340583e-01),
    (5, 1.00000000000000e-01, 2.60308179096444e-09),
    (5, 2.00000000000000e-01, 8.31945436094694e-08),
    (5, 5.00000000000000e-01, 8.05362724135748e-06),
    (5, 1.00000000000000e+00, 2.49757730211235e-04),
    (5, 1.50000000000000e+00, 1.79942176736061e-03),
    (5, 2.00000000000000e+00, 7.03962975587169e-03),
    (5, 2.50000000000000e+00, 1.95016251345032e-02),
    (5, 3.00000000000000e+00, 4.30284348770476e-02),
    (5, 4.00000000000000e+00, 1.32086656047098e-01),
    (5, 5.00000000000000e+00, 2.61140546120170e-01),
    (5, 2.50000000000000e-01, 2.53651615874724e-07),
    (5, 7.50000000000000e-01, 6.03641665105765e-05),
    (5, 1.00000000000000e+01, -2.34061528186794e-01),
    (5, 2.00000000000000e+01, 1.51169767982395e-01),
    (20, 1.00000000000000e-01, 3.91943772085861e-45),
    (20, 2.00000000000000e-01, 4.10836077398096e-39),
    (20, 5.00000000000000e-01, 3.72720196170470e-31),
    (20, 1.00000000000000e+00, 3.87350300852465e-25),
    (20, 1.50000000000000e+00, 1.26899721893326e-21),
    (20, 2.00000000000000e+00, 3.91897280509075e-19),
    (20, 2.50000000000000e+00, 3.30907938365879e-17),
    (20, 3.00000000000000e+00, 1.22759467379930e-15),
    (20, 4.00000000000000e+00, 3.55951162859386e-13),
    (20, 5.00000000000000e+00, 2.77033005212894e-11),
    (20, 2.50000000000000e-01, 3.56248055105870e-37),
    (20, 7.50000000000000e-01, 1.23478706936335e-27),
    (20, 1.00000000000000e+01, 1.15133692478134e-05),
    (20, 2.00000000000000e+01, 1.64747773775327e-01),
];

const Y_TABLE: [(u32, f64, f64); 28] = [
    (0, 1.00000000000000e-01, -1.53423865135037e+00),
    (0, 2.00000000000000e-01, -1.08110532237211e+00),
    (0, 5.00000000000000e-01, -4.44518733506707e-01),
    (0, 1.00000000000000e+00, 8.82569642156770e-02),
    (0, 1.50000000000000e+00, 3.82448923797759e-01),
    (0, 2.00000000000000e+00, 5.10375672649745e-01),
    (0, 2.50000000000000e+00, 4.98070359615232e-01),
    (0, 3.00000000000000e+00, 3.76850010012791e-01),
    (0, 4.00000000000000e+00, -1.69407393250648e-02),
    (0, 5.00000000000000e+00, -3.08517625249033e-01),
    (0, 2.50000000000000e-01, -9.31573024930059e-01),
    (0, 7.50000000000000e-01, -1.37172769385772e-01),
    (0, 1.00000000000000e+01, 5.56711672835996e-02),
    (0, 2.00000000000000e+01, 6.26405968093837e-02),
    (5, 1.00000000000000e-01, -2.44614845023039e+07),
    (5, 2.00000000000000e-01, -7.65856774575641e+05),
    (5, 5.00000000000000e-01, -7.94630147880748e+03),
    (5, 1.00000000000000e+00, -2.60405866625812e+02),
    (5, 1.50000000000000e+00, -3.71903083954981e+01),
    (5, 2.00000000000000e+00, -9.93598912848198e+00),
    (5, 2.50000000000000e+00, -3.83017600074075e+00),
    (5, 3.00000000000000e+00, -1.90594595382867e+00),
    (5, 4.00000000000000e+00, -7.95851421114201e-01),
    (5, 5.00000000000000e+00, -4.53694822491101e-01),
    (5, 2.50000000000000e-01, -2.51309481518524e+05),
    (5, 7.50000000000000e-01, -1.06724689522898e+03),
    (5, 1.00000000000000e+01, 1.35403047689363e-01),
    (5, 2.00000000000000e+01, -1.00035767889532e-01),
];

const K_TABLE: [(u32, f64, f64); 26] = [
    (0, 2.00000000000000e-01, 1.75270385552815e+00),
    (0, 5.00000000000000e-01, 9.24419071227666e-01),
    (0, 1.00000000000000e+00, 4.21024438240708e-01),
    (0, 1.50000000000000e+00, 2.13805562647526e-01),
    (0, 2.00000000000000e+00, 1.13893872749533e-01),
    (0, 2.50000000000000e+00, 6.23475532003662e-02),
    (0, 3.00000000000000e+00, 3.47395043862792e-02),
    (0, 4.00000000000000e+00, 1.11596760858530e-02),
    (0, 5.00000000000000e+00, 3.69109833404259e-03),
    (0, 2.50000000000000e-01, 1.54150675124830e+00),
    (0, 7.50000000000000e-01, 6.10582422116464e-01),
    (0, 1.00000000000000e+01, 1.77800623161677e-05),
    (0, 2.00000000000000e+01, 5.74123781533652e-10),
    (5, 2.00000000000000e-01, 1.19700499168726e+06),
    (5, 5.00000000000000e-01, 1.20979794760964e+04),
    (5, 1.00000000000000e+00, 3.60960589601241e+02),
    (5, 1.50000000000000e+00, 4.40677811593011e+01),
    (5, 2.00000000000000e+00, 9.43104910059647e+00),
    (5, 2.50000000000000e+00, 2.71688429078654e+00),
    (5, 3.00000000000000e+00, 9.37773602386808e-01),
    (5, 4.00000000000000e+00, 1.54342548725997e-01),
    (5, 5.00000000000000e+00, 3.27062737120319e-02),
    (5, 2.50000000000000e-01, 3.91683989623349e+05),
    (5, 7.50000000000000e-01, 1.56258703396911e+03),
    (5, 1.00000000000000e+01, 5.75418499853123e-05),
    (5, 2.00000000000000e+01, 1.05386601399742e-09),
];

const I_TABLE: [(u32, f64, f64); 26] = [
    (0, 2.00000000000000e-01, 1.01002502779515e+00),
    (0, 5.00000000000000e-01, 1.06348337074132e+00),
    (0, 1.00000000000000e+00, 1.26606587775201e+00),
    (0, 1.50000000000000e+00, 1.64672318977289e+00),
    (0, 2.00000000000000e+00, 2.27958530233607e+00),
    (0, 2.50000000000000e+00, 3.28983914405013e+00),
    (0, 3.00000000000000e+00, 4.88079258586503e+00),
    (0, 4.00000000000000e+00, 1.13019219521363e+01),
    (0, 5.00000000000000e+00, 2.72398718236045e+01),
    (0, 2.50000000000000e-01, 1.01568614122361e+00),
    (0, 7.50000000000000e-01, 1.14564677804400e+00),
    (0, 1.00000000000000e+01, 2.81571662846625e+03),
    (0, 2.00000000000000e+01, 4.35582825595535e+07),
    (5, 2.00000000000000e-01, 8.34723214699189e-08),
    (5, 5.00000000000000e-01, 8.22317131310926e-06),
    (5, 1.00000000000000e+00, 2.71463155956972e-04),
    (5, 1.50000000000000e+00, 2.17055956909756e-03),
    (5, 2.00000000000000e+00, 9.82567932313170e-03),
    (5, 2.50000000000000e+00, 3.28434751720232e-02),
    (5, 3.00000000000000e+00, 9.12064776615134e-02),
    (5, 4.00000000000000e+00, 5.04724363113166e-01),
    (5, 5.00000000000000e+00, 2.15797454732255e+00),
    (5, 2.50000000000000e-01, 2.54976164498828e-07),
    (5, 7.50000000000000e-01, 6.32611227398117e-05),
    (5, 1.00000000000000e+01, 7.77188286403260e+02),
    (5, 2.00000000000000e+01, 2.30183922134137e+07),
];

const JNU_YNU_TABLE: [(f64, f64, f64, f64); 40] = [
    (0.5, 0.1, 2.51892940326001e-01, -2.51052736895851e+00),
    (0.5, 0.5, 5.40973789934529e-01, -9.90245880243405e-01),
    (0.5, 1.0, 6.71396707141804e-01, -4.31098868018376e-01),
    (0.5, 2.0, 5.13016136561828e-01, 2.34785710406249e-01),
    (0.5, 5.0, -3.42167984798163e-01, -1.01217709185108e-01),
    (0.5, 10.0, -1.37263735755050e-01, 2.11708866331398e-01),
    (0.5, 20.0, 1.62880763855031e-01, -7.28069047850618e-02),
    (0.5, 50.0, -2.96058318889246e-02, -1.08884756350540e-01),
    (1.5, 0.1, 8.40203430150015e-03, -2.53571666299111e+01),
    (1.5, 0.5, 9.17016996256514e-02, -2.52146555042134e+00),
    (1.5, 1.0, 2.40297839123427e-01, -1.10249557516018e+00),
    (1.5, 2.0, 4.91293778687163e-01, -3.95623281358704e-01),
    (1.5, 5.0, -1.69651306144741e-01, 3.21924442961140e-01),
    (1.5, 10.0, 1.97982492755892e-01, 1.58434622388190e-01),
    (1.5, 20.0, -6.46628665923111e-02, -1.66521109094283e-01),
    (1.5, 50.0, -1.09476872988318e-01, 2.74281367619138e-02),
    (2.5, 0.1, 1.68088719003341e-04, -7.58204471528374e+02),
    (2.5, 0.5, 9.23640781937973e-03, -1.41385474222846e+01),
    (2.5, 1.0, 4.94968102284780e-02, -2.87638785746216e+00),
    (2.5, 2.0, 2.23924531468916e-01, -8.28220632444304e-01),
    (2.5, 5.0, 2.40377201111318e-01, 2.94372374961793e-01),
    (2.5, 10.0, 1.96658483581818e-01, -1.64178479614941e-01),
    (2.5, 20.0, -1.72580193843878e-01, 4.78287384209194e-02),
    (2.5, 50.0, 2.30372195096255e-02, 1.10530444556254e-01),
    (5.5, 0.1, 2.42632250905068e-10, -2.38568535112822e+08),
    (5.5, 0.5, 1.67985579649158e-06, -3.46003723231775e+04),
    (5.5, 1.0, 7.38531193859482e-05, -7.97438019436180e+02),
    (5.5, 2.0, 2.97347067050333e-03, -2.09781995753435e+01),
    (5.5, 5.0, 1.90564369028837e-01, -5.71749418290235e-01),
    (5.5, 10.0, -1.40120932366592e-01, 2.36754460665841e-01),
    (5.5, 20.0, 5.95323254540897e-02, -1.71890894731253e-01),
    (5.5, 50.0, -1.13110423458543e-01, -3.93304003995588e-03),
    (10.5, 0.1, 1.83469858800356e-21, -1.65240301466198e+19),
    (10.5, 0.5, 3.98550515718813e-14, -7.61508842905838e+11),
    (10.5, 1.0, 5.67818747763465e-11, -5.36349976627599e+08),
    (10.5, 2.0, 7.70152730519648e-08, -4.01042565823492e+05),
    (10.5, 5.0, 7.26752689741490e-04, -4.75578165541700e+01),
    (10.5, 10.0, 1.63007366390325e-01, -4.35123468587179e-01),
    (10.5, 20.0, 1.41611992284732e-01, -1.31466434375494e-01),
    (10.5, 50.0, -8.48497209435532e-02, 7.63048781453420e-02),
];
const INU_KNU_TABLE: [(f64, f64, f64, f64); 40] = [
    (0.5, 0.1, 2.52733984600132e-01, 3.58616683879726e+00),
    (0.5, 0.5, 5.87993086790416e-01, 1.07504760349992e+00),
    (0.5, 1.0, 9.37674888245488e-01, 4.61068504447895e-01),
    (0.5, 2.0, 2.04623686308906e+00, 1.19937771968061e-01),
    (0.5, 5.0, 2.64775474975591e+01, 3.77661337464288e-03),
    (0.5, 10.0, 2.77878460387457e+03, 1.79934780937052e-05),
    (0.5, 20.0, 4.32797462724289e+07, 5.77637397470745e-10),
    (0.5, 50.0, 2.92515685299129e+20, 3.41862009545707e-23),
    (1.5, 0.1, 8.41885518609277e-03, 3.94478352267699e+01),
    (1.5, 0.5, 9.64034738340168e-02, 3.22514281049976e+00),
    (1.5, 1.0, 2.93525326347480e-01, 9.22137008895789e-01),
    (1.5, 2.0, 1.09947318863311e+00, 1.79906657952092e-01),
    (1.5, 5.0, 2.11844422647941e+01, 4.53193604957146e-03),
    (1.5, 10.0, 2.50090615494212e+03, 1.97928259030757e-05),
    (1.5, 20.0, 4.11157589588075e+07, 6.06519267344282e-10),
    (1.5, 50.0, 2.86665371593146e+20, 3.48699249736622e-23),
    (2.5, 0.1, 1.68329017348885e-04, 1.18702122364189e+03),
    (2.5, 0.5, 9.57224378631588e-03, 2.04259044664985e+01),
    (2.5, 1.0, 5.70989092030482e-02, 3.22747953113526e+00),
    (2.5, 2.0, 3.97027080139391e-01, 3.89797758896200e-01),
    (2.5, 5.0, 1.37668821386826e+01, 6.49577500438576e-03),
    (2.5, 10.0, 2.02851275739194e+03, 2.39313258646279e-05),
    (2.5, 20.0, 3.71123824286078e+07, 6.68615287572387e-10),
    (2.5, 50.0, 2.75315763003540e+20, 3.62783964529905e-23),
    (5.5, 0.1, 2.42818962901460e-10, 3.74326429228270e+08),
    (5.5, 0.5, 1.71247337250925e-06, 5.28611657116946e+04),
    (5.5, 1.0, 7.97584358338079e-05, 1.12085753431283e+03),
    (5.5, 2.0, 4.04506814035155e-03, 2.10903075895088e+01),
    (5.5, 5.0, 1.32942379428402e+00, 5.05099379178238e-02),
    (5.5, 10.0, 5.97577653628482e+02, 7.33045300798502e-05),
    (5.5, 20.0, 2.01515232366678e+07, 1.19640348019984e-09),
    (5.5, 50.0, 2.16107129593534e+20, 4.59980196488973e-23),
    (10.5, 0.1, 1.83549645647906e-21, 2.59422284599591e+19),
    (10.5, 0.5, 4.02906216452201e-14, 1.18053923199853e+12),
    (10.5, 1.0, 5.93051121645790e-11, 7.99301031088060e+08),
    (10.5, 2.0, 9.16449610994113e-08, 5.10351414719992e+05),
    (10.5, 5.0, 2.15774415219657e-03, 1.99147112653683e+01),
    (10.5, 10.0, 1.37877627637049e+01, 2.49982455913333e-03),
    (10.5, 20.0, 2.75313149701346e+06, 8.03978709166383e-09),
    (10.5, 50.0, 9.66910969340875e+19, 1.01218099858594e-22),
];

const BESSELJY_TABLE: [(f64, f64, f64, f64, f64, f64); 40] = [
    (
        0.5,
        0.1,
        2.51892940326001e-01,
        -2.51052736895851e+00,
        1.25106266732850e+00,
        1.28045297851186e+01,
    ),
    (
        0.5,
        0.5,
        5.40973789934529e-01,
        -9.90245880243405e-01,
        4.49272090308876e-01,
        1.53121967017793e+00,
    ),
    (
        0.5,
        1.0,
        6.71396707141804e-01,
        -4.31098868018376e-01,
        9.54005144474742e-02,
        8.86946141150992e-01,
    ),
    (
        0.5,
        2.0,
        5.13016136561828e-01,
        2.34785710406249e-01,
        -3.63039744546706e-01,
        4.54319708960266e-01,
    ),
    (
        0.5,
        5.0,
        -3.42167984798163e-01,
        -1.01217709185108e-01,
        1.35434507664925e-01,
        -3.32046213879652e-01,
    ),
    (
        0.5,
        10.0,
        -1.37263735755050e-01,
        2.11708866331398e-01,
        -2.04845679543646e-01,
        -1.47849179071620e-01,
    ),
    (
        0.5,
        20.0,
        1.62880763855031e-01,
        -7.28069047850618e-02,
        6.87348856886860e-02,
        1.64700936474658e-01,
    ),
    (
        0.5,
        50.0,
        -2.96058318889246e-02,
        -1.08884756350540e-01,
        1.09180814669429e-01,
        -2.85169843254192e-02,
    ),
    (
        1.5,
        0.1,
        8.40203430150015e-03,
        -2.53571666299111e+01,
        1.25862425803499e-01,
        3.77846972079708e+02,
    ),
    (
        1.5,
        0.5,
        9.17016996256514e-02,
        -2.52146555042134e+00,
        2.65868691057574e-01,
        6.57415077102061e+00,
    ),
    (
        1.5,
        1.0,
        2.40297839123427e-01,
        -1.10249557516018e+00,
        3.10949948456663e-01,
        1.22264449472189e+00,
    ),
    (
        1.5,
        2.0,
        4.91293778687163e-01,
        -3.95623281358704e-01,
        1.44545802546456e-01,
        5.31503171425276e-01,
    ),
    (
        1.5,
        5.0,
        -1.69651306144741e-01,
        3.21924442961140e-01,
        -2.91272592954741e-01,
        -1.97795042073450e-01,
    ),
    (
        1.5,
        10.0,
        1.97982492755892e-01,
        1.58434622388190e-01,
        -1.66961109668434e-01,
        1.87943672973170e-01,
    ),
    (
        1.5,
        20.0,
        -6.46628665923111e-02,
        -1.66521109094283e-01,
        1.67730478849455e-01,
        -6.03178216029906e-02,
    ),
    (
        1.5,
        50.0,
        -1.09476872988318e-01,
        2.74281367619138e-02,
        -2.63215256992751e-02,
        -1.09707600453397e-01,
    ),
    (
        2.5,
        0.1,
        1.68088719003341e-04,
        -7.58204471528374e+02,
        4.19981632641662e-03,
        1.89297546215795e+04,
    ),
    (
        2.5,
        0.5,
        9.23640781937973e-03,
        -1.41385474222846e+01,
        4.55196605287527e-02,
        6.81712715610018e+01,
    ),
    (
        2.5,
        1.0,
        4.94968102284780e-02,
        -2.87638785746216e+00,
        1.16555813552232e-01,
        6.08847406849523e+00,
    ),
    (
        2.5,
        2.0,
        2.23924531468916e-01,
        -8.28220632444304e-01,
        2.11388114351018e-01,
        6.39652509196676e-01,
    ),
    (
        2.5,
        5.0,
        2.40377201111318e-01,
        2.94372374961793e-01,
        -2.89839906700400e-01,
        1.74738255480244e-01,
    ),
    (
        2.5,
        10.0,
        1.96658483581818e-01,
        -1.64178479614941e-01,
        1.48817871860438e-01,
        1.99479242291926e-01,
    ),
    (
        2.5,
        20.0,
        -1.72580193843878e-01,
        4.78287384209194e-02,
        -4.30903423618264e-02,
        -1.72499701396898e-01,
    ),
    (
        2.5,
        50.0,
        2.30372195096255e-02,
        1.10530444556254e-01,
        -1.10628733963799e-01,
        2.19016145341011e-02,
    ),
    (
        5.5,
        0.1,
        2.42632250905068e-10,
        -2.38568535112822e+08,
        1.33429073021272e-08,
        1.31186182487610e+10,
    ),
    (
        5.5,
        0.5,
        1.67985579649158e-06,
        -3.46003723231775e+04,
        1.84137209020613e-05,
        3.78674137980961e+05,
    ),
    (
        5.5,
        1.0,
        7.38531193859482e-05,
        -7.97438019436180e+02,
        4.00481747638247e-04,
        4.29582938877140e+03,
    ),
    (
        5.5,
        2.0,
        2.97347067050333e-03,
        -2.09781995753435e+01,
        7.70984913514483e-03,
        5.26560204155066e+01,
    ),
    (
        5.5,
        5.0,
        1.90564369028837e-01,
        -5.71749418290235e-01,
        1.24041903115444e-01,
        2.95979089958379e-01,
    ),
    (
        5.5,
        10.0,
        -1.40120932366592e-01,
        2.36754460665841e-01,
        -1.89349246455681e-01,
        -1.34403177288234e-01,
    ),
    (
        5.5,
        20.0,
        5.95323254540897e-02,
        -1.71890894731253e-01,
        1.63740040689973e-01,
        6.19086604250048e-02,
    ),
    (
        5.5,
        50.0,
        -1.13110423458543e-01,
        -3.93304003995588e-03,
        5.05421036231352e-03,
        -1.12390323074740e-01,
    ),
    (
        10.5,
        0.1,
        1.83469858800356e-21,
        -1.65240301466198e+19,
        1.92635374651256e-19,
        1.73493619412264e+21,
    ),
    (
        10.5,
        0.5,
        3.98550515718813e-14,
        -7.61508842905838e+11,
        8.36089291839495e-13,
        1.59716304567426e+13,
    ),
    (
        10.5,
        1.0,
        5.67818747763465e-11,
        -5.36349976627599e+08,
        5.93736600581288e-10,
        5.60335779288845e+09,
    ),
    (
        10.5,
        2.0,
        7.70152730519648e-08,
        -4.01042565823492e+05,
        3.97586006111247e-07,
        2.06272039119088e+06,
    ),
    (
        10.5,
        5.0,
        7.26752689741490e-04,
        -4.75578165541700e+01,
        1.36071007625812e-03,
        8.61524906164340e+01,
    ),
    (
        10.5,
        10.0,
        1.63007366390325e-01,
        -4.35123468587179e-01,
        8.13987715594035e-02,
        1.73264939140635e-01,
    ),
    (
        10.5,
        20.0,
        1.41611992284732e-01,
        -1.31466434375494e-01,
        1.07221263975874e-01,
        1.25236507643921e-01,
    ),
    (
        10.5,
        50.0,
        -8.48497209435532e-02,
        7.63048781453420e-02,
        -7.37209080880956e-02,
        -8.37613896650714e-02,
    ),
];

const BESSELIK_TABLE: [(f64, f64, f64, f64, f64, f64); 40] = [
    (
        0.5,
        0.1,
        2.52733984600132e-01,
        3.58616683879726e+00,
        1.27208877818675e+00,
        -2.15170010327836e+01,
    ),
    (
        0.5,
        0.5,
        5.87993086790416e-01,
        1.07504760349992e+00,
        6.84396560624433e-01,
        -2.15009520699984e+00,
    ),
    (
        0.5,
        1.0,
        9.37674888245488e-01,
        4.61068504447895e-01,
        7.62362770470224e-01,
        -6.91602756671842e-01,
    ),
    (
        0.5,
        2.0,
        2.04623686308906e+00,
        1.19937771968061e-01,
        1.61103240440537e+00,
        -1.49922214960077e-01,
    ),
    (
        0.5,
        5.0,
        2.64775474975591e+01,
        3.77661337464288e-03,
        2.38321970145500e+01,
        -4.15427471210717e-03,
    ),
    (
        0.5,
        10.0,
        2.77878460387457e+03,
        1.79934780937052e-05,
        2.63984538513585e+03,
        -1.88931519983904e-05,
    ),
    (
        0.5,
        20.0,
        4.32797462724289e+07,
        5.77637397470745e-10,
        4.21977526156182e+07,
        -5.92078332407513e-10,
    ),
    (
        0.5,
        50.0,
        2.92515685299129e+20,
        3.41862009545707e-23,
        2.89590528446138e+20,
        -3.45280629641165e-23,
    ),
    (
        1.5,
        0.1,
        8.41885518609277e-03,
        3.94478352267699e+01,
        1.26451156808740e-01,
        -5.95303695240345e+02,
    ),
    (
        1.5,
        0.5,
        9.64034738340168e-02,
        3.22514281049976e+00,
        2.98782665288366e-01,
        -1.07504760349992e+01,
    ),
    (
        1.5,
        1.0,
        2.93525326347480e-01,
        9.22137008895789e-01,
        4.97386898724268e-01,
        -1.84427401779158e+00,
    ),
    (
        1.5,
        2.0,
        1.09947318863311e+00,
        1.79906657952092e-01,
        1.22163197161422e+00,
        -2.54867765432131e-01,
    ),
    (
        1.5,
        5.0,
        2.11844422647941e+01,
        4.53193604957146e-03,
        2.01222148181208e+01,
        -5.13619418951432e-03,
    ),
    (
        1.5,
        10.0,
        2.50090615494212e+03,
        1.97928259030757e-05,
        2.40364868063325e+03,
        -2.09624019791665e-05,
    ),
    (
        1.5,
        20.0,
        4.11157589588075e+07,
        6.06519267344282e-10,
        4.01960643505184e+07,
        -6.23126342521566e-10,
    ),
    (
        1.5,
        50.0,
        2.86665371593146e+20,
        3.48699249736622e-23,
        2.83915724151335e+20,
        -3.52322987037806e-23,
    ),
    (
        2.5,
        0.1,
        1.68329017348885e-04,
        1.18702122364189e+03,
        4.21062975237064e-03,
        -2.97149784262741e+04,
    ),
    (
        2.5,
        0.5,
        9.57224378631588e-03,
        2.04259044664985e+01,
        4.85422549024374e-02,
        -1.05354665142992e+02,
    ),
    (
        2.5,
        1.0,
        5.70989092030482e-02,
        3.22747953113526e+00,
        1.50778053339859e-01,
        -8.99083583673394e+00,
    ),
    (
        2.5,
        2.0,
        3.97027080139391e-01,
        3.89797758896200e-01,
        6.03189338458872e-01,
        -6.67153856572342e-01,
    ),
    (
        2.5,
        5.0,
        1.37668821386826e+01,
        6.49577500438576e-03,
        1.43010011954528e+01,
        -7.77982355176434e-03,
    ),
    (
        2.5,
        10.0,
        2.02851275739194e+03,
        2.39313258646279e-05,
        1.99377796559413e+03,
        -2.57756573692327e-05,
    ),
    (
        2.5,
        20.0,
        3.71123824286078e+07,
        6.68615287572387e-10,
        3.64767111552315e+07,
        -6.90096178290830e-10,
    ),
    (
        2.5,
        50.0,
        2.75315763003540e+20,
        3.62783964529905e-23,
        2.72899583442969e+20,
        -3.66838447963117e-23,
    ),
    (
        5.5,
        0.1,
        2.42818962901460e-10,
        3.74326429228270e+08,
        1.33569107019785e-08,
        -2.05921121302073e+10,
    ),
    (
        5.5,
        0.5,
        1.71247337250925e-06,
        5.28611657116946e+04,
        1.89029872071443e-05,
        -5.84398027357764e+05,
    ),
    (
        5.5,
        1.0,
        7.97584358338079e-05,
        1.12085753431283e+03,
        4.44775480292363e-04,
        -6.28736066090371e+03,
    ),
    (
        5.5,
        2.0,
        4.04506814035155e-03,
        2.10903075895088e+01,
        1.17339337572070e-02,
        -6.24285473232195e+01,
    ),
    (
        5.5,
        5.0,
        1.32942379428402e+00,
        5.05099379178238e-02,
        1.91993178841398e+00,
        -7.74955021895320e-02,
    ),
    (
        5.5,
        10.0,
        5.97577653628482e+02,
        7.33045300798502e-05,
        6.59190204523965e+02,
        -8.64797595933183e-05,
    ),
    (
        5.5,
        20.0,
        2.01515232366678e+07,
        1.19640348019984e-09,
        2.04275313654447e+07,
        -1.26841182586042e-09,
    ),
    (
        5.5,
        50.0,
        2.16107129593534e+20,
        4.59980196488973e-23,
        2.15265247407260e+20,
        -4.67278656610238e-23,
    ),
    (
        10.5,
        0.1,
        1.83549645647906e-21,
        2.59422284599591e+19,
        1.92735108210892e-19,
        -2.72407052211334e+21,
    ),
    (
        10.5,
        0.5,
        4.02906216452201e-14,
        1.18053923199853e+12,
        8.46978557116462e-13,
        -2.48223666904172e+13,
    ),
    (
        10.5,
        1.0,
        5.93051121645790e-11,
        7.99301031088060e+08,
        6.25277690934645e-10,
        -8.43459996385166e+09,
    ),
    (
        10.5,
        2.0,
        9.16449610994113e-08,
        5.10351414719992e+05,
        4.89050438884269e-07,
        -2.73241882196171e+06,
    ),
    (
        10.5,
        5.0,
        2.15774415219657e-03,
        1.99147112653683e+01,
        4.98144316706299e-03,
        -4.67136001923531e+01,
    ),
    (
        10.5,
        10.0,
        1.37877627637049e+01,
        2.49982455913333e-03,
        1.96739633755080e+01,
        -3.68577150980511e-03,
    ),
    (
        10.5,
        20.0,
        2.75313149701346e+06,
        8.03978709166383e-09,
        3.05560228784199e+06,
        -9.23806516199438e-09,
    ),
    (
        10.5,
        50.0,
        9.66910969340875e+19,
        1.01218099858594e-22,
        9.78704419219422e+19,
        -1.04391615737081e-22,
    ),
];
