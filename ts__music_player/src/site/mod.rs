mod netease;
mod tencent;
mod xl_fm;

use rustc_serialize::json::Json;
use std::sync::mpsc::Sender;
use crate::player::Download;

#[allow(dead_code)]
pub fn play_netease(sender: Sender<Download>, keyword: String) {
    let _songs = netease::get_songs_data(keyword);
    for song in _songs {
        let obj = song.as_object().unwrap();
        let name = match obj.get("name").unwrap() {
            Json::String(s) => s,
            _ => "error name",
        };
        let id = match obj.get("id").unwrap() {
            Json::U64(s) => s,
            _ => &0,
        };
        let song_url = netease::get_song_play_url_by_id(id.to_string());
        if !song_url.is_empty() {
            let mut download = Download {
                name: name.to_string(),
                url: song_url,
            };
            let _ = sender.send(download).map_err(|e| {
                println!("{:#?}", e);
            });
        }
    }
}

#[allow(dead_code)]
pub fn play_xinlifm(sender: Sender<Download>, keyword: String) {
    let songs = xl_fm::get_songs_data(keyword);
    for song in songs {
        let obj = song.as_object().unwrap();
        let name = match obj.get("title").unwrap() {
            Json::String(s) => s,
            _ => "error name",
        };
        let id = match obj.get("id").unwrap() {
            Json::U64(s) => s,
            _ => &0,
        };
        let song_url = xl_fm::get_song_playurl_by_id(id.to_string());
        if !song_url.is_empty() {
            let download = Download {
                name: name.to_string(),
                url: song_url,
            };
            let _ = sender.send(download).map_err(|e| {
                println!("{:#?}", e);
            });
        }
    }
}