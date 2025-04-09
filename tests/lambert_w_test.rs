use core::f64;

use approx::{assert_abs_diff_eq, assert_relative_eq};
use puruspe::{lambert_w, lambert_w0, lambert_wm1, sp_lambert_w0, sp_lambert_wm1};

const LAMBERT_W0_TABLE: [(f64, f64); 20] = [
    (1.00000000000000e-01, 9.12765271608623e-02),
    (2.00000000000000e-01, 1.68915973499110e-01),
    (5.00000000000000e-01, 3.51733711249196e-01),
    (1.00000000000000e+00, 5.67143290409784e-01),
    (1.50000000000000e+00, 7.25861357766226e-01),
    (2.00000000000000e+00, 8.52605502013725e-01),
    (2.50000000000000e+00, 9.58586356728703e-01),
    (3.00000000000000e+00, 1.04990889496404e+00),
    (4.00000000000000e+00, 1.20216787319704e+00),
    (5.00000000000000e+00, 1.32672466524220e+00),
    (1.00000000000000e+01, 1.74552800274070e+00),
    (2.00000000000000e+01, 2.20500327802406e+00),
    (5.00000000000000e+01, 2.86089017798221e+00),
    (2.50000000000000e-01, 2.03888354702240e-01),
    (7.50000000000000e-01, 4.69150210694988e-01),
    (1.00000000000000e-05, 9.99990000149997e-06),
    (1.00000000000000e-10, 9.99999999900000e-11),
    (1.00000000000000e+05, 9.28457142862211e+00),
    (1.00000000000000e+10, 2.00286854133050e+01),
    (1.00000000000000e+308, 7.02641362034107e+02),
];
const LAMBERT_WM1_TABLE: [(f64, f64); 13] = [
    (-1.62330466849397e-01, -2.87373297576420e+00),
    (-1.41318890794931e-02, -6.06123496778854e+00),
    (-1.78131724758206e-01, -2.72926392333150e+00),
    (-1.03843592031058e-01, -3.52465080303420e+00),
    (-3.64876111085733e-01, -1.13356447829203e+00),
    (-5.46821782149597e-02, -4.38423187070222e+00),
    (-3.63270038872334e-01, -1.16731536489767e+00),
    (-1.02171094066010e-02, -6.44736274096470e+00),
    (-2.37699099525656e-01, -2.24582072744266e+00),
    (-3.03767654679841e-01, -1.75258268173280e+00),
    (-1.00000000000000e-03, -9.11800647040274e+00),
    (-3.10000000000000e-05, -1.29420012897721e+01),
    (-1.00000000000000e-100, -2.35721158875685e+02),
];
const BRANCH_POINT: (f64, f64) = (-0.36787944117144232, -1.0);

