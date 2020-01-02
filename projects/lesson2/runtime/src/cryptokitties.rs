use support::{decl_storage, decl_module, ensure, traits::Randomness, decl_event};
use system::ensure_signed;
use codec::{Encode, Decode};

//use crate::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashT;

pub trait Trait: balances::Trait {
   // type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
   type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

}
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<Balance, Hash> {
    id: Hash, 
    dna: Hash, 
    price: Balance,
    gen: u64
}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        Value: u64;
        MyMap get(map_getter): map T::AccountId => u64;

        Kitties: map T::Hash => Kitty<T::Balance, T::Hash>;
        KittyOwner: map T::Hash => Option<T::AccountId>;

        AllKittiesArray get(kitty_by_index): map u64 => T::Hash;
        AllKittiesCount get(kitty_count): u64;
        AllKittiesIndex : map T::Hash => u64;

        OwnedKittiesArray: map (T::AccountId, u64) => T::Hash;
        OwnedKittiesCount: map T::AccountId => u64;
        OwnedKittiesIndex: map T::Hash => u64;

    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn deposit_event() = default;

        fn create_kitty(origin, balance: T::Balance) {
            let sender = ensure_signed(origin)?;

			let random_hash = <randomness_collective_flip::Module<T>>::random_seed();
            
            ensure!(!<KittyOwner<T>>::exists(random_hash), "Kitty already exists");

            let my_zero_balance = balance;
            let kitty = Kitty {
                id: random_hash,
                dna: random_hash,
                price: my_zero_balance,
                gen: 0
            };

            <Kitties<T>>::insert(random_hash, kitty);
            <KittyOwner<T>>::insert(random_hash, &sender);

            let kitty_count = Self::kitty_count();
            let new_kitty_count = kitty_count.checked_add(1).ok_or("Overflow add one...")?;

            <AllKittiesArray<T>>::insert(kitty_count, random_hash);
            <AllKittiesCount>::put(new_kitty_count);
            <AllKittiesIndex<T>>::insert(random_hash, kitty_count);

            let owned_kitty_count = <OwnedKittiesCount<T>>::get(&sender);
            let new_owned_kitty_count = owned_kitty_count.checked_add(1).ok_or("Overflow add one...")?;
            
            <OwnedKittiesArray<T>>::insert((&sender, owned_kitty_count), random_hash);
            <OwnedKittiesCount<T>>::insert(&sender, new_owned_kitty_count);
            <OwnedKittiesIndex<T>>::insert(random_hash, owned_kitty_count);

            Self::deposit_event(RawEvent::Created(sender, random_hash));
        }
    }
}

decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Hash,
        <T as balances::Trait>::Balance
    {
        Created(AccountId, Hash),
        PriceSet(AccountId, Hash, Balance),
    }
);
