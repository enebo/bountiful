use bountiful::resources::{Point, Map, Tile};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 20;

fn make_map(start: Point, end: Point) -> Map {
    let mut map = Map::new(MAP_WIDTH, MAP_HEIGHT, '.', 1);
    let mut rng = rand::thread_rng();

    // Add random walls
    let n_walls = 200;
    for _ in 0..n_walls {
        let target = Point::new(
            rng.gen_range(0, MAP_WIDTH as usize - 1),
            rng.gen_range(0, MAP_HEIGHT as usize - 1),
        );
        if target != start && target != end {
            map.set_at(&target, Tile::new('#', 1));
        }
    }

    map
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let (start, end) = (Point::new(1, MAP_HEIGHT - 1), Point::new(MAP_WIDTH - 3, MAP_HEIGHT - 1));

    c.bench_function("a_star_test_map", |b| {
        b.iter(|| {
            let map = make_map(start, end);
            if let Some(path) = map.shortest_path(&start, &end) {
                black_box(path);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);