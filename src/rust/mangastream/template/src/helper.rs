use aidoku::{
	prelude::format,
	std::{defaults::defaults_get, html::Node, String, StringRef, Vec},
	MangaStatus,
};

use crate::template::MangaStreamSource;

// generate url for listing page
pub fn get_listing_url(
	listing: [&str; 3],
	base_url: String,
	pathname: String,
	listing_name: String,
	page: i32,
) -> String {
	let list_type = if listing_name == listing[0] {
		"order=update"
	} else if listing_name == listing[1] {
		"order=popular"
	} else if listing_name == listing[2] {
		"order=latest"
	} else {
		""
	};
	match page {
		1 => format!("{}/{}/?{}", base_url, pathname, list_type),
		_ => format!("{}/{}/?page={}&{}", base_url, pathname, page, list_type),
	}
}

// return the manga status
pub fn manga_status(
	status: String,
	status_options: [&'static str; 5],
	status_options_2: [&'static str; 5],
) -> MangaStatus {
	if status.contains(status_options[0]) || status.contains(status_options_2[0]) {
		MangaStatus::Ongoing
	} else if status.contains(status_options[1]) || status.contains(status_options_2[1]) {
		MangaStatus::Completed
	} else if status.contains(status_options[2]) || status.contains(status_options_2[2]) {
		MangaStatus::Hiatus
	} else if status.contains(status_options[3])
		|| status.contains(status_options[4])
		|| status.contains(status_options_2[3])
		|| status.contains(status_options_2[4])
	{
		MangaStatus::Cancelled
	} else {
		MangaStatus::Unknown
	}
}

//converts integer(i32) to string
pub fn i32_to_string(mut integer: i32) -> String {
	if integer == 0 {
		return String::from("0");
	}
	let mut string = String::with_capacity(11);
	let pos = if integer < 0 {
		string.insert(0, '-');
		1
	} else {
		0
	};
	while integer != 0 {
		let mut digit = integer % 10;
		if pos == 1 {
			digit *= -1;
		}
		string.insert(pos, char::from_u32((digit as u32) + ('0' as u32)).unwrap());
		integer /= 10;
	}
	string
}

// return chpater number from string
pub fn get_chapter_number(id: String) -> f32 {
	id.chars()
		.filter(|a| (*a >= '0' && *a <= '9') || *a == ' ' || *a == '.')
		.collect::<String>()
		.split(' ')
		.collect::<Vec<&str>>()
		.into_iter()
		.map(|a| a.parse::<f32>().unwrap_or(0.0))
		.find(|a| *a > 0.0)
		.unwrap_or(0.0)
}

// generates the search, filter and homepage url
pub fn get_search_url(
	source: &MangaStreamSource,
	query: String,
	page: i32,
	included_tags: Vec<String>,
	excluded_tags: Vec<String>,
	status: String,
	manga_type: String,
) -> String {
	let mut url = format!("{}/{}", source.base_url, source.traverse_pathname);
	if query.is_empty() && included_tags.is_empty() && status.is_empty() && manga_type.is_empty() {
		return get_listing_url(
			source.listing,
			source.base_url.clone(),
			String::from(source.traverse_pathname),
			String::from(source.listing[0]),
			page,
		);
	}
	if !query.is_empty() {
		url.push_str(&format!("/page/{}?s={}", page, query.replace(' ', "+")))
	} else {
		url.push_str(&format!("/?page={}", page));
	}
	if !included_tags.is_empty() || !excluded_tags.is_empty() {
		if excluded_tags.is_empty() {
			for tag in included_tags {
				url.push_str(&format!("&genre%5B%5D={}", tag));
			}
		} else if !included_tags.is_empty() && !excluded_tags.is_empty() {
			for tag in included_tags {
				url.push_str(&format!("&genre%5B%5D={}", tag));
			}
			for tag in excluded_tags {
				url.push_str(&format!("&genre%5B%5D=-{}", tag));
			}
		} else {
			for tag in excluded_tags {
				url.push_str(&format!("&genre%5B%5D=-{}", tag));
			}
		}
	}
	if !status.is_empty() {
		url.push_str(&format!("&status={}", status));
	}
	if !manga_type.is_empty() {
		url.push_str(&format!("&type={}", manga_type));
	}
	url
}

// return the date depending on the language
pub fn get_date(source: &MangaStreamSource, raw_date: StringRef) -> f64 {
	match source.base_url.contains(source.date_string) {
		true => raw_date
			.0
			.as_date(source.chapter_date_format_2, Some(source.locale_2), None)
			.unwrap_or(0.0),
		_ => raw_date
			.0
			.as_date(source.chapter_date_format, Some(source.locale), None)
			.unwrap_or(0.0),
	}
}

// encoding non alpha-numeric characters to utf8
pub fn img_url_encode(string: String) -> String {
	let mut result: Vec<u8> = Vec::with_capacity(string.len() * 3);
	let hex = "0123456789abcdef".as_bytes();
	let bytes = string.as_bytes();

	for byte in bytes {
		let curr = *byte;
		if curr == b'-' {
			result.push(b'-');
		} else if curr == b'.' {
			result.push(b'.');
		} else if curr == b'_' {
			result.push(b'_');
		} else if curr.is_ascii_lowercase() || curr.is_ascii_uppercase() || curr.is_ascii_digit() {
			result.push(curr);
		} else {
			result.push(b'%');
			if hex[curr as usize >> 4] >= 97 && hex[curr as usize >> 4] <= 122 {
				result.push(hex[curr as usize >> 4] - 32);
			} else {
				result.push(hex[curr as usize >> 4]);
			}
			if hex[curr as usize & 15] >= 97 && hex[curr as usize & 15] <= 122 {
				result.push(hex[curr as usize & 15] - 32);
			} else {
				result.push(hex[curr as usize & 15]);
			}
		}
	}
	String::from_utf8(result).unwrap_or_default()
}

//get the image sources as some images are in base64 format
pub fn get_image_src(node: Node) -> String {
	let mut image = String::new();
	let src = node.select("img").first().attr("src").read();
	let data_lazy = node.select("img").first().attr("data-lazy-src").read();
	let data_src = node.select("img").first().attr("data-src").read();
	if !src.starts_with("data") && !src.is_empty() {
		image = node
			.select("img")
			.first()
			.attr("src")
			.read()
			.replace("?resize=165,225", "");
	} else if !data_lazy.starts_with("data") && !data_lazy.is_empty() {
		image = node
			.select("img")
			.first()
			.attr("data-lazy-src")
			.read()
			.replace("?resize=165,225", "");
	} else if !data_src.starts_with("data") && !data_src.is_empty() {
		image = node
			.select("img")
			.first()
			.attr("data-src")
			.read()
			.replace("?resize=165,225", "");
	}
	let img_split = image.split('/').collect::<Vec<&str>>();
	let last_encoded = img_url_encode(String::from(img_split[img_split.len() - 1]));
	let mut encoded_img = String::new();

	(0..img_split.len() - 1).for_each(|i| {
		encoded_img.push_str(img_split[i]);
		encoded_img.push('/');
	});
	encoded_img.push_str(&last_encoded);
	append_protocol(encoded_img)
}

pub fn append_protocol(url: String) -> String {
	if url.starts_with("https") || url.starts_with("http") {
		url
	} else {
		format!("{}{}", "https:", url)
	}
}

pub fn urlencode<T: AsRef<[u8]>>(url: T) -> String {
	let bytes = url.as_ref();
	let hex = "0123456789ABCDEF".as_bytes();

	let mut result: Vec<u8> = Vec::with_capacity(bytes.len() * 3);

	for byte in bytes {
		let curr = *byte;
		if curr.is_ascii_alphanumeric() || b";,/?:@&=+$-_.!~*'()#".contains(&curr) {
			result.push(curr);
		} else {
			result.push(b'%');
			result.push(hex[curr as usize >> 4]);
			result.push(hex[curr as usize & 15]);
		}
	}
	String::from_utf8(result).unwrap_or_default()
}

/// This function is used to get the permanent url of a manga or chapter
///
/// This is done by removing the random number near the end of the url
///
/// This will work for most if not all sources that use randomized url's for the
/// `manga url`, but for the `chapter url` it will only work for some sources
pub fn get_permanet_url(original_url: String) -> String {
	let mut original_url = original_url;

	// remove trailing slash
	if original_url.ends_with('/') {
		original_url.pop();
	};

	// get the leading garbage from end of url
	// example https://luminousscans.com/series/1671729411-a-bad-person/
	// will return 1671729411, this random number is completely useless and
	// only exists to stop scrapers
	let garbage = original_url
		.split('/')
		.last()
		.expect("Failed to split url by /")
		.split('-')
		.next()
		.expect("Failed to split url by -");

	// check if the garbage is a 10 digit number to prevent removing the wrong part
	// of the url the garbage should always be a 10 digit number
	if garbage.parse::<u32>().is_ok() && garbage.len() == 10 {
		// remove the garbage from the url
		// example https://luminousscans.com/series/1671729411-a-bad-person/
		// will return https://luminousscans.com/series/a-bad-person
		original_url.replace(&format!("{}{}", garbage, "-"), "")
	} else {
		original_url
	}
}

/// This function is used to get the id from a url
///
/// The id is the last part of the url
pub fn get_id_from_url(url: String) -> String {
	let mut url = url;

	// remove trailing slash
	if url.ends_with('/') {
		url.pop();
	};

	// this will get the last part of the url
	// example https://flamescans.org/series/the-world-after-the-fall
	// will return the-world-after-the-fall
	// example https://flamescans.org/the-world-after-the-fall-chapter-55
	// will return the-world-after-the-fall-chapter-55
	let id = url.split('/').last().expect("Failed to parse id from url");

	String::from(id)
}

pub fn get_lang_code() -> String {
	let mut code = String::new();
	if let Ok(languages) = defaults_get("languages") {
		if let Ok(arr) = languages.as_array() {
			if let Ok(language) = arr.get(0).as_string() {
				code = language.read();
			}
		}
	}
	code
}
