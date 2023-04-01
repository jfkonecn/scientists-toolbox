pub struct IjnRegionPoint {
    pub i: f64,
    pub j: f64,
    pub n: f64,
}

pub struct JnRegionPoint {
    pub j: f64,
    pub n: f64,
}

pub struct NRegionPoint {
    pub n: f64,
}

pub static REGION_1_AND_4: &[IjnRegionPoint] = &[
    IjnRegionPoint {
        i: 0.0,
        j: -2.0,
        n: 1.4632971213167E-01,
    },
    IjnRegionPoint {
        i: 0.0,
        j: -1.0,
        n: -8.4548187169114E-01,
    },
    IjnRegionPoint {
        i: 0.0,
        j: 0.0,
        n: -3.756360367204,
    },
    IjnRegionPoint {
        i: 0.0,
        j: 1.0,
        n: 3.3855169168385E+00,
    },
    IjnRegionPoint {
        i: 0.0,
        j: 2.0,
        n: -9.5791963387872E-01,
    },
    IjnRegionPoint {
        i: 0.0,
        j: 3.0,
        n: 1.5772038513228E-01,
    },
    IjnRegionPoint {
        i: 0.0,
        j: 4.0,
        n: -1.6616417199501E-02,
    },
    IjnRegionPoint {
        i: 0.0,
        j: 5.0,
        n: 8.1214629983568E-04,
    },
    IjnRegionPoint {
        i: 1.0,
        j: -9.0,
        n: 2.8319080123804E-04,
    },
    IjnRegionPoint {
        i: 1.0,
        j: -7.0,
        n: -6.0706301565874E-04,
    },
    IjnRegionPoint {
        i: 1.0,
        j: -1.0,
        n: -1.8990068218419E-02,
    },
    IjnRegionPoint {
        i: 1.0,
        j: 0.0,
        n: -3.2529748770505E-02,
    },
    IjnRegionPoint {
        i: 1.0,
        j: 1.0,
        n: -2.1841717175414E-02,
    },
    IjnRegionPoint {
        i: 1.0,
        j: 3.0,
        n: -5.2838357969930E-05,
    },
    IjnRegionPoint {
        i: 2.0,
        j: -3.0,
        n: -4.7184321073267E-04,
    },
    IjnRegionPoint {
        i: 2.0,
        j: 0.0,
        n: -3.0001780793026E-04,
    },
    IjnRegionPoint {
        i: 2.0,
        j: 1.0,
        n: 4.7661393906987E-05,
    },
    IjnRegionPoint {
        i: 2.0,
        j: 3.0,
        n: -4.4141845330846E-06,
    },
    IjnRegionPoint {
        i: 2.0,
        j: 17.0,
        n: -7.2694996297594E-16,
    },
    IjnRegionPoint {
        i: 3.0,
        j: -4.0,
        n: -3.1679644845054E-05,
    },
    IjnRegionPoint {
        i: 3.0,
        j: 0.0,
        n: -2.8270797985312E-06,
    },
    IjnRegionPoint {
        i: 3.0,
        j: 6.0,
        n: -8.5205128120103E-10,
    },
    IjnRegionPoint {
        i: 4.0,
        j: -5.0,
        n: -2.2425281908000E-06,
    },
    IjnRegionPoint {
        i: 4.0,
        j: -2.0,
        n: -6.5171222895601E-07,
    },
    IjnRegionPoint {
        i: 4.0,
        j: 10.0,
        n: -1.4341729937924E-13,
    },
    IjnRegionPoint {
        i: 5.0,
        j: -8.0,
        n: -4.0516996860117E-07,
    },
    IjnRegionPoint {
        i: 8.0,
        j: -11.0,
        n: -1.2734301741641E-09,
    },
    IjnRegionPoint {
        i: 8.0,
        j: -6.0,
        n: -1.7424871230634E-10,
    },
    IjnRegionPoint {
        i: 21.0,
        j: -29.0,
        n: -6.8762131295531E-19,
    },
    IjnRegionPoint {
        i: 23.0,
        j: -31.0,
        n: 1.4478307828521E-20,
    },
    IjnRegionPoint {
        i: 29.0,
        j: -38.0,
        n: 2.6335781662795E-23,
    },
    IjnRegionPoint {
        i: 30.0,
        j: -39.0,
        n: -1.1947622640071E-23,
    },
    IjnRegionPoint {
        i: 31.0,
        j: -40.0,
        n: 1.8228094581404E-24,
    },
    IjnRegionPoint {
        i: 32.0,
        j: -41.0,
        n: -9.3537087292458E-26,
    },
];