#[rustfmt::skip]
const COMPLEX_LAMBERT_W_TABLE: [(i32, (f64, f64), (f64, f64)); 153] = [
    (0, (1.00000000000000e-01, 2.00000000000000e-01), (1.16456459794844e-01, 1.61397090591358e-01)),
    (0, (-1.50000000000000e+00, -5.00000000000000e-01), (1.47043313306281e-01, -1.35698492904899e+00)),
    (0, (1.00000000000000e+00, 1.00000000000000e+00), (6.56966069230436e-01, 3.25450339413415e-01)),
    (0, (-2.00000000000000e+00, 2.00000000000000e+00), (6.74681027462047e-01, 1.27281226692619e+00)),
    (0, (1.00000000000000e-03, 1.00000000000000e-03), (9.99997010645834e-04, 9.98002999979253e-04)),
    (0, (-1.00000000000000e-05, -1.00000000000000e-05), (-9.99999999699989e-06, -1.00002000030000e-05)),
    (0, (1.00000000000000e-10, 1.00000000000000e-10), (1.00000000000000e-10, 9.99999999800000e-11)),
    (0, (1.00000000000000e+05, 1.00000000000000e+05), (9.59546746386683e+00, 7.11394912032392e-01)),
    (0, (-1.00000000000000e+10, -1.00000000000000e+10), (2.03531360860092e+01, -2.24627431934268e+00)),
    (1, (1.00000000000000e-01, 2.00000000000000e-01), (-3.32549863908025e+00, 5.25536186629760e+00)),
    (1, (-1.50000000000000e+00, -5.00000000000000e-01), (-1.40967033166704e-01, 1.81503624456954e+00)),
    (1, (1.00000000000000e+00, 1.00000000000000e+00), (-1.34284894070080e+00, 5.24724937429140e+00)),
    (1, (-2.00000000000000e+00, 2.00000000000000e+00), (-9.05853268153494e-01, 6.93876805629691e+00)),
    (1, (1.00000000000000e-03, 1.00000000000000e-03), (-8.85161007244910e+00, 4.38713759711072e+00)),
    (1, (-1.00000000000000e-05, -1.00000000000000e-05), (-1.37923465336253e+01, 8.46711143530535e-01)),
    (1, (1.00000000000000e-10, 1.00000000000000e-10), (-2.59475859423146e+01, 4.08306931120557e+00)),
    (1, (1.00000000000000e+05, 1.00000000000000e+05), (9.42334138291600e+00, 6.46712003142808e+00)),
    (1, (-1.00000000000000e+10, -1.00000000000000e+10), (2.03430228207593e+01, 3.74493944239079e+00)),
    (-1, (1.00000000000000e-01, 2.00000000000000e-01), (-2.89113637492848e+00, -2.80466094303430e+00)),
    (-1, (-1.50000000000000e+00, -5.00000000000000e-01), (-1.55492481040736e+00, -7.32300425533279e+00)),
    (-1, (1.00000000000000e+00, 1.00000000000000e+00), (-9.86969573221275e-01, -3.66385700328479e+00)),
    (-1, (-2.00000000000000e+00, 2.00000000000000e+00), (1.54029157303212e-01, -2.41976336082461e+00)),
    (-1, (1.00000000000000e-03, 1.00000000000000e-03), (-8.77690637064100e+00, -2.64935215349900e+00)),
    (-1, (-1.00000000000000e-05, -1.00000000000000e-05), (-1.38798307734598e+01, -5.89970521455116e+00)),
    (-1, (1.00000000000000e-10, 1.00000000000000e-10), (-2.59394857458299e+01, -2.45038025190116e+00)),
    (-1, (1.00000000000000e+05, 1.00000000000000e+05), (9.48654525241926e+00, -5.01175556022104e+00)),
    (-1, (-1.00000000000000e+10, -1.00000000000000e+10), (2.02859166581783e+01, -8.25299614835927e+00)),
    (2, (1.00000000000000e-01, 2.00000000000000e-01), (-4.01884636351973e+00, 1.17737851708391e+01)),
    (2, (-1.50000000000000e+00, -5.00000000000000e-01), (-1.63860506821632e+00, 7.97303621903506e+00)),
    (2, (1.00000000000000e+00, 1.00000000000000e+00), (-2.12088393794371e+00, 1.16001371107746e+01)),
    (2, (-2.00000000000000e+00, 2.00000000000000e+00), (-1.54996917078976e+00, 1.32351900868020e+01)),
    (2, (1.00000000000000e-03, 1.00000000000000e-03), (-9.23021941907345e+00, 1.10866971984750e+01)),
    (2, (-1.00000000000000e-05, -1.00000000000000e-05), (-1.39296392861204e+01, 7.56616813127560e+00)),
    (2, (1.00000000000000e-10, 1.00000000000000e-10), (-2.60146991386510e+01, 1.05969997046789e+01)),
    (2, (1.00000000000000e+05, 1.00000000000000e+05), (9.12465445887102e+00, 1.24147928250188e+01)),
    (2, (-1.00000000000000e+10, -1.00000000000000e+10), (2.02594278237148e+01, 9.76118646062654e+00)),
    (-2, (1.00000000000000e-01, 2.00000000000000e-01), (-3.82480330714685e+00, -9.50588496990850e+00)),
    (-2, (-1.50000000000000e+00, -5.00000000000000e-01), (-2.16862583645425e+00, -1.36579497138827e+01)),
    (-2, (1.00000000000000e+00, 1.00000000000000e+00), (-1.97664835818959e+00, -1.00153178860529e+01)),
    (-2, (-2.00000000000000e+00, 2.00000000000000e+00), (-1.10991825030977e+00, -8.50968191215266e+00)),
    (-2, (1.00000000000000e-03, 1.00000000000000e-03), (-9.13671388761849e+00, -9.44116428569044e+00)),
    (-2, (-1.00000000000000e-05, -1.00000000000000e-05), (-1.41028348339895e+01, -1.25064488644793e+01)),
    (-2, (1.00000000000000e-10, 1.00000000000000e-10), (-2.59933963064382e+01, -8.97173118397959e+00)),
    (-2, (1.00000000000000e+05, 1.00000000000000e+05), (9.20115941424720e+00, -1.09107732005270e+01)),
    (-2, (-1.00000000000000e+10, -1.00000000000000e+10), (2.01646617674720e+01, -1.43055343738672e+01)),
    (3, (1.00000000000000e-01, 2.00000000000000e-01), (-4.42523889933714e+00, 1.81467177938732e+01)),
    (3, (-1.50000000000000e+00, -5.00000000000000e-01), (-2.21432668372646e+00, 1.43053461278382e+01)),
    (3, (1.00000000000000e+00, 1.00000000000000e+00), (-2.54951966162968e+00, 1.79228561161299e+01)),
    (3, (-2.00000000000000e+00, 2.00000000000000e+00), (-1.93743706052764e+00, 1.95361051778842e+01)),
    (3, (1.00000000000000e-03, 1.00000000000000e-03), (-9.55677967995500e+00, 1.75658916862188e+01)),
    (3, (-1.00000000000000e-05, -1.00000000000000e-05), (-1.41625993439365e+01, 1.41362353118199e+01)),
    (3, (1.00000000000000e-10, 1.00000000000000e-10), (-2.61198331625982e+01, 1.70722726260608e+01)),
    (3, (1.00000000000000e+05, 1.00000000000000e+05), (8.83850770934163e+00, 1.85096504748857e+01)),
    (3, (-1.00000000000000e+10, -1.00000000000000e+10), (2.01295762452058e+01, 1.58270556370793e+01)),
    (-3, (1.00000000000000e-01, 2.00000000000000e-01), (-4.29992366797182e+00, -1.59076138444176e+01)),
    (-3, (-1.50000000000000e+00, -5.00000000000000e-01), (-2.54422963707569e+00, -1.99718936671959e+01)),
    (-3, (1.00000000000000e+00, 1.00000000000000e+00), (-2.45847800852231e+00, -1.63440604860586e+01)),
    (-3, (-2.00000000000000e+00, 2.00000000000000e+00), (-1.66189354567321e+00, -1.48108244650887e+01)),
    (-3, (1.00000000000000e-03, 1.00000000000000e-03), (-9.48229539503134e+00, -1.59571929424543e+01)),
    (-3, (-1.00000000000000e-05, -1.00000000000000e-05), (-1.43357082147051e+01, -1.89882753981914e+01)),
    (-3, (1.00000000000000e-10, 1.00000000000000e-10), (-2.60912940122029e+01, -1.54574037625587e+01)),
    (-3, (1.00000000000000e+05, 1.00000000000000e+05), (8.90610946505189e+00, -1.69765056463555e+01)),
    (-3, (-1.00000000000000e+10, -1.00000000000000e+10), (2.00193779050643e+01, -2.04106741980788e+01)),
    (4, (1.00000000000000e-01, 2.00000000000000e-01), (-4.71388172948040e+00, 2.44788528228533e+01)),
    (4, (-1.50000000000000e+00, -5.00000000000000e-01), (-2.57575359510472e+00, 2.06178181851174e+01)),
    (4, (1.00000000000000e+00, 1.00000000000000e+00), (-2.84789207281856e+00, 2.42303457310143e+01)),
    (4, (-2.00000000000000e+00, 2.00000000000000e+00), (-2.21558030616414e+00, 2.58325818742762e+01)),
    (4, (1.00000000000000e-03, 1.00000000000000e-03), (-9.81507934629349e+00, 2.39585287750855e+01)),
    (4, (-1.00000000000000e-05, -1.00000000000000e-05), (-1.43902183599606e+01, 2.05959019773948e+01)),
    (4, (1.00000000000000e-10, 1.00000000000000e-10), (-2.62411865814302e+01, 2.35070404688606e+01)),
    (4, (1.00000000000000e+05, 1.00000000000000e+05), (8.59616476558792e+00, 2.46824758122289e+01)),
    (4, (-1.00000000000000e+10, -1.00000000000000e+10), (1.99819890342455e+01, 2.19443771253420e+01)),
    (-4, (1.00000000000000e-01, 2.00000000000000e-01), (-4.62132585039345e+00, -2.22500078939870e+01)),
    (-4, (-1.50000000000000e+00, -5.00000000000000e-01), (-2.81618421601037e+00, -2.62750135438041e+01)),
    (-4, (1.00000000000000e+00, 1.00000000000000e+00), (-2.78125991468724e+00, -2.26543888795349e+01)),
    (-4, (-2.00000000000000e+00, 2.00000000000000e+00), (-2.01458775622481e+00, -2.11106084166325e+01)),
    (-4, (1.00000000000000e-03, 1.00000000000000e-03), (-9.75578040557180e+00, -2.23652253694710e+01)),
    (-4, (-1.00000000000000e-05, -1.00000000000000e-05), (-1.45428126857475e+01, -2.53981150344532e+01)),
    (-4, (1.00000000000000e-10, 1.00000000000000e-10), (-2.62102303194846e+01, -2.19018376910432e+01)),
    (-4, (1.00000000000000e+05, 1.00000000000000e+05), (8.65271017901480e+00, -2.31344564664377e+01)),
    (-4, (-1.00000000000000e+10, -1.00000000000000e+10), (1.98707535945452e+01, -2.65604447520228e+01)),
    (5, (1.00000000000000e-01, 2.00000000000000e-01), (-4.93785688911669e+00, 3.07932775174142e+01)),
    (5, (-1.50000000000000e+00, -5.00000000000000e-01), (-2.84026556473957e+00, 2.69201700373995e+01)),
    (5, (1.00000000000000e+00, 1.00000000000000e+00), (-3.07719259060358e+00, 3.05300754538032e+01)),
    (5, (-2.00000000000000e+00, 2.00000000000000e+00), (-2.43279598099709e+00, 3.21257416496661e+01)),
    (5, (1.00000000000000e-03, 1.00000000000000e-03), (-1.00245963198507e+01, 3.03111286900973e+01)),
    (5, (-1.00000000000000e-05, -1.00000000000000e-05), (-1.45900971018289e+01, 2.69934108746896e+01)),
    (5, (1.00000000000000e-10, 1.00000000000000e-10), (-2.63648951712406e+01, 2.99080102778645e+01)),
    (5, (1.00000000000000e+05, 1.00000000000000e+05), (8.39327658905961e+00, 3.08957905697996e+01)),
    (5, (-1.00000000000000e+10, -1.00000000000000e+10), (1.98344303181810e+01, 2.81035191561468e+01)),
    (-5, (1.00000000000000e-01, 2.00000000000000e-01), (-4.86448959702566e+00, -2.85693292732093e+01)),
    (-5, (-1.50000000000000e+00, -5.00000000000000e-01), (-3.02962166969726e+00, -3.25722267298523e+01)),
    (-5, (1.00000000000000e+00, 1.00000000000000e+00), (-3.02461792185673e+00, -2.89556525885945e+01)),
    (-5, (-2.00000000000000e+00, 2.00000000000000e+00), (-2.27447807732827e+00, -2.74061339030597e+01)),
    (-5, (1.00000000000000e-03, 1.00000000000000e-03), (-9.97590108568133e+00, -2.87254788124116e+01)),
    (-5, (-1.00000000000000e-05, -1.00000000000000e-05), (-1.47220688555235e+01, -3.17673554854086e+01)),
    (-5, (1.00000000000000e-10, 1.00000000000000e-10), (-2.63342094174687e+01, -2.83104840283573e+01)),
    (-5, (1.00000000000000e+05, 1.00000000000000e+05), (8.44079282651413e+00, -2.93398576426818e+01)),
    (-5, (-1.00000000000000e+10, -1.00000000000000e+10), (1.97288593007463e+01, -3.27435992458054e+01)),
    (10, (1.00000000000000e-01, 2.00000000000000e-01), (-5.63354894170227e+00, 6.22779928874857e+01)),
    (10, (-1.50000000000000e+00, -5.00000000000000e-01), (-3.61072749491542e+00, 5.83794440343904e+01)),
    (10, (1.00000000000000e+00, 1.00000000000000e+00), (-3.78218520497303e+00, 6.19855132198778e+01)),
    (10, (-2.00000000000000e+00, 2.00000000000000e+00), (-3.11359240264734e+00, 6.35683100871303e+01)),
    (10, (1.00000000000000e-03, 1.00000000000000e-03), (-1.07010369748846e+01, 6.18752034014718e+01)),
    (10, (-1.00000000000000e-05, -1.00000000000000e-05), (-1.52707387003207e+01, 5.86501476596711e+01)),
    (10, (1.00000000000000e-10, 1.00000000000000e-10), (-2.68876133418300e+01, 6.16351036786085e+01)),
    (10, (1.00000000000000e+05, 1.00000000000000e+05), (7.72197118589596e+00, 6.21700293176347e+01)),
    (10, (-1.00000000000000e+10, -1.00000000000000e+10), (1.92410021350064e+01, 5.92190150706817e+01)),
    (-10, (1.00000000000000e-01, 2.00000000000000e-01), (-5.59755065598300e+00, -6.00609786700142e+01)),
    (-10, (-1.50000000000000e+00, -5.00000000000000e-01), (-3.70276867713420e+00, -6.40231283382707e+01)),
    (-10, (1.00000000000000e+00, 1.00000000000000e+00), (-3.75656945808514e+00, -6.04135576350880e+01)),
    (-10, (-2.00000000000000e+00, 2.00000000000000e+00), (-3.03665668030775e+00, -5.88533109292444e+01)),
    (-10, (1.00000000000000e-03, 1.00000000000000e-03), (-1.06759529319823e+01, -6.03004283510755e+01)),
    (-10, (-1.00000000000000e-05, -1.00000000000000e-05), (-1.53439744706286e+01, -6.33797254062099e+01)),
    (-10, (1.00000000000000e-10, 1.00000000000000e-10), (-2.68657507961297e+01, -6.00550086017816e+01)),
    (-10, (1.00000000000000e+05, 1.00000000000000e+05), (7.74705329252584e+00, -6.06028022599792e+01)),
    (-10, (-1.00000000000000e+10, -1.00000000000000e+10), (1.91718833334960e+01, -6.39086976083923e+01)),
    (20, (1.00000000000000e-01, 2.00000000000000e-01), (-6.32865238788265e+00, 1.25149532847671e+02)),
    (20, (-1.50000000000000e+00, -5.00000000000000e-01), (-4.34024467424233e+00, 1.21237283412570e+02)),
    (20, (1.00000000000000e+00, 1.00000000000000e+00), (-4.48112256824107e+00, 1.24842429155994e+02)),
    (20, (-2.00000000000000e+00, 2.00000000000000e+00), (-3.80033306676223e+00, 1.26419051962370e+02)),
    (20, (1.00000000000000e-03, 1.00000000000000e-03), (-1.13919418775437e+01, 1.24787269428583e+02)),
    (20, (-1.00000000000000e-05, -1.00000000000000e-05), (-1.59756946776269e+01, 1.21606090882015e+02)),
    (20, (1.00000000000000e-10, 1.00000000000000e-10), (-2.75286818607044e+01, 1.24660967657583e+02)),
    (20, (1.00000000000000e+05, 1.00000000000000e+05), (7.03012861781248e+00, 1.24934519207735e+02)),
    (20, (-1.00000000000000e+10, -1.00000000000000e+10), (1.85578652123765e+01, 1.21887808612600e+02)),
    (-20, (1.00000000000000e-01, 2.00000000000000e-01), (-6.31083350040121e+00, -1.22934471183385e+02)),
    (-20, (-1.50000000000000e+00, -5.00000000000000e-01), (-4.38567925176181e+00, -1.26878199614427e+02)),
    (-20, (1.00000000000000e+00, 1.00000000000000e+00), (-4.46847041889690e+00, -1.23271278438061e+02)),
    (-20, (-2.00000000000000e+00, 2.00000000000000e+00), (-3.76236357818421e+00, -1.21705811578335e+02)),
    (-20, (1.00000000000000e-03, 1.00000000000000e-03), (-1.13793623811437e+01, -1.23215419487558e+02)),
    (-20, (-1.00000000000000e-05, -1.00000000000000e-05), (-1.60131647378883e+01, -1.26323013185747e+02)),
    (-20, (1.00000000000000e-10, 1.00000000000000e-10), (-2.75165569932784e+01, -1.23087575219802e+02)),
    (-20, (1.00000000000000e+05, 1.00000000000000e+05), (7.04272844180896e+00, -1.23364538513839e+02)),
    (-20, (-1.00000000000000e+10, -1.00000000000000e+10), (1.85208474210487e+01, -1.26594374422785e+02)),
    (50, (1.00000000000000e-01, 2.00000000000000e-01), (-7.24648242934708e+00, 3.13672519795192e+02)),
    (50, (-1.50000000000000e+00, -5.00000000000000e-01), (-5.27777042519842e+00, 3.09751589862180e+02)),
    (50, (1.00000000000000e+00, 1.00000000000000e+00), (-5.40091686727165e+00, 3.13356633214950e+02)),
    (50, (-2.00000000000000e+00, 2.00000000000000e+00), (-4.71274062358173e+00, 3.14929700218227e+02)),
    (50, (1.00000000000000e-03, 1.00000000000000e-03), (-1.23092243717978e+01, 3.13334602786458e+02)),
    (50, (-1.00000000000000e-05, -1.00000000000000e-05), (-1.69049806191180e+01, 3.10177827476539e+02)),
    (50, (1.00000000000000e-10, 1.00000000000000e-10), (-2.84304863639158e+01, 3.13283365031084e+02)),
    (50, (1.00000000000000e+05, 1.00000000000000e+05), (6.11184975944375e+00, 3.13393366890638e+02)),
    (50, (-1.00000000000000e+10, -1.00000000000000e+10), (1.76333081171052e+01, 3.10289042136690e+02)),
    (-50, (1.00000000000000e-01, 2.00000000000000e-01), (-7.23940095332919e+00, -3.11458080918498e+02)),
    (-50, (-1.50000000000000e+00, -5.00000000000000e-01), (-5.29581037920822e+00, -3.15391521482978e+02)),
    (-50, (1.00000000000000e+00, 1.00000000000000e+00), (-5.39589244833863e+00, -3.11785766185831e+02)),
    (-50, (-2.00000000000000e+00, 2.00000000000000e+00), (-4.69766635345302e+00, -3.10217132543604e+02)),
    (-50, (1.00000000000000e-03, 1.00000000000000e-03), (-1.23042051714777e+01, -3.11763624885338e+02)),
    (-50, (-1.00000000000000e-05, -1.00000000000000e-05), (-1.69200199000431e+01, -3.14890982200860e+02)),
    (-50, (1.00000000000000e-10, 1.00000000000000e-10), (-2.84254981961486e+01, -3.11712130889970e+02)),
    (-50, (1.00000000000000e+05, 1.00000000000000e+05), (6.11687198007925e+00, -3.11822684860496e+02)),
    (-50, (-1.00000000000000e+10, -1.00000000000000e+10), (1.76182884831039e+01, -3.15000536288444e+02)),
];

