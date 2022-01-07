macro_rules! push_struct_to_lua {
	{$lua:ident => $struct:ident => {$($field:ident),*}} => {
		$lua.new_table();

		$(
			$struct.$field.push_to_lua($lua);
			$lua.set_field(-2, concat!(stringify!($field), "\0").as_ptr() as *const _);
		)*
	};
}
pub trait PushToLua: Sized {
	unsafe fn push_to_lua(self, lua: gmod::lua::State);
}

// Primitives

impl PushToLua for &str {
	#[inline]
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		lua.push_string(self);
	}
}
impl PushToLua for bool {
	#[inline]
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		lua.push_boolean(self);
	}
}
impl PushToLua for u32 {
	#[inline]
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		lua.push_integer(self as _);
	}
}
impl PushToLua for f64 {
	#[inline]
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		lua.push_number(self);
	}
}
impl PushToLua for u16 {
	#[inline]
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		lua.push_number(self as _);
	}
}

// Containers

impl<T: PushToLua> PushToLua for Option<T> {
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		match self {
			Some(val) => val.push_to_lua(lua),
			None => lua.push_nil(),
		}
	}
}
impl<K: PushToLua, V: PushToLua> PushToLua for std::collections::BTreeMap<K, V> {
	#[inline]
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		lua.new_table();

		for (k, v) in self {
			k.push_to_lua(lua);
			v.push_to_lua(lua);
			lua.set_table(-3);
		}
	}
}
impl<T: PushToLua> PushToLua for Vec<T> {
	#[inline]
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		let len = self.len().min(i32::MAX as _) as i32;

		lua.create_table(len, 0);

		for (i, val) in self.into_iter().take(len as usize).enumerate() {
			val.push_to_lua(lua);
			lua.raw_seti(-2, (i + 1) as _);
		}
	}
}

// Structs

impl PushToLua for maxminddb::geoip2::model::Traits {
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		push_struct_to_lua! {
			lua => self => {
				is_anonymous_proxy,
				is_satellite_provider
			}
		}
	}
}
impl PushToLua for maxminddb::geoip2::model::Subdivision<'_> {
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		push_struct_to_lua! {
			lua => self => {
				geoname_id,
				iso_code,
				names
			}
		}
	}
}
impl PushToLua for maxminddb::geoip2::model::Postal<'_> {
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		push_struct_to_lua! {
			lua => self => {
				code
			}
		}
	}
}
impl PushToLua for maxminddb::geoip2::model::RepresentedCountry<'_> {
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		push_struct_to_lua! {
			lua => self => {
				names,
				iso_code,
				geoname_id
			}
		}
	}
}
impl PushToLua for maxminddb::geoip2::model::Country<'_> {
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		push_struct_to_lua! {
			lua => self => {
				names,
				is_in_european_union,
				iso_code,
				geoname_id
			}
		}
	}
}
impl PushToLua for maxminddb::geoip2::model::Continent<'_> {
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		push_struct_to_lua! {
			lua => self => {
				code,
				geoname_id,
				names
			}
		}
	}
}
impl PushToLua for maxminddb::geoip2::model::City<'_> {
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		push_struct_to_lua! {
			lua => self => {
				geoname_id,
				names
			}
		}
	}
}
impl PushToLua for maxminddb::geoip2::model::Location<'_> {
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		push_struct_to_lua! {
			lua => self => {
				latitude,
				longitude,
				metro_code,
				time_zone
			}
		}
	}
}
impl PushToLua for maxminddb::geoip2::Country<'_> {
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		push_struct_to_lua! {
			lua => self => {
				country,
				continent,
				registered_country,
				represented_country,
				traits
			}
		}
	}
}
impl PushToLua for maxminddb::geoip2::AnonymousIp {
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		push_struct_to_lua! {
			lua => self => {
				is_anonymous,
				is_anonymous_vpn,
				is_hosting_provider,
				is_public_proxy,
				is_tor_exit_node
			}
		}
	}
}
impl PushToLua for maxminddb::geoip2::Asn<'_> {
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		push_struct_to_lua! {
			lua => self => {
				autonomous_system_number,
				autonomous_system_organization
			}
		}
	}
}
impl PushToLua for maxminddb::geoip2::City<'_> {
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		push_struct_to_lua! {
			lua => self => {
				city,
				continent,
				country,
				location,
				postal,
				registered_country,
				represented_country,
				subdivisions,
				traits
			}
		}
	}
}
impl PushToLua for maxminddb::geoip2::ConnectionType<'_> {
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		push_struct_to_lua! {
			lua => self => {
				connection_type
			}
		}
	}
}
impl PushToLua for maxminddb::geoip2::DensityIncome {
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		push_struct_to_lua! {
			lua => self => {
				population_density,
				average_income
			}
		}
	}
}
impl PushToLua for maxminddb::geoip2::Domain<'_> {
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		push_struct_to_lua! {
			lua => self => {
				domain
			}
		}
	}
}
impl PushToLua for maxminddb::geoip2::Isp<'_> {
	unsafe fn push_to_lua(self, lua: gmod::lua::State) {
		push_struct_to_lua! {
			lua => self => {
				autonomous_system_number,
				autonomous_system_organization,
				isp,
				organization
			}
		}
	}
}
