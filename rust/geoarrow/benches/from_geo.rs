use geo::polygon;

use criterion::{Criterion, criterion_group, criterion_main};
use geoarrow::array::{PolygonArray, PolygonBuilder};
use geoarrow_schema::{CoordType, Dimension};

fn create_data() -> Vec<geo::Polygon> {
    // An L shape
    // https://github.com/georust/geo/blob/7cb7d0ffa6bf1544c5ca9922bd06100c36f815d7/README.md?plain=1#L40
    let poly = polygon![
        (x: 0.0, y: 0.0),
        (x: 4.0, y: 0.0),
        (x: 4.0, y: 1.0),
        (x: 1.0, y: 1.0),
        (x: 1.0, y: 4.0),
        (x: 0.0, y: 4.0),
        (x: 0.0, y: 0.0),
    ];
    let v = vec![poly; 1000];
    v
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let data = create_data();

    c.bench_function("convert Vec<geo::Polygon> to PolygonArray", |b| {
        b.iter(|| {
            let mut_arr = PolygonBuilder::from_polygons(
                &data,
                Dimension::XY,
                CoordType::default_interleaved(),
                Default::default(),
            );
            let _arr: PolygonArray = mut_arr.into();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
