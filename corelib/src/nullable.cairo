use box::BoxTrait;
use traits::Default;
use traits::Felt252DictValue;

pub extern type Nullable<T>;

pub enum FromNullableResult<T> {
    Null: (),
    NotNull: Box<T>,
}

pub extern fn null<T>() -> Nullable<T> nopanic;
pub extern fn nullable_from_box<T>(value: Box<T>) -> Nullable<T> nopanic;
pub extern fn match_nullable<T>(value: Nullable<T>) -> FromNullableResult<T> nopanic;

pub trait NullableTrait<T> {
    fn deref(self: Nullable<T>) -> T;
}

impl NullableImpl<T> of NullableTrait<T> {
    fn deref(self: Nullable<T>) -> T {
        match match_nullable(self) {
            FromNullableResult::Null(()) => panic_with_felt252('Attempted to deref null value'),
            FromNullableResult::NotNull(value) => value.unbox(),
        }
    }
}

// Impls for generic types
impl NullableCopy<T, impl TCopy: Copy<T>> of Copy<Nullable<T>>;
impl NullableDrop<T, impl TDrop: Drop<T>> of Drop<Nullable<T>>;

impl NullableDefault<T> of Default<Nullable<T>> {
    #[inline(always)]
    fn default() -> Nullable<T> nopanic {
        null()
    }
}

impl NullableFelt252DictValue<T> of Felt252DictValue<Nullable<T>> {
    #[inline(always)]
    fn zero_default() -> Nullable<T> nopanic {
        null()
    }
}
