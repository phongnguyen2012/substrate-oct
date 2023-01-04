
use super::*;

impl<T: Config<I>, I: 'static> Pallet<T, I> {
	/// Get the owner of the item, if the item exists.
	pub fn owner(collection: T::CollectionId, item: T::ItemId) -> Option<T::AccountId> {
		Item::<T, I>::get(collection, item).map(|i| i.owner)
	}

	/// Get the owner of the collection, if the collection exists.
	pub fn collection_owner(collection: T::CollectionId) -> Option<T::AccountId> {
		Collection::<T, I>::get(collection).map(|i| i.owner)
	}

	#[cfg(any(test, feature = "runtime-benchmarks"))]
	pub fn set_next_id(id: T::CollectionId) {
		NextCollectionId::<T, I>::set(Some(id));
	}

	#[cfg(test)]
	pub fn get_next_id() -> T::CollectionId {
		NextCollectionId::<T, I>::get().unwrap_or(T::CollectionId::initial_value())
	}
}