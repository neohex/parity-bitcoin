use bytes::Bytes;
use hash::{H160, H256, H264, H512, H520};
use ser::{Serializable, Stream, Deserializable, Reader, Error};
use ser::compact_integer::CompactInteger;

macro_rules! impl_ser_for_hash {
	($name: ident, $size: expr) => {
		impl Serializable for $name {
			fn serialize(&self, stream: &mut Stream) {
				stream.append_slice(&**self);
			}
		}

		impl Deserializable for $name {
			fn deserialize(reader: &mut Reader) -> Result<Self, Error> where Self: Sized {
				let slice = try!(reader.read_slice($size));
				let mut result = Self::default();
				result.copy_from_slice(slice);
				Ok(result)
			}
		}
	}
}

impl_ser_for_hash!(H160, 20);
impl_ser_for_hash!(H256, 32);
impl_ser_for_hash!(H264, 33);
impl_ser_for_hash!(H512, 64);
impl_ser_for_hash!(H520, 65);

impl Serializable for Bytes {
	fn serialize(&self, stream: &mut Stream) {
		stream
			.append(&CompactInteger::from(self.len()))
			.append_slice(&self);
	}
}

impl Deserializable for Bytes {
	fn deserialize(reader: &mut Reader) -> Result<Self, Error> where Self: Sized {
		let len = try!(reader.read::<CompactInteger>());
		reader.read_slice(len.into()).map(|b| b.to_vec().into())
	}
}

#[cfg(test)]
mod tests {
	use bytes::Bytes;
	use ser::{serialize, deserialize};

	#[test]
	fn test_bytes_deserialize() {
		let raw = vec![0x02, 0x01, 0x45];
		let expected: Bytes = "0145".into();
		assert_eq!(expected, deserialize(&raw).unwrap());
	}

	#[test]
	fn test_bytes_serialize() {
		let expected = vec![0x02, 0x01, 0x45];
		let bytes: Bytes = "0145".into();
		assert_eq!(expected, serialize(&bytes));
	}
}