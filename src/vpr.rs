//! VOCALOID5 Editorから出力される.vpr形式

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::Result;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Vpr {
    /// バージョン情報
    pub version: Version,
    /// ベンダー（YAMAHA以外あるんですかね？？）
    #[serde(default = "vpr_vender")]
    pub vender: String, // "Yamaha Corporation"
    /// タイトル
    pub title: String,
    /// マスタートラック（サンプリングレートとかテンポ情報）
    pub master_track: MasterTrack,
    /// ボイス情報（の配列）
    pub voices: Vec<Voice>,
    /// トラック（の配列）
    pub tracks: Vec<Track>,
}

impl From<super::vsqx4::Vsqx4> for Vpr {
    fn from(v: super::vsqx4::Vsqx4) -> Self {
        super::v4to5::convert_vsqx4_to_vpr(&v)
    }
}

impl From<super::vsqx3::Vsqx3> for Vpr {
    fn from(v: super::vsqx3::Vsqx3) -> Self {
        super::v4to5::convert_vsqx4_to_vpr(&v.into())
    }
}

impl Vpr {
    pub fn open<P: AsRef<std::path::Path>>(path: P) -> Result<Vpr> {
        use std::fs::File;

        Self::from_reader(File::open(path)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Vpr> {
        use std::io::Cursor;

        Self::from_reader(Cursor::new(bytes))
    }

    pub fn from_reader<R: std::io::Read + std::io::Seek>(reader: R) -> Result<Vpr> {
        use zip::ZipArchive;

        let mut z = ZipArchive::new(reader)?;
        let seq = z.by_name("Project\\sequence.json");

        let seq = if seq.is_ok() {
            seq?
        } else {
            // Rust compiler is not smart enough to know seq is not be used
            // after this. A workaround is to drop seq before calling by_name.
            drop(seq);
            z.by_name("Project/sequence.json")?
        };

        Ok(serde_json::from_reader(seq)?)
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn to_json_binary(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }

    pub fn write_vpr<W: std::io::Write + std::io::Seek>(&self, writer: W) -> Result<()> {
        use std::io::Write;
        use zip::{write::FileOptions, ZipWriter};

        let mut w = ZipWriter::new(writer);

        w.start_file("Project\\sequence.json", FileOptions::default())?;
        w.write_all(&self.to_json_binary()?)?;
        w.finish()?;

        Ok(())
    }
}

pub(crate) fn vpr_vender() -> String {
    "Yamaha Corporation".into()
}

/// バージョン情報
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub revision: u64,
}

impl Version {
    pub fn new(major: u64, minor: u64, revision: u64) -> Self {
        Self {
            major,
            minor,
            revision,
        }
    }
}

/// マスタートラック
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MasterTrack {
    /// サンプリングレート
    pub sampling_rate: u64,
    /// ループ（VOCALOID5 Editor上でつかう情報？）
    #[serde(rename = "loop")]
    pub loop_info: Loop,
    /// テンポ情報
    pub tempo: Tempo,
    /// タイムシグネチャー（拍子など）
    pub time_sig: TimeSignature,
    /// ボリューム情報
    pub volume: Volume,
}

/// ループ
#[derive(Default, Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Loop {
    /// ループの有無
    pub is_enabled: bool,
    /// ループの開始位置（単位はティック？）
    pub begin: i64,
    /// ループの終了位置
    pub end: i64,
}

/// テンポ情報
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Tempo {
    /// VOCALOID5 Editorで表示されているか否か？
    pub is_folded: bool,
    /// 不明
    pub height: f64,
    /// グローバルテンポ
    pub global: GlobalTempo,
    /// tempoイベント（の配列）。
    /// テンポはBPM * 100で与えられる？？？？？？？？？
    pub events: Vec<ControlChange>,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GlobalTempo {
    /// 不明（falseでいい？）
    pub is_enabled: bool,
    /// BPM * 100？（ティック数ではなさそう）
    pub value: u64,
}

/// タイムシグネチャー（拍子記号）
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TimeSignature {
    /// VOCALOID5 Editorで表示されているか否か？
    pub is_folded: bool,
    /// time signatureイベント（の配列）
    pub events: Vec<TimeSignatureEvent>,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct TimeSignatureEvent {
    /// 小節位置？
    pub bar: i64,
    /// 分子
    #[serde(rename = "numer")]
    pub numerator: i64,
    /// 分母
    #[serde(rename = "denom")]
    pub denominator: i64,
}

/// ボリューム
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    /// VOCALOID5 Editorで表示されているか否か？
    pub is_folded: bool,
    /// 不明
    pub height: f64,
    /// ボリュームイベント（の配列）。おそらく単位はデシベル？
    pub events: Vec<ControlChange>,
}

impl Default for Volume {
    fn default() -> Self {
        Self {
            is_folded: true,
            height: 0.0,
            events: vec![ControlChange { pos: 0, value: 0 }],
        }
    }
}

/// パンポット
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Panpot {
    /// VOCALOID5 Editorで表示されているか否か？
    pub is_folded: bool,
    /// 不明
    pub height: f64,
    /// ボリュームイベント（の配列）。おそらく単位はデシベル？
    pub events: Vec<ControlChange>,
}

impl Default for Panpot {
    fn default() -> Self {
        Self {
            is_folded: true,
            height: 0.0,
            events: vec![ControlChange { pos: 0, value: 0 }],
        }
    }
}

/// ボイス情報
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Voice {
    #[serde(rename = "compID")]
    pub comp_id: String,
    #[serde(rename = "langID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// トラック
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    /// ボカロのトラックは0？
    #[serde(rename = "type")]
    pub track_type: i64,
    /// トラック名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// トラックの色？
    pub color: i64,
    /// バス番号？
    pub bus_no: i64,
    /// VOCALOID5 Editorで表示されているか否か？
    pub is_folded: bool,
    /// Editor上の何か？
    pub height: f64,
    /// ボリュームイベント
    pub volume: Volume,
    /// パンポットイベント
    pub panpot: Panpot,
    /// ミュートされているかどうか。
    pub is_muted: bool,
    /// ソロかどうか。
    pub is_solo_mode: bool,
    /// パート（クリップ）
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parts: Vec<Part>,
}

/// パート
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    /// パート名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 位置
    pub pos: u64,
    /// 長さ
    pub duration: u64,
    /// スタイル名
    pub style_name: String,
    /// 声の設定
    pub voice: Voice,
    /// MIDIエフェクト
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub midi_effects: Vec<MidiEffect>,
    /// ノート
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<Note>,
}

/// MIDIエフェクト
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MidiEffect {
    pub id: String,
    pub is_bypassed: bool,
    pub is_folded: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<Parameter>,
}

/// パラメタ
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,
    pub value: serde_json::Value,
}

