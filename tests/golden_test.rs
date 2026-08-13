use sakuramml::{compile, SAKURA_DEBUG_NONE};
use sha2::{Digest, Sha256};

fn assert_midi_golden(name: &str, source: &str, expected_sha256: &str) {
    let result = compile(source, SAKURA_DEBUG_NONE);
    assert!(
        result.log.is_empty(),
        "{name}: compilation produced diagnostics:\n{}",
        result.log
    );
    assert!(
        result.bin.starts_with(b"MThd"),
        "{name}: compiler did not produce a MIDI file"
    );

    let actual_sha256 = format!("{:x}", Sha256::digest(&result.bin));
    assert_eq!(
        actual_sha256,
        expected_sha256,
        "{name}: MIDI output changed ({} bytes)",
        result.bin.len()
    );
}

#[test]
fn sample_midi_is_unchanged() {
    let cases = [
        (
            "candy_of_kujirahand.mml",
            include_str!("../samples/candy_of_kujirahand.mml"),
            // #78: 先行指定の重複値を書き込まなくなったため、CC#1の重複7件分だけ変化
            "a6fe466b7f47371124dd6678893b83f977c34e3b71fc8fe1341803400d4e826b",
        ),
        (
            "sakura2.mml",
            include_str!("../samples/sakura2.mml"),
            "94ef309a7a4c2401f69f4bdfa63469be44619662134840ba0384851ce260abe1",
        ),
        (
            "seija.mml",
            include_str!("../samples/seija.mml"),
            "7ada444b1387dd7f62e3fd5c38bdf2bd87bccd05814cb5c4c8262114a2beb807",
        ),
    ];

    for (name, source, expected_sha256) in cases {
        assert_midi_golden(name, source, expected_sha256);
    }
}

#[test]
fn representative_mml_midi_is_unchanged() {
    let cases = [
        (
            "notes_and_parameters",
            "Tempo(120) o4 l8 q75 v96 c d+ e- r > f4. < g16^32",
            "8c3dd9090895b73520f9c1392ba1cb4150d796181b92c3d6f965c1346d696040",
        ),
        (
            "harmony_and_loops",
            "o4 l8 'ceg' [4 c:d] [2 [2 e:f] g]",
            "b5f191c1d038773bb3c33bb5a2136ca0058df821670b19c94d3c593f99e3bd08",
        ),
        (
            "subroutine_and_function",
            "Sub{o5 l8 ceg} o4 c Function ARP(INT N){If(N==1){e}ELSE{g}} ARP(1)",
            "8529bec1f8378121b27ba74c1f9466a52f26705675f56b23a814fb1fe7d79fb7",
        ),
        (
            "midi_controls",
            "CH(1) CC(10,64) RPN(0,1,64) NRPN(1,2,3) PitchBend(-1024) SysEx$=F0,7E,7F,09,01,F7; o4c",
            "8f6aa1bef22f275d5864027181ef93861f70784abb1daaa3e46d62333c82636e",
        ),
        (
            "tie_modes",
            "o4 l8 Slur(0,24) c&d&e Slur(1,0) f&g&a Slur(2,48) c&d&e Slur(3,3) c&e&g",
            "4e94764a54ba3eacb434b7d1192fd9ead0345babc75eb6b5b8963f80f28043b1",
        ),
        (
            "conditionals_and_loops",
            "Int N=0 If(1){c}ELSE{d} For(I=0;I<3;I++){e} While(N<2){f N++}",
            "2168e97c0a2452b7f9e6cae12542468ce74f357a67010a5e9df9203a807129e8",
        ),
        (
            "rhythm_macro",
            "$x{n60,} Rhythm{bxsh mxcb}",
            "dcdd44559786d8b96a7400dd11d0ec0ac9faeefe084048defc45fea2a563912c",
        ),
    ];

    for (name, source, expected_sha256) in cases {
        assert_midi_golden(name, source, expected_sha256);
    }
}
