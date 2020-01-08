use support::{StorageMap, Parameter, ensure};
use sp_runtime::traits::Member;
use codec::{Encode, Decode};

#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq))]
#[derive(Encode, Decode)]
pub struct LinkedItem<Value> {
	pub prev: Option<Value>,
	pub next: Option<Value>,
}

pub struct LinkedList<Storage, Key, Value>(rstd::marker::PhantomData<(Storage, Key, Value)>);

impl<Storage, Key, Value> LinkedList<Storage, Key, Value> where
    Value: Parameter + Member + Copy,
    Key: Parameter,
    Storage: StorageMap<(Key, Option<Value>), LinkedItem<Value>, Query = Option<LinkedItem<Value>>>,
{
    fn read_head(key: &Key) -> LinkedItem<Value> {
 		Self::read(key, None)
 	}

  	fn write_head(account: &Key, item: LinkedItem<Value>) {
 		Self::write(account, None, item);
 	}

  	pub fn read(key: &Key, value: Option<Value>) -> LinkedItem<Value> {
 		Storage::get((&key, value)).unwrap_or_else(|| LinkedItem {
 			prev: None,
 			next: None,
 		})
 	}

  	pub fn write(key: &Key, value: Option<Value>, item: LinkedItem<Value>) {
 		Storage::insert((&key, value), item);
 	}

    pub fn append(key: &Key, value: Value) {
        // 作业：实现 append
		let head = Self::read_head(key);

		let new_head = LinkedItem {
			prev: Some(value),
			next: head.next
		};
		Self::write_head(key, new_head);

		let last = Self::read(key, head.prev);
		let new_last_sec = LinkedItem {
			prev: last.prev,
			next: Some(value)
		};
		Self::write(key, head.prev, new_last_sec);

		let new_last = LinkedItem {
			prev: head.prev,
			next: None
		};
		Self::write(key, Some(value), new_last);
    }

    pub fn remove(key: &Key, value: Value) {
        // 作业：实现 remove
		let item = Self::read(key, Some(value));

		let pre = Self::read(key, item.prev);
		let new_pre = LinkedItem {
			prev: pre.prev,
			next: item.next
		};
		Self::write(key, item.prev, new_pre);

		let next = Self::read(key, item.next);

		let new_next = LinkedItem {
			prev: item.prev,
			next: next.next
		};
		Self::write(key, item.next, new_next);

		Storage::remove((key, Some(value)));
    }
}