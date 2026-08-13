//! runner: タイ(&)の各モードの処理
use super::*;

/// タイ・スラーでつないだ範囲の、古いピッチベンドを削除する (#78)
/// PB.onNoteWave などと同時に指定されたときは、タイ・スラーを優先させる
fn remove_pitch_bend_on_tie_notes(song: &mut Song) {
    let tie_notes = &trk!(song).tie_notes;
    if tie_notes.len() == 0 {
        return;
    }
    let begin = tie_notes[0].time;
    let last = &tie_notes[tie_notes.len() - 1];
    let end = last.time + last.v2;
    trk!(song).remove_pitch_bend_in_range(begin, end);
}

/// タイ・スラーでつないだ音符に、異なる音程が含まれるか
/// (同じ音程だけならベンドは書き込まれない)
fn has_different_note_no(song: &Song) -> bool {
    let tie_notes = &trk!(song).tie_notes;
    for i in 1..tie_notes.len() {
        if tie_notes[i - 1].v1 != tie_notes[i].v1 {
            return true;
        }
    }
    false
}

/// TieMode::Port
pub(super) fn tie_mode_port(song: &mut Song) {
    // 異音程をベンドでつなぐときだけ、重なった古いピッチベンドを削除する (#78)
    if has_different_note_no(song) {
        remove_pitch_bend_on_tie_notes(song);
    }
    let mut last_note = trk!(song).tie_notes.remove(0);
    let mut tie_value = trk!(song).tie_value;
    loop {
        if trk!(song).tie_notes.len() == 0 {
            song.add_reserved_event(last_note);
            break;
        }
        let next_event = trk!(song).tie_notes.remove(0);
        // same note no
        if last_note.v1 == next_event.v1 {
            // add note length
            let time_pos = next_event.time + next_event.v2;
            last_note.v2 = time_pos - last_note.time;
            continue;
        }
        // check bend range in track
        let mut bend_range = trk!(song).bend_range;
        if bend_range <= 0 {
            // set bend range
            trk!(song).bend_range = 12;
            let timepos = if last_note.time <= 0 {
                0
            } else {
                last_note.time - 1
            };
            let bend_range_event = Event::pitch_bend_range(timepos, trk!(song).channel, 12);
            if !song.add_event(bend_range_event) {
                return;
            }
            bend_range = 12;
        }
        // calc pitch range
        // bend value range: -8192 to 8191
        let note_diff: isize = next_event.v1 - last_note.v1;
        tie_value = if tie_value == 0 {
            (song.timebase * 4) / 8
        } else {
            tie_value
        };
        let bend_from = (note_diff as f32 * (8192f32 / bend_range as f32)) as isize;
        let bend_to = 0;
        let mut last_v = 0;
        for i in 0..tie_value {
            let timepos = next_event.time - tie_value + i;
            let v = ((bend_from - bend_to) as f32 * (i as f32 / tie_value as f32)) as isize;
            if last_v == v {
                continue;
            }
            last_v = v;
            let bend_event = Event::pitch_bend(timepos, trk!(song).channel, v + 8192);
            if !song.add_event(bend_event) {
                return;
            }
        }
        last_note.v2 = next_event.time - last_note.time;
        song.add_reserved_event(last_note);
        let bend_event_end = Event::pitch_bend(next_event.time, trk!(song).channel, bend_to + 8192);
        if !song.add_event(bend_event_end) {
            return;
        }
        last_note = next_event;
    }
}

