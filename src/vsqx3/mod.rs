//! VOCALOID3 Editorから出力される.vsqx形式

use crate::Result;
use serde::Deserialize;

pub mod serializer;

/// VOCALOID 3用のVsqx構造体。
///
/// ## Caveats
/// `vsqx::Vsqx4`とバイナリ互換であることを前提にしているので、ここを変更する場合は`Vsqx4`の変更も必要。
/// もし何かが足りない場合には、
/// ```ignore
/// //...
/// #[serde(skip_serializing_if = "Option::is_none")]
/// pub hoge: Option<i64>,
/// //...
/// ```
/// のように、
/// * Optionで包む
/// * 空の場合のデシリアライズを行わない
///
/// ようにすればいい。
///
/// また、特定のバージョンで必要な情報を変更するときには、
/// `From`トレイトでサニタイズを行うように。
#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", rename = "vsqx4")]
pub struct Vsqx3 {
    /* XML関連タグ */
    #[serde(rename = "xmlns", default = "_vsqx3_default_xmlns")]
    xmlns: String,
    #[serde(rename = "xmlns:xsi", default = "_vsqx3_default_xmlns_xsi")]
    xmlns_xsi: String,
    #[serde(
        rename = "xsi:schemaLocation",
        default = "_vsqx3_default_xsi_schema_location"
    )]
    xsi_schema_location: String,
    /// バージョン情報
    #[serde(default = "_vsqx3_default_version")]
    pub version: String,
    #[serde(default = "_vsqx3_default_vendor")]
    pub vender: String,
    #[serde(default, rename = "vVoiceTable")]
    pub voice_table: VoiceTable,
    pub mixer: Mixer,
    pub master_track: MasterTrack,
    pub vs_track: Vec<VsTrack>,
    pub se_track: SeTrack,
    pub karaoke_track: KaraokeTrack,
    #[serde(default)]
    pub aux: Vec<Aux>,
}

impl Vsqx3 {
    pub fn write<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        use super::write_xml::WriteXml;
        use quick_xml::Writer;
        use std::fs::File;
        use std::io::BufWriter;

        let writer = File::create(path)?;
        let writer = BufWriter::new(writer);
        let mut writer = Writer::new(writer);
        writer.write(br#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#)?;
        writer.write(b"\n")?;
        <Self as WriteXml>::tagged(self, &mut writer, b"vsq3")?;

        Ok(())
    }

    pub fn to_string(&self) -> Result<String> {
        use super::write_xml::WriteXml;
        use quick_xml::Writer;
        use std::io::Cursor;

        let mut writer = Writer::new(Cursor::new(Vec::new()));
        writer.write(br#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#)?;
        writer.write(b"\n")?;
        <Self as WriteXml>::tagged(self, &mut writer, b"vsq3")?;

        let res = writer.into_inner().into_inner();

        Ok(String::from_utf8_lossy(&res).into())
    }

    pub fn open<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        use std::fs::File;
        use std::io::BufReader;
        let file = File::open(path)?;
        let file = BufReader::new(file);

        Ok(quick_xml::de::from_reader(file)?)
    }
}

use std::str::FromStr;

impl FromStr for Vsqx3 {
    type Err = crate::Error;

    fn from_str(string: &str) -> Result<Self> {
        Ok(quick_xml::de::from_str(string)?)
    }
}

impl From<super::vsqx4::Vsqx4> for Vsqx3 {
    // ここでVsqxのサニタイズを行うこと。
    fn from(v4: super::vsqx4::Vsqx4) -> Self {
        assert_eq!(
            std::mem::size_of::<super::vsqx4::Vsqx4>(),
            std::mem::size_of::<Self>()
        );

        // バイナリ互換性を前提にしている
        let mut this: Self = unsafe { std::mem::transmute(v4) };

        this.xmlns = _vsqx3_default_xmlns();
        this.xmlns_xsi = _vsqx3_default_xmlns_xsi();
        this.version = _vsqx3_default_version();
        this.xsi_schema_location = _vsqx3_default_xsi_schema_location();

        this
    }
}

