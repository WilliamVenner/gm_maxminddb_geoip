#![feature(c_unwind)]

#[macro_use] extern crate gmod;
#[macro_use] extern crate thiserror;

use std::{borrow::Cow, cell::RefCell, net::IpAddr, path::PathBuf, str::FromStr};
use maxminddb::MaxMindDBError;

mod serialize;
use serialize::PushToLua;

enum MaybeBorrowed<'a, T> {
	Borrowed(&'a T),
	Owned(T),
}
impl<T> AsRef<T> for MaybeBorrowed<'_, T> {
	#[inline]
	fn as_ref(&self) -> &T {
		match self {
			MaybeBorrowed::Borrowed(borrowed) => borrowed,
			MaybeBorrowed::Owned(owned) => owned,
		}
	}
}

#[derive(Error, Debug)]
pub enum DBError {
	#[error("{0}")]
	Internal(#[from] MaxMindDBError),

	#[error("You didn't install the MaxMindDB database! I expected to find one in garrysmod/maxminddb.mmdb or garrysmod/data/maxminddb.dat, you can get it from here: https://maxmind.com")]
	NotInstalled,
}

fn init_db() -> Result<maxminddb::Reader<memmap2::Mmap>, DBError> {
	if PathBuf::from("garrysmod/maxminddb.mmdb").exists() {
		// Try static garrysmod/maxminddb.mmdb first
		maxminddb::Reader::open_mmap("garrysmod/maxminddb.mmdb").map_err(Into::into)
	} else if PathBuf::from("garrysmod/data/maxminddb.dat").exists() {
		// Then try loading from data/
		maxminddb::Reader::open_mmap("garrysmod/data/maxminddb.dat").map_err(Into::into)
	} else {
		Err(DBError::NotInstalled)
	}
}

thread_local! {
	static DB: RefCell<Result<maxminddb::Reader<memmap2::Mmap>, DBError>> = RefCell::new(init_db());
}

#[lua_function]
unsafe fn refresh(lua: gmod::lua::State) -> i32 {
	DB.with(|db| match init_db() {
		Ok(refreshed) => {
			lua.push_boolean(true);

			*db.borrow_mut() = Ok(refreshed);

			1
		}
		Err(refreshed) => {
			lua.push_boolean(false);
			lua.push_string(&refreshed.to_string());

			if let Err(ref mut err) = *db.borrow_mut() {
				*err = refreshed;
			}

			2
		}
	})
}

#[lua_function]
unsafe fn query(lua: gmod::lua::State) -> i32 {
	let ip_addr = match IpAddr::from_str(lua.check_string(1).as_ref()) {
		Ok(ip_addr) => ip_addr,
		Err(err) => lua.error(&format!("Invalid IP address: {}", err)),
	};

	let record = match GeoIPRecord::try_from(lua.check_integer(2)) {
		Ok(record) => record,
		Err(_) => lua.error("Unknown or invalid GeoIP record type"),
	};

	DB.with(|db| {
		match db
			.borrow()
			.as_ref()
			.map_err(MaybeBorrowed::Borrowed)
			.and_then(|db| {
				record
					.lookup(lua, db, ip_addr)
					.map_err(MaybeBorrowed::Owned)
			}) {
			Ok(_) => 1,
			Err(err) => {
				lua.push_nil();
				lua.push_string(&err.as_ref().to_string());
				2
			}
		}
	})
}

#[lua_function]
unsafe fn country(lua: gmod::lua::State) -> i32 {
	let ip_addr = match IpAddr::from_str(lua.check_string(1).as_ref()) {
		Ok(ip_addr) => ip_addr,
		Err(err) => lua.error(&format!("Invalid IP address: {}", err)),
	};

	let lang = lua.get_string(2).unwrap_or(Cow::Borrowed("en"));

	#[inline]
	fn extract_country<'a, 'b>(
		country: maxminddb::geoip2::Country<'b>,
		lang: &'a str,
	) -> Option<&'b str> {
		let names = &country.country?.names?;
		names
			.get(lang)
			.or_else(|| names.get("en"))
			.or_else(|| names.get("en-US"))
			.or_else(|| names.into_iter().next().map(|(_, v)| v))
			.map(|country| *country)
	}

	DB.with(|db| {
		match db
			.borrow()
			.as_ref()
			.map_err(MaybeBorrowed::Borrowed)
			.and_then(|db| {
				db.lookup::<maxminddb::geoip2::Country>(ip_addr)
					.map_err(Into::into)
					.map_err(MaybeBorrowed::Owned)
			}) {
			Ok(country) => {
				if let Some(country) = extract_country(country, lang.as_ref()) {
					lua.push_string(country);
				} else {
					lua.push_nil();
				}
				1
			}
			Err(err) => {
				lua.push_nil();
				lua.push_string(&err.as_ref().to_string());
				2
			}
		}
	})
}

macro_rules! geoip_records {
	{$first_record:ident, $($record:ident),*} => {
		#[repr(isize)]
		pub enum GeoIPRecord {
			$first_record = 0,
			$($record,)*
		}
		impl TryFrom<isize> for GeoIPRecord {
			type Error = isize;

			fn try_from(value: isize) -> Result<Self, Self::Error> {
				#[allow(non_upper_case_globals)] const $first_record: isize = GeoIPRecord::$first_record as isize;
				$(#[allow(non_upper_case_globals)] const $record: isize = GeoIPRecord::$record as isize;)*

				#[allow(non_upper_case_globals)]
				match value {
					$first_record => Ok(GeoIPRecord::$first_record),
					$($record => Ok(GeoIPRecord::$record),)*
					_ => Err(value)
				}
			}
		}
		impl GeoIPRecord {
			fn lookup<T: AsRef<[u8]>>(self, lua: gmod::lua::State, db: &maxminddb::Reader<T>, ip_addr: IpAddr) -> Result<(), DBError> {
				unsafe {
					match self {
						GeoIPRecord::$first_record => db.lookup::<maxminddb::geoip2::$first_record>(ip_addr)?.push_to_lua(lua),
						$(GeoIPRecord::$record => db.lookup::<maxminddb::geoip2::$record>(ip_addr)?.push_to_lua(lua),)*
					}
				}
				Ok(())
			}
		}

		#[gmod13_open]
		unsafe fn gmod13_open(lua: gmod::lua::State) -> i32 {
			lua_stack_guard!(lua => {
				lua.new_table();

				lua.push_string(env!("CARGO_PKG_VERSION"));
				lua.set_field(-2, lua_string!("VERSION"));

				lua.push_function(refresh);
				lua.set_field(-2, lua_string!("refresh"));

				lua.push_function(country);
				lua.set_field(-2, lua_string!("country"));

				lua.push_function(query);
				lua.set_field(-2, lua_string!("query"));

				lua.new_table();

				lua.push_integer(GeoIPRecord::$first_record as isize);
				lua.set_field(-2, concat!(stringify!($first_record), "\0").as_ptr() as *const _);

				$(
					lua.push_integer(GeoIPRecord::$record as isize);
					lua.set_field(-2, concat!(stringify!($record), "\0").as_ptr() as *const _);
				)*

				lua.set_field(-2, lua_string!("records"));

				lua.set_global(lua_string!("maxminddb"));
			});
			0
		}
	};
}
geoip_records! {
	AnonymousIp,
	Asn,
	City,
	ConnectionType,
	Country,
	DensityIncome,
	Domain,
	Isp
}
