#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_error,decl_event, dispatch, traits::Get};
use frame_system::ensure_signed;

use frame_support::traits::Vec;
use codec::{Encode,Decode};


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, Debug)]
pub struct Property<AccountId> {
    id: u32,
    name: Vec<u8>,
    area: Vec<u8>,
	prop_owner: AccountId,
	value: Vec<u8>,
	
}


// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	//// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}




// pub type ProductOf<T> =
//     Product<<T as frame_system::Config>::AccountId>;


// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	
	trait Store for Module<T: Config> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		 Something get(fn something): Option<u32>;

		 PropertyInformation get(fn property_information):
		 map hasher(blake2_128_concat) u32 => Option<Property<T::AccountId>>;
		 //Owner get(fn new_owner): T::AccountId;
		 OwnerOf get(fn owner_of): map hasher(blake2_128_concat) u32 => Option<T::AccountId>;
	}

}
decl_error!{
	pub enum Error for Module<T:Config>{
	
		NoneValue,
		StorageOverflow,
	  }
	}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where 
	 <T as frame_system::Config>::AccountId{
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32,AccountId),
		PropertyInformationStored(AccountId,Vec<u8>,u32,Vec<u8>,Vec<u8>),
		OwnershipTransferred(AccountId,AccountId, u32),
		
	}
		
	
);



// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		fn add_property(origin,id: u32, name: Vec<u8>, area: Vec<u8>, value: Vec<u8>) -> dispatch::DispatchResult {

            // The dispatch origin of this call must be `ManufactureOrigin`.
            let sender = ensure_signed(origin)?;
           
            let property = Property {
                id,
                name: name.clone(),
				area: area.clone(),
				prop_owner: sender.clone(),
				value: value.clone(),
				
            };

			<PropertyInformation<T>>::insert(&id, property);
			<OwnerOf<T>>::insert(&id, &sender);
            // Emit the event that barcode has been added in chain for a product
            Self::deposit_event(RawEvent::PropertyInformationStored(sender,name,id, value, area));

            // Return a successful DispatchResult
            Ok(())
        }

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		fn transfer_ownership(origin, newOwner: T::AccountId, id: u32) -> dispatch::DispatchResult {
			
            let sender = ensure_signed(origin)?;
           // ensure!(sender == Self::owner(), "This function can only be called by the owner");
		   let property = <PropertyInformation<T>>::get(id);

           <OwnerOf<T>>::insert(&id, &newOwner);
			
            Self::deposit_event(RawEvent::OwnershipTransferred(sender, newOwner, id));
            Ok(())
        }

		
		  
    }
}
	


