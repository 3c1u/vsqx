//! VOCALOID5形式からVOCALOID4形式にダウングレード

use super::vpr::Vpr;
use super::vsqx4::{self, Vsqx4};

/// .vpr形式からvsqx4への変換をここで行う。
pub(crate) fn convert_vpr_to_vsqx4(vpr: &Vpr) -> Vsqx4 {
    let mut v = Vsqx4::default();

    // テンポ情報のコピー
    v.master_track.tempos.clear();
    for t in &vpr.master_track.tempo.events {
        v.master_track.tempos.push(vsqx4::Tempo {
            position: t.pos,
            value: t.value,
        });
    }

    v.master_track.pre_measure = 0;

    // 拍子情報のコピー
    v.master_track.time_signatures.clear();
    for ts in &vpr.master_track.time_sig.events {
        v.master_track.time_signatures.push(vsqx4::TimeSignature {
            position: ts.bar,
            numerator: ts.numerator,
            denominator: ts.denominator,
        });
    }

    // ボイスライブラリ情報のコピー
    use std::collections::HashMap;

    v.voice_table.voices.clear();
    let mut comp_to_pc: HashMap<&str, i64> = HashMap::new();

    for (i, vx) in vpr.voices.iter().enumerate() {
        comp_to_pc.insert(&vx.comp_id, i as i64);

        v.voice_table.voices.push(vsqx4::Voice {
            bs: vx.lang_id.unwrap_or(0),
            pc: i as i64,
            id: vx.comp_id.clone(),
            name: vx.name.as_ref().map(Clone::clone).unwrap_or_default(),
            ..Default::default()
        })
    }

    // ステレオ・モノラルトラックのミキサー情報を付加
    v.mixer.mono_unit.push(vsqx4::MonoUnit::default());
    v.mixer.stereo_unit.push(vsqx4::StereoUnit::default());

    // ボカロトラックのコピー
    for (i, track) in vpr.tracks.iter().enumerate() {
        // ボカロトラック以外は無視
        if track.track_type != 0 {
            continue;
        }

        let mut vsqx_track = vsqx4::VsTrack::default();
        vsqx_track.track_no = i as i64;

        // TODO: 他の情報

        v.mixer.vs_unit.push(vsqx4::VsUnit {
            track_no: vsqx_track.track_no,
            ..Default::default()
        });

        // ボカロパートをすべてコピーする
        for part in &track.parts {
            let mut p = vsqx4::VsPart::default();

            // TODO: workaround
            p.position = part.pos as i64 + 7680;
            p.play_time = Some(part.duration);

            let singer_id = *comp_to_pc.get(&*part.voice.comp_id).unwrap();
            p.singers.push(vsqx4::Singer {
                position: 0,
                bs: part
                    .voice
                    .lang_id
                    .or(vpr.voices[singer_id as usize].lang_id)
                    .unwrap_or(0),
                pc: singer_id,
            });

            for note in &part.notes {
                let n = vsqx4::Note {
                    position: note.pos,
                    duration: note.duration as i64,
                    note_num: note.number,
                    velocity: note.velocity as i64,
                    lyric: note.lyric.clone(),
                    phoneme: note.phoneme.clone(),
                    ..Default::default()
                };
                p.notes.push(n);
            }

            vsqx_track.parts.push(p);
        }

        v.vs_track.push(vsqx_track);
    }

    v
}
