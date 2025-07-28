macro_rules! btree_1 {
	( $($key: expr => $val: expr),* ) => {
		{
			let mut map = std::collections::BTreeMap::new();
			$(map.insert($key, $val);)*
			map
		}
	}
}
