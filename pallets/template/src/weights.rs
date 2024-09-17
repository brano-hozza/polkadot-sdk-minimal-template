use frame::prelude::*;

/// Weight functions needed for `pallet_lottery`.
pub trait WeightInfo {
    fn mint_unsafe() -> Weight;
    fn transfer() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn mint_unsafe() -> Weight {
        Weight::from_parts(29_722_000, 3593)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }

    fn transfer() -> Weight {
        Weight::from_parts(29_722_000, 3593)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
}
