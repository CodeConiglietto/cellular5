use serde::Serialize;

#[derive(Serialize)]
struct Foo {
    bar: Bar,
    bazzes: Vec<Baz>,
    boz: Box<Boz>,
}

#[derive(Serialize)]
struct Bar {
    next: Option<Box<Bar>>,
}

#[derive(Serialize)]
enum Baz {
    Bruf,
    Blof(Boz),
    Blof2(Boz, Bax),
    Bluf { bar: Bar, boz: Boz },
}

#[derive(Serialize)]
struct Bax;

#[derive(Serialize)]
struct Boz {
    number: u32,
    other_number: f32,
}

fn main() {
    let foo = Foo {
        bar: Bar {
            next: Some(Box::new(Bar {
                next: Some(Box::new(Bar { next: None })),
            })),
        },
        bazzes: vec![
            Baz::Bruf,
            Baz::Blof(Boz {
                number: 1,
                other_number: 100.0,
            }),
            Baz::Blof2(
                Boz {
                    number: 11,
                    other_number: -1.0,
                },
                Bax,
            ),
            Baz::Bluf {
                bar: Bar {
                    next: Some(Box::new(Bar { next: None })),
                },
                boz: Boz {
                    number: 99,
                    other_number: 10000.0,
                },
            },
        ],
        boz: Box::new(Boz {
            number: 10000,
            other_number: f32::INFINITY,
        }),
    };

    println!("{}", dot_serde::to_string(&foo).unwrap());
}
