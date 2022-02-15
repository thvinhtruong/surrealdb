use crate::dbs::Options;
use crate::dbs::Runtime;
use crate::dbs::Transaction;
use crate::err::Error;
use crate::sql::array::Array;
use crate::sql::idiom::Idiom;
use crate::sql::value::Value;

impl Value {
	pub async fn array(
		&mut self,
		ctx: &Runtime,
		opt: &Options,
		txn: &Transaction<'_>,
		path: &Idiom,
	) -> Result<(), Error> {
		let val = Value::from(Array::default());
		self.set(ctx, opt, txn, path, val).await
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::dbs::test::mock;
	use crate::sql::test::Parse;

	#[tokio::test]
	async fn array_none() {
		let (ctx, opt, txn) = mock().await;
		let idi = Idiom::default();
		let mut val = Value::parse("{ test: { other: null, something: 123 } }");
		let res = Value::parse("[]");
		val.array(&ctx, &opt, &txn, &idi).await.unwrap();
		assert_eq!(res, val);
	}

	#[tokio::test]
	async fn array_path() {
		let (ctx, opt, txn) = mock().await;
		let idi = Idiom::parse("test");
		let mut val = Value::parse("{ test: { other: null, something: 123 } }");
		let res = Value::parse("{ test: [] }");
		val.array(&ctx, &opt, &txn, &idi).await.unwrap();
		assert_eq!(res, val);
	}
}