impl From<super::vpr::Vpr> for Vsqx3 {
    fn from(vpr: super::vpr::Vpr) -> Self {
        super::v5to4::convert_vpr_to_vsqx4(&vpr).into()
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
pub struct VoiceTable {
    #[serde(rename = "vVoice")]
    pub voices: Vec<Voice>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Voice {
    #[serde(rename = "vBS")]
    bs: i64,
    #[serde(rename = "vPC")]
    pc: i64,
    #[serde(rename = "compID")]
    id: String,
    #[serde(rename = "vVoiceName")]
    name: String,
    #[serde(rename = "vVoiceParam")]
    parameters: VoiceParameters,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct VoiceParameters {
    #[serde(rename = "bre")]
    pub breathiness: i64,
    #[serde(rename = "bri")]
    pub brightness: i64,
    #[serde(rename = "cle")]
    pub clearness: i64,
    #[serde(rename = "gen")]
    pub gender: i64,
    #[serde(rename = "ope")]
    pub openness: i64,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Mixer {
    pub master_unit: MasterUnit,
    pub vs_unit: Vec<VsUnit>,
    pub se_unit: Vec<SeUnit>,
    pub karaoke_unit: Vec<KaraokeUnit>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MasterUnit {
    #[serde(rename = "outDev")]
    output_device: i64,
    #[serde(rename = "retLevel")]
    return_level: i64,
    #[serde(rename = "vol")]
    volume: i64,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VsUnit {
    #[serde(rename = "vsTrackNo")]
    track_no: i64,
    #[serde(rename = "inGain")]
    input_gain: i64,
    #[serde(rename = "sendLevel")]
    send_level: i64,
    #[serde(rename = "sendEnable")]
    is_send_enabled: i64,
    #[serde(rename = "mute")]
    mute: i64,
    #[serde(rename = "solo")]
    solo: i64,
    #[serde(rename = "pan")]
    pan: i64,
    #[serde(rename = "vol")]
    volume: i64,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SeUnit {
    #[serde(rename = "inGain")]
    input_gain: i64,
    #[serde(rename = "sendLevel")]
    send_level: i64,
    #[serde(rename = "sendEnable")]
    is_send_enabled: i64,
    #[serde(rename = "mute")]
    mute: i64,
    #[serde(rename = "solo")]
    solo: i64,
    #[serde(rename = "pan")]
    pan: i64,
    #[serde(rename = "vol")]
    volume: i64,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KaraokeUnit {
    #[serde(rename = "inGain")]
    input_gain: i64,
    #[serde(rename = "mute")]
    mute: i64,
    #[serde(rename = "solo")]
    solo: i64,
    #[serde(rename = "vol")]
    volume: i64,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MasterTrack {
    #[serde(rename = "seqName")]
    pub name: String,
    pub comment: String,
    pub resolution: i64,
    pub pre_measure: i64,
    #[serde(rename = "timeSig")]
    pub time_signatures: Vec<TimeSignature>,
    #[serde(rename = "tempo")]
    pub tempos: Vec<Tempo>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct TimeSignature {
    #[serde(rename = "posMes")]
    pub position: i64,
    #[serde(rename = "nume")]
    pub numerator: i64,
    #[serde(rename = "denomi")]
    pub denominator: i64,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Tempo {
    #[serde(rename = "posTick")]
    pub position: i64,
    #[serde(rename = "bpm")]
    pub value: i64,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct VsTrack {
    #[serde(rename = "vsTrackNo")]
    pub track_no: i64,
    #[serde(rename = "trackName")]
    pub name: String,
    pub comment: String,
    #[serde(rename = "musicalPart", default)]
    pub parts: Vec<VsPart>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VsPart {
    #[serde(rename = "posTick")]
    pub position: i64,
    #[serde(rename = "stylePlugin")]
    pub style_plugin: StylePlugin,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "partStyle")]
    pub style: Style,
    #[serde(rename = "singer")]
    pub singers: Vec<Singer>,
    #[serde(rename = "cc", default)]
    pub control_changes: Vec<ControlChange>,
    #[serde(rename = "note")]
    pub notes: Vec<Note>,
    #[serde(default)]
    pub plane: i64,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct ControlChange<T = i64> {
    pub id: String,
    pub pos: i64,
    pub value: T,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct StylePlugin {
    #[serde(rename = "stylePluginID")]
    pub id: String,
    #[serde(rename = "stylePluginName")]
    pub name: String,
    pub version: String,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Singer {
    #[serde(rename = "posTick")]
    position: i64,
    #[serde(rename = "vBS")]
    bs: i64,
    #[serde(rename = "vPC")]
    pc: i64,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Note {
    #[serde(rename = "posTick")]
    pub position: i64,
    #[serde(rename = "durTick")]
    pub duration: i64,
    #[serde(rename = "noteNum")]
    pub note_num: i64,
    #[serde(rename = "velocity")]
    pub velocity: i64,
    #[serde(rename = "lyric")]
    pub lyric: String,
    #[serde(rename = "phnms")]
    pub phoneme: String,
    #[serde(rename = "noteStyle")]
    pub style: Style,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Style {
    #[serde(rename = "attr")]
    pub styles: Vec<StyleKey>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct StyleKey {
    pub id: String,
    #[serde(rename = "$value")]
    pub value: i64,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct SeTrack {}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct KaraokeTrack {}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Aux {
    #[serde(rename = "auxID")]
    id: String,
    content: String,
}

/* デフォルト値 */

fn _vsqx3_default_xmlns() -> String {
    "http://www.yamaha.co.jp/vocaloid/schema/vsq3/".into()
}

fn _vsqx3_default_xmlns_xsi() -> String {
    "http://www.w3.org/2001/XMLSchema-instance".into()
}

fn _vsqx3_default_xsi_schema_location() -> String {
    "http://www.yamaha.co.jp/vocaloid/schema/vsq3/ vsq3.xsd".into()
}

fn _vsqx3_default_version() -> String {
    "3.0.0.11".into()
}

fn _vsqx3_default_vendor() -> String {
    "Yamaha corporation".into()
}

#[test]
#[cfg(test)]
fn test_vsqx3_parse() {
    let vsqx3 = include_str!("../test/v3.vsqx");
    let _v: Vsqx3 = quick_xml::de::from_str(&vsqx3).unwrap();
}

#[test]
#[cfg(test)]
fn test_vsqx3_convert() {
    use super::vsqx4::Vsqx4;

    let vsqx4 = include_str!("../test/v4.vsqx");
    let v: Vsqx4 = quick_xml::de::from_str(&vsqx4).unwrap();
    let _v: Vsqx3 = v.into();
}
