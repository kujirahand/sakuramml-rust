//! runner: メタイベント・ログ出力・SMFへの直接出力
use super::*;

/// MetaTextに書き込める文字列は127バイトまでなので、文字境界を保ったまま切り詰める
pub(super) fn trim_meta_text(txt_raw: &str) -> String {
    let mut txt = String::from("");
    let mut cnt = 0;
    for c in txt_raw.chars() {
        cnt += c.len_utf8();
        if cnt < 128 {
            txt.push(c);
            continue;
        }
        break;
    }
    txt
}

/// コメントの実行
pub(super) fn exec_comment(song: &mut Song, t: &Token) {
    // 「/// xxx」形式のコメントは、行番号付きでMetaTextに埋め込む (デバッグ用) #79
    if t.value_i == COMMENT_DEBUG {
        let body = t.value_s.clone().unwrap_or(String::from(""));
        let txt = trim_meta_text(&format!("L{}: {}", t.lineno + 1, body));
        let e = Event::meta(
            trk!(song).timepos,
            0xFF,
            1, // Meta type = Text
            txt.len() as isize,
            txt.into_bytes(),
        );
        song.add_event(e);
    }
}

/// Print命令 - 引数をログに出力する
pub(super) fn exec_print(song: &mut Song, t: &Token) {
    let args_tokens = t.children.clone().unwrap_or(vec![]);
    // println!("@@@print_args=:{:?}", args_tokens);
    let args = exec_args(song, &args_tokens);
    let mut disp: Vec<String> = vec![];
    for v in args {
        disp.push(v.to_s());
    }
    let disp_s = disp.join(" ");
    let msg = format!("[PRINT]({}) {}", t.lineno, disp_s);
    if song.debug {
        println!("{}", msg);
    }
    song.add_log(msg);
}

/// メタテキストの書き込み
pub(super) fn exec_meta_text(song: &mut Song, t: &Token) {
    let txt_raw = exec_args(song, &t.children.clone().unwrap_or(vec![]))[0].to_s();
    let txt = trim_meta_text(&txt_raw);
    let e = Event::meta(
        trk!(song).timepos,
        0xFF,
        t.value_i,
        txt.len() as isize,
        txt.into_bytes(),
    );
    song.add_event(e);
}

/// ポート番号の指定
pub(super) fn exec_port(song: &mut Song, t: &Token) {
    let port = exec_args(song, &t.children.clone().unwrap_or(vec![]))[0].to_i();
    trk!(song).port = port;
    let e = Event::meta(
        trk!(song).timepos,
        0xFF,
        0x21,
        0x01,
        vec![port as u8],
    );
    song.add_event(e);
}

/// 拍子の指定
pub(super) fn exec_time_signature(song: &mut Song, t: &Token) {
    let args = exec_args(song, &t.children.clone().unwrap_or(vec![]));
    if args.len() < 2 {
        runtime_error(song, "[TimeSignature] argument must be 2");
        return;
    }
    song.timesig_frac = value_range(2, args[0].to_i(), 64);
    song.timesig_deno = value_range(2, args[1].to_i(), 64);
    song.timesig_deno = match song.timesig_deno {
        2 => 2,
        4 => 4,
        8 => 8,
        16 => 16,
        _ => {
            runtime_error(song, "[TimeSignature] value must be 2/4/8/16,n");
            4
        }
    };
    let deno_v = match song.timesig_deno {
        2 => 1,
        4 => 2,
        8 => 3,
        16 => 4,
        _ => 2,
    };
    let e = Event::meta(
        trk!(song).timepos,
        0xFF,
        0x58,
        0x04,
        vec![song.timesig_frac as u8, deno_v as u8, 0x18, 0x08],
    );
    song.add_event(e);
}

/// SMFへバイト列を直接書き込む
pub(super) fn exec_direct_smf(song: &mut Song, t: &Token) {
    let args = exec_args(song, &t.children.clone().unwrap_or(vec![]));
    if args.len() >= 1 {
        let timepos = trk!(song).timepos;
        let args_u8 = args.iter().map(|v| v.to_i() as u8).collect();
        trk!(song).events.push(Event::direct_smf(timepos, args_u8));
    }
}

/// ノートオンを直接書き込む
pub(super) fn exec_note_on(song: &mut Song, t: &Token) {
    let args = exec_args(song, &t.children.clone().unwrap_or(vec![]));
    if args.len() >= 2 {
        let timepos = trk!(song).timepos;
        let mut args_u8: Vec<u8> = args.iter().map(|v| v.to_i() as u8).collect();
        args_u8.insert(0, 0x90 | trk!(song).channel as u8);
        trk!(song).events.push(Event::direct_smf(timepos, args_u8));
    }
}

/// ノートオフを直接書き込む
pub(super) fn exec_note_off(song: &mut Song, t: &Token) {
    let args = exec_args(song, &t.children.clone().unwrap_or(vec![]));
    if args.len() >= 2 {
        let timepos = trk!(song).timepos;
        let mut args_u8: Vec<u8> = args.iter().map(|v| v.to_i() as u8).collect();
        args_u8.insert(0, 0x80 | trk!(song).channel as u8);
        trk!(song).events.push(Event::direct_smf(timepos, args_u8));
    }
}
