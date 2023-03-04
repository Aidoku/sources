use aidoku::{helpers::uri::*, prelude::*, std::String, std::Vec};
use alloc::{borrow::ToOwned, string::ToString};

pub const BASE_URL: &str = "https://hentai2read.com";

pub fn create_advanced_search_body(
	manga_title: Option<String>,
	artist_name: Option<String>,
	status: Option<i64>,
	tag_search_mode: Option<String>,
	include_tags: Option<Vec<i64>>,
	exclude_tags: Option<Vec<i64>>,
) -> String {
	let mut form_body = format!("cmd_wpm_wgt_mng_sch_sbm=Search&txt_wpm_wgt_mng_sch_nme=&cmd_wpm_pag_mng_sch_sbm=&txt_wpm_pag_mng_sch_nme={}&txt_wpm_pag_mng_sch_ats={}&rad_wpm_pag_mng_sch_sts={}&rad_wpm_pag_mng_sch_tag_mde={}",
		encode_uri(manga_title.unwrap_or_default()),
        encode_uri(artist_name.unwrap_or_default()),
        &status.unwrap_or_default(),
        &tag_search_mode.unwrap_or_default()
	);

	if let Some(include_tags) = include_tags {
		for tag in include_tags.iter() {
			form_body.push_str(format!("&chk_wpm_pag_mng_sch_mng_tag_inc[]={}", tag).as_str());
		}
	}

	if let Some(exclude_tags) = exclude_tags {
		for tag in exclude_tags.iter() {
			form_body.push_str(format!("&chk_wpm_pag_mng_sch_mng_tag_exc[]={}", tag).as_str());
		}
	}

	form_body
}

pub fn genre_id_from_filter(str: &str) -> i64 {
	let genre_id = str.split('_').last().unwrap_or_default();
	genre_id.parse::<i64>().unwrap_or_default()
}

pub fn clean_cover_url(str: &str) -> String {
	// /cdn-cgi/image/format=auto/https://img1.hentaicdn.com/hentai/cover/_S38878.jpg?x63162
	let mut url = str.to_owned();
	url.replace_range(0..url.find("https://").unwrap_or_default(), "");
	url
}

pub fn parse_chapter_number(str: &str) -> f32 {
	let chapter_number = str.split('/').nth_back(1).unwrap_or_default();
	chapter_number.parse::<f32>().unwrap_or_default()
}

pub fn change_page(str: &str, page: i32) -> String {
	let mut url = str.to_owned();
	let page_str = url.split('/').nth_back(1).unwrap_or_default();
	url.replace_range(url.len() - page_str.len().., &page.to_string());
	url
}

pub fn get_manga_id(str: &str) -> String {
	let url = str.to_owned();

	let manga_id = url.split('/').nth_back(1).unwrap_or_default();
	manga_id.to_string()
}

pub fn between_string(s: &str, start: &str, end: &str) -> Option<String> {
	let start = s.find(start)? + start.len();
	let end = s.find(end)? - start;
	Some(s[start..start + end].to_string())
}
