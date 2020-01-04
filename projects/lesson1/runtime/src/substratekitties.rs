use support::{decl_storage, decl_module, ensure, traits::Randomness, decl_event};
use system::ensure_signed;
use codec::{Encode, Decode};
//use runtime_primitives::traits::{As, Zero};
//use sr_primitives::traits::As;
use rstd::cmp;

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

    fn breed_kitty(origin, kitty1: T::Hash, kitty2: T::Hash) {
        let sender = ensure_signed(origin)?;

        ensure!(<Kitties<T>>::exists(kitty1), "Kitty1 is not exists");
        ensure!(<Kitties<T>>::exists(kitty2), "Kitty2 is not exists");


        let random_hash = <randomness_collective_flip::Module<T>>::random_seed();

        let kitty_1 = <Kitties<T>>::get(kitty1);
        let kitty_2 = <Kitties<T>>::get(kitty2);

        let mut new_dna = kitty_1.dna;

        for(i, (dna_2, r)) in kitty_2.dna.as_ref().iter().zip(random_hash.as_ref().iter()).enumerate(){
            if r % 2 == 0 {
                new_dna.as_mut()[i] = *dna_2;
            }
        }

        let new_kitty = Kitty {
            id: random_hash,
            dna: new_dna,
            price: kitty_1.price,
            gen: cmp::max(kitty_1.gen, kitty_2.gen) + 1,
        };

        Self::mint(sender, random_hash, new_kitty);
    }

        fn buy_kitty(origin, kitty_hash: T::Hash, max_price: T::Balance) {
            let sender = ensure_signed(origin)?;
            ensure!(<Kitties<T>>::exists(kitty_hash), "This kitty is not exists");

            let owner = <KittyOwner<T>>::get(kitty_hash).ok_or("No owner of this kitty")?;
            ensure!(sender != owner, "You cannot buy your own kitty");

            let mut kitty = <Kitties<T>>::get(kitty_hash);

            let kitty_price = kitty.price;
       //     ensure!(!(kitty_price==0u128), "This kitty is not for sale.");
            ensure!(kitty_price <= max_price, "You price is not enough.");

            //<balances::Module<T> as Currency<_>>::transfer(&sender, &owner, kitty_price)?;

            Self::transfer_from(owner.clone(), sender.clone(), kitty_hash)
                .expect("`owner` is shown to own the kitty; \
                `owner` must have greater than 0 kitties, so transfer cannot cause underflow; \
                `all_kitty_count` shares the same type as `owned_kitty_count` \
                and minting ensure there won't ever be more than `max()` kitties, \
                which means transfer cannot cause an overflow; \
                qed");

            //kitty.price = <T::Balance as As<u64>>::sa(0);
            <Kitties<T>>::insert(kitty_hash, kitty);

           // Self::deposit_event(RawEvent::Bought(sender, owner, kitty_id, kitty_price));

        }


        fn transfer(origin, receiver: T::AccountId, kitty_hash: T::Hash) {
            let sender = ensure_signed(origin)?;
            ensure!(<Kitties<T>>::exists(kitty_hash), "This kitty is not exists.");

            let owner = <KittyOwner<T>>::get(kitty_hash).ok_or("No owner of this hash")?;
            ensure!( owner == sender, "You are not the owner." );

            Self::transfer_from(sender, receiver, kitty_hash);

        }

        fn set_price(origin, hash: T::Hash, price: T::Balance) {
            let sender = ensure_signed(origin)?;
            ensure!(<Kitties<T>>::exists(hash), "This kitty is not exists.");

            let owner = <KittyOwner<T>>::get(hash).ok_or("No owner of this hash")?;
            ensure!( owner == sender, "You are not the owner" );

            let mut kitty = <Kitties<T>>::get(hash);
            kitty.price = price;

            <Kitties<T>>::insert(hash, kitty);

            Self::deposit_event(RawEvent::PriceSet(sender, hash, price));
        }

        fn set_value(origin, value: u64) {
            let _sender = ensure_signed(origin)?;
            <Value>::put(value);
        }
        fn set_map_value(origin, key: T::AccountId, value: u64) {
            let _sender = ensure_signed(origin)?;
            <MyMap<T>>::insert(key, value);
        }
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

impl<T:Trait> Module<T> {
    fn transfer_from(sender: T::AccountId, receiver: T::AccountId, kitty_hash: T::Hash) ->Result<u64, &'static str> {
        
        let owner = <KittyOwner<T>>::get(kitty_hash).ok_or("No owner for this kitty")?;

        ensure!(owner == sender, "'from' account does not own this kitty");

        let from_last_index = <OwnedKittiesCount<T>>::get(&sender);
        let to_last_index = <OwnedKittiesCount<T>>::get(&receiver);

        let new_from_last_index = from_last_index.checked_sub(1).ok_or("underflow")?;
        let new_to_last_index = to_last_index.checked_add(1).ok_or("overflow")?;

        let kitty_pos = <OwnedKittiesIndex<T>>::get(kitty_hash);

        if new_from_last_index != kitty_pos {
            let last_from_kitty = <OwnedKittiesArray<T>>::get((&sender, new_from_last_index));
            <OwnedKittiesArray<T>>::insert((&sender, kitty_pos), last_from_kitty);
            <OwnedKittiesIndex<T>>::insert(last_from_kitty, kitty_pos);
        }

        <KittyOwner<T>>::insert(&kitty_hash, &receiver);
        <OwnedKittiesIndex<T>>::insert(&kitty_hash, to_last_index);
        <OwnedKittiesArray<T>>::remove((&sender, new_from_last_index));
        <OwnedKittiesArray<T>>::insert((&receiver, to_last_index), kitty_hash);

        <OwnedKittiesCount<T>>::insert(&sender, new_from_last_index);
        <OwnedKittiesCount<T>>::insert(&receiver, new_to_last_index);
        Ok(2)
    //     //Self::deposit_event(RawEvent::Transferred(from, to, kitty_hash));

    }

    fn mint(to: T::AccountId, kitty_id: T::Hash, new_kitty: Kitty<T::Balance, T::Hash>) ->Result<(), &'static str> {
        //ensure!(KittyOwner<T>>::exists(kitty_id), "Kitty already exists");

        let owned_kitty_count = <OwnedKittiesCount<T>>::get(&to);

        let new_owned_kitty_count = owned_kitty_count.checked_add(1).ok_or("Overflow adding a new kitty to account balance")?;

        let all_kitties_count = <AllKittiesCount>::get();

        let new_all_kitties_count = all_kitties_count.checked_add(1).ok_or("Overflow adding a new kitty to total supply")?;

        <Kitties<T>>::insert(kitty_id, new_kitty);
        <KittyOwner<T>>::insert(kitty_id, &to);

        <AllKittiesArray<T>>::insert(all_kitties_count, kitty_id);
        <AllKittiesCount>::put(new_all_kitties_count);
        <AllKittiesIndex<T>>::insert(kitty_id, all_kitties_count);

        <OwnedKittiesArray<T>>::insert((to.clone(), owned_kitty_count), kitty_id);
        <OwnedKittiesCount<T>>::insert(&to, new_owned_kitty_count);
        <OwnedKittiesIndex<T>>::insert(kitty_id, owned_kitty_count);

        Ok(())
        //Self::deposit_event(RawEvent::Created(to, kitty_id));
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