/// MIDIノート
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub lyric: String,
    pub phoneme: String,
    pub is_protected: bool,
    pub pos: i64,
    pub duration: u64,
    pub number: i64,
    pub velocity: u8,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub exp: HashMap<String, i64>,
    pub singing_skill: Option<SingingSkill>,
    pub vibrato: Vibrato,
    // TODO
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SingingSkill {
    pub duration: i64,
    pub weight: SkillWeight,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SkillWeight {
    pub pre: i64,
    pub post: i64,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Vibrato {
    #[serde(rename = "type")]
    pub vibrato_type: i64,
    pub duration: i64,
}

/// コントロールチェンジ（pos, valueで与えられるMIDIイベント）
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct ControlChange<T = i64> {
    pub pos: i64,
    pub value: T,
}

/* 以下テスト用 */

#[test]
#[cfg(test)]
/// .vprファイルを正しく開けるかテスト
fn test_vpr_open() {
    Vpr::from_bytes(include_bytes!("test/v5.vpr")).unwrap();
}

#[test]
#[cfg(test)]
/// .vpr -> .vsqx4変換テスト
fn test_vpr_to_vsqx4() {
    use super::vsqx4::Vsqx4;

    let vpr = Vpr::from_bytes(include_bytes!("test/v5.vpr")).unwrap();
    let v4 = Vsqx4::from(vpr);

    v4.write("./test_vocaloid.vsqx").unwrap();
}

#[test]
#[cfg(test)]
/// 構造体の定義に従い適切にデシリアライズ・シリアライズされているか確認する。
/// 主に型エラー（Optionか否かなど）などを弾くためにあるので、**過信しすぎないこと**。
fn test_vpr_serde() {
    use serde_json;

    // パースしてデシリアライズ
    let vpr: Vpr = serde_json::from_str(include_str!("test/vpr.json")).unwrap();

    // シリアライズする
    let vpr2 = serde_json::to_string(&vpr).unwrap();

    // デシリアライズ -> シリアライズが恒等写像になることを確認
    let vpr2: Vpr = serde_json::from_str(&vpr2).unwrap();
    assert_eq!(vpr, vpr2);
}

#[test]
#[ignore] // height属性のパースに失敗する
#[cfg(test)]
/// 構造体の定義がvprファイル**すべてをカバー**しているかの確認。
/// （`vocx2vsqx`が出力するファイルが不完全でないことを保証する）
fn test_vpr_serde_full() {
    use serde_json;

    // Vpr構造体でのパース
    let vpr: Vpr = serde_json::from_str(include_str!("test/vpr.json")).unwrap();
    let vpr = serde_json::to_string(&vpr).unwrap();

    // Jsonとしてパース
    let vpr_orig: serde_json::Value = serde_json::from_str(include_str!("test/vpr.json")).unwrap();
    let vpr_json: serde_json::Value = serde_json::from_str(&vpr).unwrap();

    // Jsonが一致することを確認
    assert_eq!(vpr_orig, vpr_json);
}
