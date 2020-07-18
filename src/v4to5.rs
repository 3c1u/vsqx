//! VOCALOID4形式からVOCALOID5形式にアップグレード

use super::vpr::*;
use super::vsqx4::{self, Vsqx4};

fn create_master_frack(v: &Vsqx4) -> MasterTrack {
    // テンポ情報
    let tempo: Vec<ControlChange> = v
        .master_track
        .tempos
        .iter()
        .map(|t| ControlChange {
            pos: t.position,
            value: t.value,
        })
        .collect();
    let tempo = Tempo {
        is_folded: false,
        height: 0.0,
        global: GlobalTempo {
            is_enabled: false,
            value: 12000,
        },
        events: tempo,
    };

    // 拍子情報
    let time_sig: Vec<TimeSignatureEvent> = v
        .master_track
        .time_signatures
        .iter()
        .map(
            |&vsqx4::TimeSignature {
                 position,
                 numerator,
                 denominator,
             }| TimeSignatureEvent {
                bar: position,
                numerator,
                denominator,
            },
        )
        .collect();
    let time_sig = TimeSignature {
        is_folded: false,
        events: time_sig,
    };

    MasterTrack {
        sampling_rate: 44100,
        loop_info: Loop::default(),
        tempo,
        time_sig,
        volume: Volume::default(),
    }
}

fn convert_part(p: &vsqx4::VsPart, voices: &[Voice]) -> Part {
    let pos = p.position as u64;
    let duration = p.play_time.unwrap_or_default();
    let voice = {
        let mut voice = voices[p.singers[0].pc as usize].clone();
        voice.name = None;
        voice
    };

    let mut notes = vec![];

    for note in &p.notes {
        let n = Note {
            is_protected: false,
            pos: note.position,
            duration: note.duration as u64,
            number: note.note_num,
            velocity: note.velocity as u8,
            lyric: note.lyric.clone(),
            phoneme: note.phoneme.clone(),
            exp: Default::default(),
            singing_skill: Some(SingingSkill {
                // TODO: よくわかってない
                duration: 0,
                weight: SkillWeight { pre: 64, post: 64 },
            }),
            vibrato: Vibrato {
                vibrato_type: 0,
                duration: 0,
            },
        };

        notes.push(n);
    }

    Part {
        name: p.name.clone(),
        pos,
        duration,
        voice,
        notes,
        midi_effects: vec![],
        style_name: "No Effect".into(),
    }
}

fn convert_track(t: &vsqx4::VsTrack, voices: &[Voice]) -> Track {
    let parts = t.parts.iter().map(|p| convert_part(p, voices)).collect();

    Track {
        track_type: 0, // たぶんボカロ
        name: Some(t.name.clone()),
        color: 0,
        bus_no: 0,
        is_folded: true,
        height: 0.0,
        volume: Volume::default(),
        panpot: Panpot::default(),
        is_muted: false,
        is_solo_mode: false,
        parts,
    }
}

pub(crate) fn convert_vsqx4_to_vpr(v: &Vsqx4) -> Vpr {
    // マスタートラックを作成
    let master_track = create_master_frack(v);

    // ボイス情報
    let voices: Vec<Voice> = v
        .voice_table
        .voices
        .iter()
        .map(|voice| Voice {
            comp_id: voice.id.clone(),
            name: Some(voice.name.clone()),
            lang_id: None,
        })
        .collect();

    // トラックの変換
    let mut tracks: Vec<Track> = vec![];

    for tr in &v.vs_track {
        let track = convert_track(tr, &voices);
        tracks.push(track);
    }

    Vpr {
        version: Version::new(5, 0, 0),
        vender: vpr_vender(),
        title: v.master_track.name.clone(),
        master_track,
        voices,
        tracks,
    }
}