pub static REGION_2_IDEAL: &[JnRegionPoint] = &[
    JnRegionPoint {
        j: 0.0,
        n: -9.6927686500217E+00,
    },
    JnRegionPoint {
        j: 1.0,
        n: 1.0086655968018E+01,
    },
    JnRegionPoint {
        j: -5.0,
        n: -5.6087911283020E-03,
    },
    JnRegionPoint {
        j: -4.0,
        n: 7.1452738081455E-02,
    },
    JnRegionPoint {
        j: -3.0,
        n: -4.0710498223928E-01,
    },
    JnRegionPoint {
        j: -2.0,
        n: 1.4240819171444E+00,
    },
    JnRegionPoint {
        j: -1.0,
        n: -4.3839511319450E+00,
    },
    JnRegionPoint {
        j: 2.0,
        n: -2.8408632460772E-01,
    },
    JnRegionPoint {
        j: 3.0,
        n: 2.1268463753307E-02,
    },
];

pub static REGION_2_RESIDUAL: &[IjnRegionPoint] = &[
    IjnRegionPoint {
        i: 1.0,
        j: 0.0,
        n: -1.7731742473213E-03,
    },
    IjnRegionPoint {
        i: 1.0,
        j: 1.0,
        n: -1.7834862292358E-02,
    },
    IjnRegionPoint {
        i: 1.0,
        j: 2.0,
        n: -4.5996013696365E-02,
    },
    IjnRegionPoint {
        i: 1.0,
        j: 3.0,
        n: -5.7581259083432E-02,
    },
    IjnRegionPoint {
        i: 1.0,
        j: 6.0,
        n: -5.0325278727930E-02,
    },
    IjnRegionPoint {
        i: 2.0,
        j: 1.0,
        n: -3.3032641670203E-05,
    },
    IjnRegionPoint {
        i: 2.0,
        j: 2.0,
        n: -1.8948987516315E-04,
    },
    IjnRegionPoint {
        i: 2.0,
        j: 4.0,
        n: -3.9392777243355E-03,
    },
    IjnRegionPoint {
        i: 2.0,
        j: 7.0,
        n: -4.3797295650573E-02,
    },
    IjnRegionPoint {
        i: 2.0,
        j: 36.0,
        n: -2.6674547914087E-05,
    },
    IjnRegionPoint {
        i: 3.0,
        j: 0.0,
        n: 2.0481737692309E-08,
    },
    IjnRegionPoint {
        i: 3.0,
        j: 1.0,
        n: 4.3870667284435E-07,
    },
    IjnRegionPoint {
        i: 3.0,
        j: 3.0,
        n: -3.2277677238570E-05,
    },
    IjnRegionPoint {
        i: 3.0,
        j: 6.0,
        n: -1.5033924542148E-03,
    },
    IjnRegionPoint {
        i: 3.0,
        j: 35.0,
        n: -4.0668253562649E-02,
    },
    IjnRegionPoint {
        i: 4.0,
        j: 1.0,
        n: -7.8847309559367E-10,
    },
    IjnRegionPoint {
        i: 4.0,
        j: 2.0,
        n: 1.2790717852285E-08,
    },
    IjnRegionPoint {
        i: 4.0,
        j: 3.0,
        n: 4.8225372718507E-07,
    },
    IjnRegionPoint {
        i: 5.0,
        j: 7.0,
        n: 2.2922076337661E-06,
    },
    IjnRegionPoint {
        i: 6.0,
        j: 3.0,
        n: -1.6714766451061E-11,
    },
    IjnRegionPoint {
        i: 6.0,
        j: 16.0,
        n: -2.1171472321355E-03,
    },
    IjnRegionPoint {
        i: 6.0,
        j: 35.0,
        n: -2.3895741934104E+01,
    },
    IjnRegionPoint {
        i: 7.0,
        j: 0.0,
        n: -5.9059564324270E-18,
    },
    IjnRegionPoint {
        i: 7.0,
        j: 11.0,
        n: -1.2621808899101E-06,
    },
    IjnRegionPoint {
        i: 7.0,
        j: 25.0,
        n: -3.8946842435739E-02,
    },
    IjnRegionPoint {
        i: 8.0,
        j: 8.0,
        n: 1.1256211360459E-11,
    },
    IjnRegionPoint {
        i: 8.0,
        j: 36.0,
        n: -8.2311340897998E+00,
    },
    IjnRegionPoint {
        i: 9.0,
        j: 13.0,
        n: 1.9809712802088E-08,
    },
    IjnRegionPoint {
        i: 10.0,
        j: 4.0,
        n: 1.0406965210174E-19,
    },
    IjnRegionPoint {
        i: 10.0,
        j: 10.0,
        n: -1.0234747095929E-13,
    },
    IjnRegionPoint {
        i: 10.0,
        j: 14.0,
        n: -1.0018179379511E-09,
    },
    IjnRegionPoint {
        i: 16.0,
        j: 29.0,
        n: -8.0882908646985E-11,
    },
    IjnRegionPoint {
        i: 16.0,
        j: 50.0,
        n: 1.0693031879409E-01,
    },
    IjnRegionPoint {
        i: 18.0,
        j: 57.0,
        n: -3.3662250574171E-01,
    },
    IjnRegionPoint {
        i: 20.0,
        j: 20.0,
        n: 8.9185845355421E-25,
    },
    IjnRegionPoint {
        i: 20.0,
        j: 35.0,
        n: 3.0629316876232E-13,
    },
    IjnRegionPoint {
        i: 20.0,
        j: 48.0,
        n: -4.2002467698208E-06,
    },
    IjnRegionPoint {
        i: 21.0,
        j: 21.0,
        n: -5.9056029685639E-26,
    },
    IjnRegionPoint {
        i: 22.0,
        j: 53.0,
        n: 3.7826947613457E-06,
    },
    IjnRegionPoint {
        i: 23.0,
        j: 39.0,
        n: -1.2768608934681E-15,
    },
    IjnRegionPoint {
        i: 24.0,
        j: 26.0,
        n: 7.3087610595061E-29,
    },
    IjnRegionPoint {
        i: 24.0,
        j: 40.0,
        n: 5.5414715350778E-17,
    },
    IjnRegionPoint {
        i: 24.0,
        j: 58.0,
        n: -9.4369707241210E-07,
    },
];