#[test]
fn test_lambert_w0() {
    assert!(lambert_w0(-1.0).is_nan());
    assert!(sp_lambert_w0(-1.0).is_nan());
    assert_abs_diff_eq!(lambert_w0(BRANCH_POINT.0), BRANCH_POINT.1);
    assert_abs_diff_eq!(
        sp_lambert_w0(BRANCH_POINT.0),
        BRANCH_POINT.1,
        epsilon = 1e-7
    );
    for (x, y) in LAMBERT_W0_TABLE {
        assert_relative_eq!(lambert_w0(x), y, max_relative = 1e-14);
        assert_relative_eq!(sp_lambert_w0(x), y, max_relative = 1e-7);
    }
    assert_eq!(lambert_w0(f64::INFINITY), f64::INFINITY);
    assert_eq!(sp_lambert_w0(f64::INFINITY), f64::INFINITY);
    assert!(lambert_w0(f64::NAN).is_nan());
    assert!(sp_lambert_w0(f64::NAN).is_nan());
}

#[test]
fn test_lambert_wm1() {
    assert!(lambert_wm1(-1.0).is_nan());
    assert!(sp_lambert_wm1(-1.0).is_nan());
    assert_abs_diff_eq!(lambert_wm1(BRANCH_POINT.0), BRANCH_POINT.1);
    assert_abs_diff_eq!(
        sp_lambert_wm1(BRANCH_POINT.0),
        BRANCH_POINT.1,
        epsilon = 1e-7
    );
    for (x, y) in LAMBERT_WM1_TABLE {
        assert_relative_eq!(lambert_wm1(x), y, max_relative = 1e-14);
        assert_relative_eq!(sp_lambert_wm1(x), y, max_relative = 1e-7);
    }
    assert!(lambert_wm1(f64::NAN).is_nan());
    assert!(sp_lambert_wm1(f64::NAN).is_nan());
}

#[test]
fn test_lambert_w() {
    for k in -10..10 {
        let w = lambert_w(k, f64::NAN, 0.0);
        assert!(w.0.is_nan() && w.1.is_nan());
        let w = lambert_w(k, 0.0, f64::NAN);
        assert!(w.0.is_nan() && w.1.is_nan());
        let w = lambert_w(k, f64::NAN, f64::NAN);
        assert!(w.0.is_nan() && w.1.is_nan());

        let w = lambert_w(k, f64::INFINITY, 0.0);
        assert!(w.0.is_nan() && w.1.is_nan());
        let w = lambert_w(k, 0.0, f64::INFINITY);
        assert!(w.0.is_nan() && w.1.is_nan());
        let w = lambert_w(k, f64::INFINITY, f64::INFINITY);
        assert!(w.0.is_nan() && w.1.is_nan());
    }

    for (k, (z_re, z_im), (w_re, w_im)) in COMPLEX_LAMBERT_W_TABLE {
        let w = lambert_w(k, z_re, z_im);
        assert_relative_eq!(w.0, w_re, max_relative = 1e-10);
        assert_relative_eq!(w.1, w_im, max_relative = 1e-10);
    }
}
