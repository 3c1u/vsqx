//! VOCALOID4 Editorから出力される.vsqx形式

use crate::Result;
use serde::Deserialize;

pub mod serializer;

/// VOCALOID 4用のVsqx構造体。
///
/// vsqx::Vsqx4とバイナリ互換であることを前提にしている。
/// 詳しくはvsqx3::Vsqx3の説明を見るように。
#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", rename = "vsq4")]
pub struct Vsqx4 {
    /* XML関連タグ */
    #[serde(rename = "xmlns", default = "_vsqx4_default_xmlns")]
    xmlns: String,
    #[serde(rename = "xmlns:xsi", default = "_vsqx4_default_xmlns_xsi")]
    xmlns_xsi: String,
    #[serde(
        rename = "xsi:schemaLocation",
        default = "_vsqx4_default_xsi_schema_location"
    )]
    xsi_schema_location: String,
    /// バージョン情報
    #[serde(default = "_vsqx4_default_version")]
    pub version: String,
    #[serde(default = "_vsqx4_default_vendor")]
    pub vender: String,
    #[serde(default, rename = "vVoiceTable")]
    pub voice_table: VoiceTable,
    pub mixer: Mixer,
    pub master_track: MasterTrack,
    pub vs_track: Vec<VsTrack>,
    pub mono_track: MonoTrack,
    #[serde(default, rename = "stTrack")]
    pub stereo_track: StereoTrack,
    #[serde(default)]
    pub aux: Vec<Aux>,
}

impl Vsqx4 {
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
        <Self as WriteXml>::tagged(self, &mut writer, b"vsq4")?;