pub static REGION_3_N1: &NRegionPoint = &NRegionPoint {
    n: 1.0658070028513E+00,
};

pub static REGION_3: &[IjnRegionPoint] = &[
    IjnRegionPoint {
        i: 0.0,
        j: 0.0,
        n: -1.5732845290239E+01,
    },
    IjnRegionPoint {
        i: 0.0,
        j: 1.0,
        n: 2.0944396974307E+01,
    },
    IjnRegionPoint {
        i: 0.0,
        j: 2.0,
        n: -7.6867707878716E+00,
    },
    IjnRegionPoint {
        i: 0.0,
        j: 7.0,
        n: 2.6185947787954E+00,
    },
    IjnRegionPoint {
        i: 0.0,
        j: 10.0,
        n: -2.8080781148620E+00,
    },
    IjnRegionPoint {
        i: 0.0,
        j: 12.0,
        n: 1.2053369696517E+00,
    },
    IjnRegionPoint {
        i: 0.0,
        j: 23.0,
        n: -8.4566812812502E-03,
    },
    IjnRegionPoint {
        i: 1.0,
        j: 2.0,
        n: -1.2654315477714E+00,
    },
    IjnRegionPoint {
        i: 1.0,
        j: 6.0,
        n: -1.1524407806681E+00,
    },
    IjnRegionPoint {
        i: 1.0,
        j: 15.0,
        n: 8.8521043984318E-01,
    },
    IjnRegionPoint {
        i: 1.0,
        j: 17.0,
        n: -6.4207765181607E-01,
    },
    IjnRegionPoint {
        i: 2.0,
        j: 0.0,
        n: 3.8493460186671E-01,
    },
    IjnRegionPoint {
        i: 2.0,
        j: 2.0,
        n: -8.5214708824206E-01,
    },
    IjnRegionPoint {
        i: 2.0,
        j: 6.0,
        n: 4.8972281541877E+00,
    },
    IjnRegionPoint {
        i: 2.0,
        j: 7.0,
        n: -3.0502617256965E+00,
    },
    IjnRegionPoint {
        i: 2.0,
        j: 22.0,
        n: 3.9420536879154E-02,
    },
    IjnRegionPoint {
        i: 2.0,
        j: 26.0,
        n: 1.2558408424308E-01,
    },
    IjnRegionPoint {
        i: 3.0,
        j: 0.0,
        n: -2.7999329698710E-01,
    },
    IjnRegionPoint {
        i: 3.0,
        j: 2.0,
        n: 1.3899799569460E+00,
    },
    IjnRegionPoint {
        i: 3.0,
        j: 4.0,
        n: -2.0189915023570E+00,
    },
    IjnRegionPoint {
        i: 3.0,
        j: 16.0,
        n: -8.2147637173963E-03,
    },
    IjnRegionPoint {
        i: 3.0,
        j: 26.0,
        n: -4.7596035734923E-01,
    },
    IjnRegionPoint {
        i: 4.0,
        j: 0.0,
        n: 4.3984074473500E-02,
    },
    IjnRegionPoint {
        i: 4.0,
        j: 2.0,
        n: -4.4476435428739E-01,
    },
    IjnRegionPoint {
        i: 4.0,
        j: 4.0,
        n: 9.0572070719733E-01,
    },
    IjnRegionPoint {
        i: 4.0,
        j: 26.0,
        n: 7.0522450087967E-01,
    },
    IjnRegionPoint {
        i: 5.0,
        j: 1.0,
        n: 1.0770512626332E-01,
    },
    IjnRegionPoint {
        i: 5.0,
        j: 3.0,
        n: -3.2913623258954E-01,
    },
    IjnRegionPoint {
        i: 5.0,
        j: 26.0,
        n: -5.0871062041158E-01,
    },
    IjnRegionPoint {
        i: 6.0,
        j: 0.0,
        n: -2.2175400873096E-02,
    },
    IjnRegionPoint {
        i: 6.0,
        j: 2.0,
        n: 9.4260751665092E-02,
    },
    IjnRegionPoint {
        i: 6.0,
        j: 26.0,
        n: 1.6436278447961E-01,
    },
    IjnRegionPoint {
        i: 7.0,
        j: 2.0,
        n: -1.3503372241348E-02,
    },
    IjnRegionPoint {
        i: 8.0,
        j: 26.0,
        n: -1.4834345352472E-02,
    },
    IjnRegionPoint {
        i: 9.0,
        j: 2.0,
        n: 5.7922953628084E-04,
    },
    IjnRegionPoint {
        i: 9.0,
        j: 26.0,
        n: 3.2308904703711E-03,
    },
    IjnRegionPoint {
        i: 10.0,
        j: 0.0,
        n: 8.0964802996215E-05,
    },
    IjnRegionPoint {
        i: 10.0,
        j: 1.0,
        n: -1.6557679795037E-04,
    },
    IjnRegionPoint {
        i: 11.0,
        j: 26.0,
        n: -4.4923899061815E-05,
    },
];

