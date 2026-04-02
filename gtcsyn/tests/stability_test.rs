use gtcsyn::HemminkiCore;

#[test]
fn test_geometric_integrity() {
    let mut core = HemminkiCore::init();
    let data = vec![1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1];
    let result = core.execute_transduction(&data);
    assert!(result.is_ok());
}

