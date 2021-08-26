#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame



use frame_support::{decl_module, decl_storage, decl_error,decl_event, dispatch, ensure, traits::Get};
use frame_system::ensure_signed;

use frame_support::traits::Vec;
use codec::{Encode,Decode};


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, Debug)]
pub struct Property {
    id: u64,
    name: Vec<u8>,
    area: Vec<u8>,
    value: Vec<u8>,  
}

// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	//// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_event! {
    pub enum Event<T> where 
    <T as frame_system::Config>::AccountId{
        Created(AccountId, u64),
        Transferred(AccountId, AccountId, u64),
    }
}

decl_storage! {
    trait Store for Module<T: Config> as PropertyModule {
        Properties get(fn property): map hasher(blake2_128_concat) u64 => Option<Property>;
        PropertyOwner get(fn owner_of): map hasher(blake2_128_concat) u64 => Option<T::AccountId>;
        
        AllPropertiesArray get(fn property_by_index): map hasher(blake2_128_concat) u64 => u64;
        AllPropertiesCount get(fn all_properties_count): u64;
        AllPropertiesIndex: map hasher(blake2_128_concat) u64 => u64;

        OwnedPropertiesArray get(fn property_of_owner_by_index): map hasher(blake2_128_concat) (T::AccountId, u64) => u64;
        OwnedPropertiesCount get(fn owned_property_count): map hasher(blake2_128_concat) T::AccountId => u64;
        OwnedPropertiesIndex: map hasher(blake2_128_concat) u64 => u64;

        Nonce: u64;
    }
}


decl_error!{
	pub enum Error for Module<T:Config>{
	
		NoneValue,
		StorageOverflow,
	  }
	}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        // Declare public functions here

        fn deposit_event() = default;

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        fn create_property(origin, id: u64, name: Vec<u8>, area: Vec<u8>, value: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            let nonce = <Nonce>::get();
            // let random_seed = <pallet_randomness_collective_flip::Module<T>>::random_seed();
            // let random_hash = (random_seed, &sender, nonce).using_encoded(BlakeTwo256::hash);

            let new_property = Property {
                id,
                name: name.clone(),
                area: area.clone(),
                value: value.clone(),
            };

            Self::mint(sender, id, new_property)?;

            <Nonce>::mutate(|n| *n += 1);

            Ok(())
        }

       
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        fn transfer(origin, to: T::AccountId, id: u64) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            let owner = Self::owner_of(id).ok_or("No owner for this Property")?;
            ensure!(owner == sender, "You do not own this property");

            Self::transfer_from(sender, to, id)?;

            Ok(())
        }

        
        
    }
}

impl<T: Config> Module<T> {
    fn mint(to: T::AccountId, id: u64, new_property: Property) -> dispatch::DispatchResult {
        // #####
        ensure!(!<PropertyOwner<T>>::contains_key(id), "Property already exists");

        let owned_property_count = Self::owned_property_count(&to);

        let new_owned_property_count = owned_property_count.checked_add(1)
            .ok_or("Overflow adding a new property to account balance")?;

        let all_properties_count = Self::all_properties_count();

        let new_all_properties_count = all_properties_count.checked_add(1)
            .ok_or("Overflow adding a new property to total supply")?;

        <Properties>::insert(id, new_property);
        <PropertyOwner<T>>::insert(id, &to);

        <AllPropertiesArray>::insert(all_properties_count, id);
        <AllPropertiesCount>::put(new_all_properties_count);
        <AllPropertiesIndex>::insert(id, all_properties_count);

        <OwnedPropertiesArray<T>>::insert((to.clone(), owned_property_count), id);
        <OwnedPropertiesCount<T>>::insert(&to, new_owned_property_count);
        <OwnedPropertiesIndex>::insert(id, owned_property_count);

        Self::deposit_event(RawEvent::Created(to, id));

        Ok(())
    }

    fn transfer_from(from: T::AccountId, to: T::AccountId, id: u64) -> dispatch::DispatchResult {
        let owner = Self::owner_of(id).ok_or("No owner for this property")?;

        ensure!(owner == from, "'from' account does not own this property");

        let owned_property_count_from = Self::owned_property_count(&from);
        let owned_property_count_to = Self::owned_property_count(&to);

        let new_owned_property_count_to = owned_property_count_to.checked_add(1)
            .ok_or("Transfer causes overflow of 'to' property balance")?;

        let new_owned_property_count_from = owned_property_count_from.checked_sub(1)
            .ok_or("Transfer causes underflow of 'from' property balance")?;

        let property_index = <OwnedPropertiesIndex>::get(id);
        if property_index != new_owned_property_count_from {
            let last_property_id = <OwnedPropertiesArray<T>>::get((from.clone(), new_owned_property_count_from));
            <OwnedPropertiesArray<T>>::insert((from.clone(), property_index), last_property_id);
            <OwnedPropertiesIndex>::insert(last_property_id, property_index);
        }

        <PropertyOwner<T>>::insert(&id, &to);
        <OwnedPropertiesIndex>::insert(id, owned_property_count_to);

        <OwnedPropertiesArray<T>>::remove((from.clone(), new_owned_property_count_from));
        <OwnedPropertiesArray<T>>::insert((to.clone(), owned_property_count_to), id);

        <OwnedPropertiesCount<T>>::insert(&from, new_owned_property_count_from);
        <OwnedPropertiesCount<T>>::insert(&to, new_owned_property_count_to);

        Self::deposit_event(RawEvent::Transferred(from, to, id));

        Ok(())
    }
}