pub static REGION_5_IDEAL: &[JnRegionPoint] = &[
    JnRegionPoint {
        j: 0.0,
        n: -1.3179983674201E+01,
    },
    JnRegionPoint {
        j: 1.0,
        n: 6.8540841634434E+00,
    },
    JnRegionPoint {
        j: -3.0,
        n: -2.4805148933466E-02,
    },
    JnRegionPoint {
        j: -2.0,
        n: 3.6901534980333E-01,
    },
    JnRegionPoint {
        j: -1.0,
        n: -3.1161318213925E+00,
    },
    JnRegionPoint {
        j: 2.0,
        n: -3.2961626538917E-01,
    },
];

pub static REGION_5_RESIDUAL: &[IjnRegionPoint] = &[
    IjnRegionPoint {
        i: 1.0,
        j: 1.0,
        n: 1.5736404855259E-03,
    },
    IjnRegionPoint {
        i: 1.0,
        j: 2.0,
        n: 9.0153761673944E-04,
    },
    IjnRegionPoint {
        i: 1.0,
        j: 3.0,
        n: -5.0270077677648E-03,
    },
    IjnRegionPoint {
        i: 2.0,
        j: 3.0,
        n: 2.2440037409485E-06,
    },
    IjnRegionPoint {
        i: 2.0,
        j: 9.0,
        n: -4.1163275453471E-06,
    },
    IjnRegionPoint {
        i: 3.0,
        j: 7.0,
        n: 3.7919454822955E-08,
    },
];

pub static BOUNDARY_34: &[NRegionPoint] = &[
    NRegionPoint { n: 348.05185628969 },
    NRegionPoint {
        n: -1.1671859879975,
    },
    NRegionPoint {
        n: 0.0010192970039326,
    },
    NRegionPoint { n: 572.54459862746 },
    NRegionPoint { n: 13.91883977887 },
];

pub static REGION_4: &[NRegionPoint] = &[
    NRegionPoint { n: 1167.0521452767 },
    NRegionPoint {
        n: -724213.16703206,
    },
    NRegionPoint {
        n: -17.073846940092,
    },
    NRegionPoint { n: 12020.82470247 },
    NRegionPoint {
        n: -3232555.0322333,
    },
    NRegionPoint { n: 14.91510861353 },
    NRegionPoint {
        n: -4823.2657361591,
    },
    NRegionPoint { n: 405113.40542057 },
    NRegionPoint {
        n: -0.23855557567849,
    },
    NRegionPoint { n: 650.17534844798 },
];