        Ok(())
    }

    pub fn to_string(&self) -> Result<String> {
        use super::write_xml::WriteXml;
        use quick_xml::Writer;
        use std::io::Cursor;

        let mut writer = Writer::new(Cursor::new(Vec::new()));
        writer.write(br#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#)?;
        writer.write(b"\n")?;
        <Self as WriteXml>::tagged(self, &mut writer, b"vsq4")?;

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

impl Default for Vsqx4 {
    fn default() -> Self {
        Self {
            xmlns: _vsqx4_default_xmlns(),
            xmlns_xsi: _vsqx4_default_xmlns_xsi(),
            xsi_schema_location: _vsqx4_default_xsi_schema_location(),
            version: _vsqx4_default_version(),
            vender: _vsqx4_default_vendor(),
            voice_table: VoiceTable::default(),
            mixer: Mixer::default(),
            master_track: MasterTrack::default(),
            vs_track: Vec::default(),
            mono_track: MonoTrack::default(),
            stereo_track: StereoTrack::default(),
            aux: vec![],
        }
    }
}

impl From<super::vsqx3::Vsqx3> for Vsqx4 {
    fn from(v3: super::vsqx3::Vsqx3) -> Self {
        assert_eq!(
            std::mem::size_of::<super::vsqx3::Vsqx3>(),
            std::mem::size_of::<Self>()
        );
        let mut this: Self = unsafe { std::mem::transmute(v3) };

        this.xmlns = _vsqx4_default_xmlns();
        this.xmlns_xsi = _vsqx4_default_xmlns_xsi();
        this.version = _vsqx4_default_version();
        this.xsi_schema_location = _vsqx4_default_xsi_schema_location();

        this
    }
}

impl From<super::vpr::Vpr> for Vsqx4 {
    fn from(vpr: super::vpr::Vpr) -> Self {
        super::v5to4::convert_vpr_to_vsqx4(&vpr)
    }
}

use std::str::FromStr;

impl FromStr for Vsqx4 {
    type Err = crate::Error;

    fn from_str(string: &str) -> Result<Self> {
        Ok(quick_xml::de::from_str(string)?)
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename = "vVoiceTable")]
pub struct VoiceTable {
    #[serde(rename = "vVoice")]
    pub voices: Vec<Voice>,
}

impl Default for VoiceTable {
    fn default() -> Self {
        VoiceTable {
            voices: vec![Voice::default()],
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Voice {
    pub(crate) bs: i64,
    pub(crate) pc: i64,
    pub(crate) id: String,
    pub(crate) name: String,
    #[serde(rename = "vPrm")]
    pub(crate) parameters: VoiceParameters,
}

impl Default for Voice {
    fn default() -> Self {
        Voice {
            // おそらく言語？
            bs: 0,
            pc: 0,
            id: "BHHN4EF9BRWTNHAB".into(),
            name: "Miku(V2)".into(),
            parameters: VoiceParameters::default(),
        }
    }
}

#[derive(Clone, Default, Deserialize, Debug, PartialEq)]
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

#[derive(Clone, Default, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", rename = "mixer")]
pub struct Mixer {
    pub master_unit: MasterUnit,
    pub vs_unit: Vec<VsUnit>,
    pub mono_unit: Vec<MonoUnit>,
    #[serde(rename = "stUnit")]
    pub stereo_unit: Vec<StereoUnit>,
}

#[derive(Clone, Default, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", rename = "masterUnit")]
pub struct MasterUnit {
    #[serde(rename = "oDev")]
    output_device: i64,
    #[serde(rename = "rLvl")]
    return_level: i64,
    #[serde(rename = "vol")]
    volume: i64,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", rename = "vsUnit")]
pub struct VsUnit {
    #[serde(rename = "tNo")]
    pub track_no: i64,
    #[serde(rename = "iGin")]
    pub input_gain: i64,
    #[serde(rename = "sLvl")]
    pub send_level: i64,
    #[serde(rename = "sEnable")]
    pub is_send_enabled: i64,
    #[serde(rename = "m")]
    pub mute: i64,
    #[serde(rename = "s")]
    pub solo: i64,
    #[serde(rename = "pan")]
    pub pan: i64,
    #[serde(rename = "vol")]
    pub volume: i64,
}

impl Default for VsUnit {
    fn default() -> Self {
        Self {
            track_no: 0,
            input_gain: 0,
            send_level: -898,
            is_send_enabled: 0,
            mute: 0,
            solo: 0,
            pan: 64,
            volume: 0,
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", rename = "monoUnit")]
pub struct MonoUnit {
    #[serde(rename = "iGin")]
    input_gain: i64,
    #[serde(rename = "sLvl")]
    send_level: i64,
    #[serde(rename = "sEnable")]
    is_send_enabled: i64,
    #[serde(rename = "m")]
    mute: i64,
    #[serde(rename = "s")]
    solo: i64,
    #[serde(rename = "pan")]
    pan: i64,
    #[serde(rename = "vol")]
    volume: i64,
}

impl Default for MonoUnit {
    fn default() -> Self {
        Self {
            input_gain: 0,
            send_level: -898,
            is_send_enabled: 0,
            mute: 0,
            solo: 0,
            pan: 64,
            volume: 0,
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", rename = "stUnit")]
pub struct StereoUnit {
    #[serde(rename = "iGin")]
    input_gain: i64,
    #[serde(rename = "m")]
    mute: i64,
    #[serde(rename = "s")]
    solo: i64,
    #[serde(rename = "vol")]
    volume: i64,
}

impl Default for StereoUnit {
    fn default() -> StereoUnit {
        Self {
            input_gain: 0,
            mute: 0,
            solo: 0,
            volume: 0,
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", rename = "masterTrack")]
pub struct MasterTrack {
    #[serde(rename = "seqName")]
    pub name: String,
    #[serde(rename = "seqName", default, skip_serializing_if = "String::is_empty")]
    pub comment: String,
    pub resolution: i64,
    pub pre_measure: i64,
    #[serde(rename = "timeSig")]
    pub time_signatures: Vec<TimeSignature>,
    #[serde(rename = "tempo")]
    pub tempos: Vec<Tempo>,
}

impl Default for MasterTrack {
    fn default() -> Self {
        Self {
            name: "Untitled".into(),
            comment: "".into(),
            resolution: 480,
            pre_measure: 0,
            time_signatures: vec![TimeSignature::default()],
            tempos: vec![Tempo::default()],
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", rename = "timeSig")]
pub struct TimeSignature {
    #[serde(rename = "m")]
    pub position: i64,
    #[serde(rename = "nu")]
    pub numerator: i64,
    #[serde(rename = "de")]
    pub denominator: i64,
}

impl Default for TimeSignature {
    fn default() -> Self {
        Self {
            position: 0,
            numerator: 4,
            denominator: 4,
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", rename = "tempo")]
pub struct Tempo {
    #[serde(rename = "t")]
    pub position: i64,
    #[serde(rename = "v")]
    pub value: i64,
}

impl Default for Tempo {
    fn default() -> Self {
        Self {
            position: 0,
            value: 12000,
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", rename = "vsTrack")]
pub struct VsTrack {
    #[serde(rename = "tNo")]
    pub track_no: i64,
    pub name: String,
    pub comment: String,
    #[serde(rename = "vsPart", default)]
    pub parts: Vec<VsPart>,
}

impl Default for VsTrack {
    fn default() -> Self {
        Self {
            track_no: 0,
            name: "Track".into(),
            comment: "".into(),
            parts: vec![],
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase", rename = "vsPart")]
pub struct VsPart {
    #[serde(rename = "t")]
    pub position: i64,
    #[serde(rename = "sPlug")]
    pub style_plugin: StylePlugin,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "pStyle")]
    pub style: Style,
    #[serde(rename = "singer")]
    pub singers: Vec<Singer>,
    #[serde(rename = "cc", default)]
    pub control_changes: Vec<ControlChange>,
    #[serde(rename = "note")]
    pub notes: Vec<Note>,
    pub plane: i64,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct ControlChange<T = i64> {
    pub id: String,
    pub pos: i64,
    pub value: T,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", rename = "sPlug")]
pub struct StylePlugin {
    pub id: String,
    pub name: String,
    pub version: String,
}

impl Default for StylePlugin {
    fn default() -> Self {
        Self {
            id: "ACA9C502-A04B-42b5-B2EB-5CEA36D16FCE".into(),
            name: "VOCALOID2 Compatible Style".into(),
            version: "3.0.0.1".into(),
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", rename = "singer")]
pub struct Singer {
    #[serde(rename = "t")]
    pub(crate) position: i64,
    pub(crate) bs: i64,
    pub(crate) pc: i64,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", rename = "note")]
pub struct Note {
    #[serde(rename = "t")]
    pub position: i64,
    #[serde(rename = "dur")]
    pub duration: i64,
    #[serde(rename = "n")]
    pub note_num: i64,
    #[serde(rename = "v")]
    pub velocity: i64,
    #[serde(rename = "y")]
    pub lyric: String,
    #[serde(rename = "p")]
    pub phoneme: String,
    #[serde(rename = "nStyle")]
    pub style: Style,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            position: 0,
            duration: 0,
            note_num: 81,
            velocity: 64,
            lyric: "あ".into(),
            phoneme: "a".into(),
            style: Style::default(),
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", rename = "nStyle")]
pub struct Style {
    #[serde(rename = "v")]
    pub styles: Vec<StyleKey>,
}

macro_rules! style {
    {} => {
        Style { styles: vec![] }
    };
    { $(<v id=$a: literal>$b: literal</v>)+ } => {
        Style {
            styles: vec! [
                $(StyleKey {
                    id: ($a).into(),
                    value: $b
                }),+
            ]
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        style! {
            <v id="accent">50</v>
            <v id="bendDep">8</v>
            <v id="bendLen">0</v>
            <v id="decay">50</v>
            <v id="fallPort">0</v>
            <v id="opening">127</v>
            <v id="risePort">0</v>
            <v id="vibLen">0</v>
            <v id="vibType">0</v>
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", rename = "v")]
pub struct StyleKey {
    pub id: String,
    #[serde(rename = "$value")]
    pub value: i64,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase", rename = "monoTrack")]
pub struct MonoTrack {}

#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase", rename = "stTrack")]
pub struct StereoTrack {}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", rename = "aux")]
pub struct Aux {
    id: String,
    content: String,
}

/* デフォルト値 */

fn _vsqx4_default_xmlns() -> String {
    "http://www.yamaha.co.jp/vocaloid/schema/vsq4/".into()
}

fn _vsqx4_default_xmlns_xsi() -> String {
    "http://www.w3.org/2001/XMLSchema-instance".into()
}

fn _vsqx4_default_xsi_schema_location() -> String {
    "http://www.yamaha.co.jp/vocaloid/schema/vsq4/ vsq4.xsd".into()
}

fn _vsqx4_default_version() -> String {
    "4.0.0.3".into()
}

fn _vsqx4_default_vendor() -> String {
    "Yamaha corporation".into()
}

#[test]
#[cfg(test)]
fn test_vsqx4_parse() {
    let vsqx4 = include_str!("../test/v4.vsqx");
    let _v: Vsqx4 = quick_xml::de::from_str(&vsqx4).unwrap();
}
