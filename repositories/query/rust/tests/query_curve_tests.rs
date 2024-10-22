use query_curve::{get_encoded_curve_query_function, query_curve, query_encoded_curve};

#[test]
fn test_query_curve_values() {
    let curve = vec![
        1.0, 1.0, 0.0, 0.0, // scale_x, scale_y, offset_x, offset_y
        0.0, 0.0, // Start point x, y
        0.5, 0.0, // Start handle x, y
        0.5, 1.0, // End handle x, y
        1.0, 1.0, // End point x, y
    ];

    assert_eq!(query_curve(&curve, 0.0), Some(0.0));

    let y_at_0_3 = query_curve(&curve, 0.3).unwrap();
    assert!((y_at_0_3 - 0.16).abs() < 0.01);

    assert_eq!(query_curve(&curve, 0.5), Some(0.5));

    let y_at_0_6 = query_curve(&curve, 0.6).unwrap();
    assert!((y_at_0_6 - 0.69).abs() < 0.01);

    let y_at_0_8 = query_curve(&curve, 0.8).unwrap();
    assert!((y_at_0_8 - 0.94).abs() < 0.01);

    assert_eq!(query_curve(&curve, 1.0), Some(1.0));
}

#[test]
fn test_query_encoded_curve_values() {
    let encoded_scaled_chain = "fxSK-fxSK-0-0-0-0-KyjA-0-KyjA-fxSK-fxSK-fxSK";
    assert_eq!(
        query_encoded_curve(&encoded_scaled_chain.to_string(), 0.0),
        Some(0.0)
    );

    let y_at_0_3 = query_encoded_curve(&encoded_scaled_chain.to_string(), 0.3).unwrap();
    assert!((y_at_0_3 - 0.16).abs() < 0.01);

    assert_eq!(
        query_encoded_curve(&encoded_scaled_chain.to_string(), 0.5),
        Some(0.5)
    );

    let y_at_0_6 = query_encoded_curve(&encoded_scaled_chain.to_string(), 0.6).unwrap();
    assert!((y_at_0_6 - 0.69).abs() < 0.01);

    let y_at_0_8 = query_encoded_curve(&encoded_scaled_chain.to_string(), 0.8).unwrap();
    assert!((y_at_0_8 - 0.94).abs() < 0.01);

    assert_eq!(
        query_encoded_curve(&encoded_scaled_chain.to_string(), 1.0),
        Some(1.0)
    );
}

#[test]
fn test_get_encoded_curve_query_function() {
    let encoded_scaled_chain = "fxSK-fxSK-0-0-0-0-KyjA-0-KyjA-fxSK-fxSK-fxSK";
    let encoded_chain_string = encoded_scaled_chain.to_string();
    let query_my_curve = get_encoded_curve_query_function(&encoded_chain_string).unwrap();

    assert_eq!(query_my_curve(0.0), Some(0.0));

    let y_at_0_3 = query_my_curve(0.3).unwrap();
    assert!((y_at_0_3 - 0.16).abs() < 0.01);

    assert_eq!(query_my_curve(0.5), Some(0.5));

    let y_at_0_6 = query_my_curve(0.6).unwrap();
    assert!((y_at_0_6 - 0.69).abs() < 0.01);

    let y_at_0_8 = query_my_curve(0.8).unwrap();
    assert!((y_at_0_8 - 0.94).abs() < 0.01);

    assert_eq!(query_my_curve(1.0), Some(1.0));

    // Testing the second encoded chain
    let encoded_scaled_chain2 = "fxSK-fxSK-0-0-0-0-264W-0-AQ1l-0-CW6H-0-KYiG-0-OgWT-fxSK-VkR1-fxSK-XqVX-fxSK-drNo-fxSK-fxSK-fxSK";
    let encoded_chain_string2 = encoded_scaled_chain2.to_string();
    let query_my_curve2 = get_encoded_curve_query_function(&encoded_chain_string2).unwrap();

    assert_eq!(query_my_curve2(0.0), Some(0.0));

    let y_at_0_3 = query_my_curve2(0.3).unwrap();
    assert!((y_at_0_3 - 0.0).abs() < 0.01);

    let y_at_0_5 = query_my_curve2(0.5).unwrap();
    assert!((y_at_0_5 - 0.37).abs() < 0.01);

    let y_at_0_7 = query_my_curve2(0.7).unwrap();
    assert!((y_at_0_7 - 0.96).abs() < 0.01);

    let y_at_0_8 = query_my_curve2(0.8).unwrap();
    assert!((y_at_0_8 - 1.0).abs() < 0.000001);
}

#[test]
fn test_query_negative_values() {
    let chain1 = "-fxSK--fxSK-0-0-0-0-fxSK-fxSK-0-0-fxSK-fxSK";
    let chain1_string = chain1.to_string();
    let query_my_curve1 = get_encoded_curve_query_function(&chain1_string).unwrap();

    assert_eq!(query_my_curve1(0.0), Some(0.0));
    assert_eq!(query_my_curve1(-1.0), Some(-1.0));
    assert_eq!(query_my_curve1(-0.5), Some(-0.5));
}

#[test]
fn test_query_with_offset_and_scale() {
    let chain1 = "1Luue-2hppI--21sMy-3NnHc-0-0-fxSK-fxSK-0-0-fxSK-fxSK";
    let chain1_string = chain1.to_string();
    let query_my_curve1 = get_encoded_curve_query_function(&chain1_string).unwrap();

    assert_eq!(query_my_curve1(-6.0), Some(20.0));
    assert_eq!(query_my_curve1(-4.0), Some(24.0));
    assert_eq!(query_my_curve1(-5.0), Some(22.0));
}
