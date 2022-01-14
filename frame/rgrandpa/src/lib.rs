#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;
use frame_support::traits::{ValidatorSet,OneSessionHandler};
use sp_std::vec::Vec;
use sp_core::crypto::KeyTypeId;
use pallet_staking;
use pallet_babe;
use sp_core::{U256, crypto::Public};
use pallet_grandpa;
use pallet_grandpa::AuthorityList;
use sp_runtime::{Percent, traits::Zero};
pub mod weights;
pub use weights::WeightInfo;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"rgra");

pub mod crypto {
	use super::KEY_TYPE;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
	};
	use sp_core::sr25519::Signature as Sr25519Signature;
	app_crypto!(sr25519, KEY_TYPE);
}

/// Identity of a rGrandpa authority.
pub type AuthorityId = crypto::Public;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;
	use codec::{Decode, Encode, EncodeLike};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_staking::Config + pallet_babe::Config + pallet_grandpa::Config{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn cycle_confirmer_num)]
	// percent of grandpa consensus count
	pub type CyclePercent<T> = StorageValue<_, u8, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn cycle_block_num)]
	// how many block past then may change the validator count for grandpa
	pub type CycleBlockNum<T:Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn nex_change_at)]
	// block number of next change
	pub type NextChangeAt<T:Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn r_next_authorities)]
	// all author we need store,it can be changed on new session,then we need refresh
	pub type RNextAuthorities<T:Config> = StorageValue<_, AuthorityList, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [ CyclePercent, CycleBlockNum]
		ParameterStored( u8, T::BlockNumber),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// new validator count is less then minimum_validator_count.
		LessThenMin,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: T::BlockNumber) -> Weight{
            let mut nextChangeAt = <NextChangeAt<T>>::get();
            let cycleBlockNum = <CycleBlockNum<T>>::get();
            if n == nextChangeAt && nextChangeAt!= Zero::zero() {
                nextChangeAt += cycleBlockNum;
                <NextChangeAt<T>>::put(nextChangeAt);
                //then set rgrandpa validator count
                Self::set_random_validator_count();
				10_000 + T::DbWeight::get().reads_writes(2, 1)
            } else {
				T::DbWeight::get().reads(2)
			}

        }
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {

		#[pallet::weight(<T as pallet::Config>::WeightInfo::set_parameter())]
		pub fn set_parameter(origin: OriginFor<T>, percent: u8,bnum:T::BlockNumber) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			ensure_root(origin)?;
			//parameter need valid
			let mut min_num  = <pallet_staking::Module<T>>::minimum_validator_count();
			let mut org_num	= <pallet_staking::Module<T>>::validator_count();
			org_num =  Percent::from_percent(percent as u8) * org_num;
			if org_num >= min_num {
				// Update storage.
				<CyclePercent<T>>::put(percent);
				<CycleBlockNum<T>>::put(bnum);
				let nextChangeAt = <frame_system::Module<T>>::block_number() + bnum;
				<NextChangeAt<T>>::put(nextChangeAt);
				Self::deposit_event(Event::ParameterStored( percent, bnum));
				// Return a successful DispatchResultWithPostInfo
				Ok(().into())
			} else {
				Err(Error::<T>::LessThenMin)?
			}
		}
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub percent: u8,
		pub block_num: T::BlockNumber,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> where
		T::BlockNumber: EncodeLike<u32>
	{
		fn default() -> Self {
			Self {
				percent: 0,
				block_num:T::BlockNumber::zero(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> where
		T::BlockNumber: EncodeLike<u32>
	{
		fn build(&self) {
			<CyclePercent<T>>::put(self.percent);
			<CycleBlockNum<T>>::put(self.block_num);
			<NextChangeAt<T>>::put(self.block_num);
		}
	}
}

impl<T: Config> Pallet<T>{

    pub fn set_random_validator_count() {

        let minNum  = <pallet_staking::Module<T>>::minimum_validator_count();

		let orgNum	= <pallet_staking::Module<T>>::validator_count();

		let mut percent = <CyclePercent<T>>::get();

		if percent > 100 {
			percent = 100;
		}

        let mut confirmerNum = Percent::from_percent(percent as u8)* orgNum;

		if confirmerNum < minNum {
			confirmerNum = minNum;
		}

        let randomness = <pallet_babe::Module<T>>::randomness();

        let rand = U256::from(randomness);

        let mut auth_list = <RNextAuthorities<T>>::get();

        let mut count = auth_list.iter().count();

		if count < confirmerNum as usize {
			return;
		}
        let mut rand_auth:AuthorityList = Vec::new();
        for i in 0..confirmerNum {
            let j = (rand % U256::from(count)).as_u32() as usize;
            rand_auth.push(auth_list.get(j).unwrap().clone());
            auth_list.remove(j);
            count-=1;
        }

        // for (auth,weight) in rand_auth.iter() {
        //     log::info!("============={:?}->{:?}",auth, weight);
        // }
        <pallet_grandpa::Module<T>>::schedule_change(rand_auth, Zero::zero(), None);
    }
}

impl<T: Config> sp_runtime::BoundToRuntimeAppPublic for Module<T> {
	type Public = AuthorityId;
}
impl<T> OneSessionHandler<T::AccountId> for Module<T>
 where T: Config {
    type Key = AuthorityId;

    fn on_genesis_session<'a, I: 'a>(validators: I)
        where I: Iterator<Item=(&'a T::AccountId, AuthorityId)>
    {
		let keys = <RNextAuthorities<T>>::get();
		if keys.is_empty() {
			let auth_list = <pallet_grandpa::Module<T>>::grandpa_authorities();
			<RNextAuthorities<T>>::put(auth_list);
		}
    }

    fn on_new_session<'a, I: 'a>(changed: bool, validators: I, queued_validators: I)
        where I: Iterator<Item=(&'a T::AccountId, AuthorityId)>
    {
        if changed {
            if let Some(pending_change) = <pallet_grandpa::Module<T>>::pending_change() {
				<RNextAuthorities<T>>::put(pending_change.next_authorities);
            }
        }
    }

    fn on_disabled(i: usize) {
    }
}