struct Arenas {
    unfloat: Arena<UNFloatNodes>,
    snfloat: Arena<SNFloatNodes>,
}

trait Storage<T> {
    fn insert(&mut self, t: T) -> ArenaIndex<T>;
    fn get(&self, idx: ArenaIndex<T>) -> T;
}

impl Storage<UNFloatNodes> for Arenas {
    fn insert(&mut self, t: T) -> ArenaIndex<T> {
        self.unfloat.insert(t)
    }

    fn get(&self, idx: ArenaIndex<T>) -> T {
        self.unfloat.get(idx)
    }
}

trait Storable<T> {
    fn insert_into(self, storage: &mut T) -> ArenaIndex<Self>;
    fn get_from(idx: ArenaIndex<Self>, storage: &T) -> &Self;
}

impl<T, U> Storable<T> for U
where
    T: Storage<U>,
{
    fn insert(&mut self, t: T) -> ArenaIndex<T> {}

    fn get(&self, idx: ArenaIndex<T>) -> T {}
}

struct ArenaBox<T> {
    index: ArenaIndex,
}

impl Mutagen for ArenaBox<T> {
    type Arg = Arenas;
}

impl Generatable for ArenaBox<T>
where
    T: Storable<Arenas>,
{
    fn generate(arg: &'a mut Arenas) -> Self {
        let t = T::generate(arg);
        Self {
            index: t.store(arg),
        }
    }
}

enum UNFloatNodes {
    Constant { value: UNFloat },
    FromSNFloat { child: ArenaBox<SNFloatNodes> },
}

enum SNFloatNodes {
    Constant { value: SNFloat },
    FromUNFloat { child: ArenaBox<UNFloatNodes> },
}