pub(super) fn tie_mode_bend(song: &mut Song) {
    // タイでつないだ範囲は、すべてベンドで表現するので、
    // 重なっている古いピッチベンドを先に削除する (#78)
    remove_pitch_bend_on_tie_notes(song);
    // first note
    let mut last_note = trk!(song).tie_notes.remove(0);
    let mut begin_note = last_note.clone();
    // set bend range
    let mut bend_range = trk!(song).bend_range;
    if bend_range <= 0 {
        trk!(song).bend_range = 12;
        let timepos = if last_note.time <= 0 {
            0
        } else {
            last_note.time - 1
        };
        let bend_range_event = Event::pitch_bend_range(timepos, trk!(song).channel, 12);
        if !song.add_event(bend_range_event) {
            return;
        }
        bend_range = 12;
    }
    // set bend 0
    let bend0 = Event::pitch_bend(last_note.time, trk!(song).channel, 8192);
    if !song.add_event(bend0) {
        return;
    }
    let mut lastpos = last_note.time + last_note.v2;
    while trk!(song).tie_notes.len() > 0 {
        let next_event = trk!(song).tie_notes.remove(0);
        lastpos = next_event.time + next_event.v2;
        // same note no
        if last_note.v1 == next_event.v1 {
            // add note length
            let time_pos = next_event.time + next_event.v2;
            last_note.v2 = time_pos - last_note.time;
            continue;
        }
        // calc pitch range
        // bend value range: -8192 to 8191
        let note_diff: isize = next_event.v1 - last_note.v1;
        let bend_event = Event::pitch_bend(
            next_event.time,
            trk!(song).channel,
            (note_diff as f32 * 8192f32 / bend_range as f32) as isize + 8192,
        );
        if !song.add_event(bend_event) {
            return;
        }
    }
    // write begin note
    begin_note.v2 = lastpos - begin_note.time;
    song.add_reserved_event(begin_note);
    // reset bend
    let bend_end = Event::pitch_bend(lastpos, trk!(song).channel, 8192);
    song.add_event(bend_end);
}

pub(super) fn tie_mode_gate(song: &mut Song) {
    let mut last_note = trk!(song).tie_notes.remove(0);
    let tie_value = trk!(song).tie_value;
    loop {
        if trk!(song).tie_notes.len() == 0 {
            song.add_reserved_event(last_note);
            break;
        }
        let next_event = trk!(song).tie_notes.remove(0);
        // same note no
        if last_note.v1 == next_event.v1 {
            // add note length
            let time_pos = next_event.time + next_event.v2;
            last_note.v2 = time_pos - last_note.time;
            continue;
        }
        // different note no
        if tie_value == 0 {
            last_note.v2 = next_event.time - last_note.time;
        } else {
            last_note.v2 = tie_value;
        }
        song.add_reserved_event(last_note);
        last_note = next_event;
    }
}

/// alpeggio mode
pub(super) fn tie_mode_alpe(song: &mut Song) {
    let last_note = &trk!(song).tie_notes[trk!(song).tie_notes.len() - 1];
    let last_pos = last_note.time + last_note.v2;
    let tie_notes = trk!(song).tie_notes.clone();
    for mut event in tie_notes.into_iter() {
        event.v2 = last_pos - event.time;
        song.add_reserved_event(event);
    }
}

pub(super) fn check_tie_notes(song: &mut Song) {
    // Tie/Slur mode (https://sakuramml.com/doc/command/11.htm)
    //
    // "Slur(type, value)"で、タイ記号"&(value)"の異音程(スラー)の動作を変更する。
    // type	typeの概略	動作	valueの意味	rangeの意味	使い方例
    // 0	グリッサンド	異音程をベンドで滑らかにつなぐ(※1)(※2)（※3）	グリッサンドの長さ	ベンドレンジを指定。省略可。	@81 l4 Slur(0,!8) c&e&g
    // 1	ベンド	異音程をベンドで表現。ギターのハンマリングに近い。(※1)	無効	無効	@25 l8 Slur(1,0) cdc c&d&c g&f&e&d
    // 2	ゲート	＆のついた音符のゲートを、valueにする	ゲートの長さ	無効	@81 l8 Slur(2,100) q50 c&d e&f g&f e&d
    // 3	アルペジオ	＆でつないだ音符の終わりまでゲートを伸ばす。そのとき、value に、ノートの最大発音音数を指定できる。	最大発音音数	無効	l16 Slur(3,100) c&e&g d&f&a
    if trk!(song).tie_notes.len() == 0 {
        return;
    }
    match trk!(song).tie_mode {
        TieMode::Port => tie_mode_port(song),
        TieMode::Bend => tie_mode_bend(song),
        TieMode::Gate => tie_mode_gate(song),
        TieMode::Alpe => tie_mode_alpe(song),
    };
}
