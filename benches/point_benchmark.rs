use criterion::{black_box, criterion_group, criterion_main, Criterion};
use programming_btc::{point::{self, point::Point}, finite_fields::campos_finitos::FieldElement};

fn operaciones_punto() {
    let prime = 223;
    let a = FieldElement::new(0, prime).unwrap();
    let b = FieldElement::new(7, prime).unwrap();
    let x1 = Some(FieldElement::new(192, prime).unwrap());
    let y1 = Some(FieldElement::new(105, prime).unwrap());
    let x2 = Some(FieldElement::new(17, prime).unwrap());
    let y2 = Some(FieldElement::new(56, prime).unwrap());

    let p1 = Point::new(x1, y1, a, b).unwrap();
    let p2 = Point::new(x2, y2, a, b).unwrap();

    let p3: Point = p1 + p2;

    let a = FieldElement::new(0, prime).unwrap();
    let b = FieldElement::new(7, prime).unwrap();
    let x1 = Some(FieldElement::new(47, prime).unwrap());
    let y1 = Some(FieldElement::new(71, prime).unwrap());
    /* let x2 = Some(FieldElement::new(116, prime).unwrap());
    let y2 = Some(FieldElement::new(55, prime).unwrap()); */
    let p1 = Point::new(x1, y1, a, b).unwrap();
    let p2 = 8 * p1;

}


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("operaciones puntos", |b| b.iter(|| operaciones_punto()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);