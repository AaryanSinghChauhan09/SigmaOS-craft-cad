use sigmaos_craft_cad::{Assembly, Part, Dimension, DimensionType};

#[test]
fn cad_assembly_integration_aaa() {
    let mut a = Assembly::new("Main Assembly");
    a.add_part(Part { id: "p1".into(), name: "Bracket".into(), material: "Steel".into() }).unwrap();
    a.add_dimension(Dimension { id: "d1".into(), dim_type: DimensionType::Length, value: 100.0, unit: "mm".into() }).unwrap();
    assert_eq!(a.part_count(), 1);
}
