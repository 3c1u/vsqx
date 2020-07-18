//! V3形式のvsqxシリアライザー

//
// TODO: マクロで生やしたい

use super::*;
use crate::write_xml::*;
use crate::Result;

use quick_xml::Writer;

impl WriteXml for Vsqx3 {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.vender.cdata_tag(writer, b"vender")?;
        self.version.cdata_tag(writer, b"version")?;

        self.voice_table.tagged(writer, b"vVoiceTable")?;
        self.mixer.tagged(writer, b"mixer")?;
        self.master_track.tagged(writer, b"masterTrack")?;

        for t in &self.vs_track {
            t.tagged(writer, b"vsTrack")?;
        }

        self.se_track.tagged(writer, b"seTrack")?;
        self.karaoke_track.tagged(writer, b"karaokeTrack")?;

        for a in &self.aux {
            a.tagged(writer, b"aux")?;
        }

        Ok(())
    }

    fn props(&self) -> Vec<(&str, &str)> {
        vec![
            ("xmlns", &self.xmlns),
            ("xmlns:xsi", &self.xmlns_xsi),
            ("xsi:schemaLocation", &self.xsi_schema_location),
        ]
    }
}

impl WriteXml for VoiceTable {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        for v in &self.voices {
            v.tagged(writer, b"vVoice")?;
        }

        Ok(())
    }
}

impl WriteXml for Voice {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.bs.tagged(writer, b"vBS")?;
        self.pc.tagged(writer, b"vPC")?;
        self.id.cdata_tag(writer, b"compID")?;
        self.name.cdata_tag(writer, b"vVoiceName")?;

        self.parameters.tagged(writer, b"vVoiceParam")?;

        Ok(())
    }
}

impl WriteXml for VoiceParameters {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.breathiness.tagged(writer, b"bre")?;
        self.brightness.tagged(writer, b"bri")?;
        self.clearness.tagged(writer, b"cle")?;
        self.gender.tagged(writer, b"gen")?;
        self.openness.tagged(writer, b"ope")?;

        Ok(())
    }
}

impl WriteXml for Mixer {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.master_unit.tagged(writer, b"masterUnit")?;

        for u in &self.vs_unit {
            u.tagged(writer, b"vsUnit")?;
        }

        for u in &self.se_unit {
            u.tagged(writer, b"seUnit")?;
        }

        for u in &self.karaoke_unit {
            u.tagged(writer, b"karaokeUnit")?;
        }

        Ok(())
    }
}

impl WriteXml for MasterUnit {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.output_device.tagged(writer, b"outDev")?;
        self.return_level.tagged(writer, b"retLevel")?;
        self.volume.tagged(writer, b"vol")?;

        Ok(())
    }
}

impl WriteXml for VsUnit {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.track_no.tagged(writer, b"vsTrackNo")?;
        self.input_gain.tagged(writer, b"inGain")?;
        self.send_level.tagged(writer, b"sendLevel")?;
        self.is_send_enabled.tagged(writer, b"sendEnable")?;
        self.mute.tagged(writer, b"mute")?;
        self.solo.tagged(writer, b"solo")?;
        self.pan.tagged(writer, b"pan")?;
        self.volume.tagged(writer, b"vol")?;

        Ok(())
    }
}

impl WriteXml for SeUnit {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.input_gain.tagged(writer, b"inGain")?;
        self.send_level.tagged(writer, b"sendLevel")?;
        self.is_send_enabled.tagged(writer, b"sendEnable")?;
        self.mute.tagged(writer, b"mute")?;
        self.solo.tagged(writer, b"solo")?;
        self.pan.tagged(writer, b"pan")?;
        self.volume.tagged(writer, b"vol")?;

        Ok(())
    }
}

impl WriteXml for KaraokeUnit {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.input_gain.tagged(writer, b"inGain")?;
        self.mute.tagged(writer, b"mute")?;
        self.solo.tagged(writer, b"solo")?;
        self.volume.tagged(writer, b"vol")?;

        Ok(())
    }
}

impl WriteXml for MasterTrack {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.name.cdata_tag(writer, b"seqName")?;
        self.comment.cdata_tag(writer, b"comment")?;
        self.resolution.tagged(writer, b"resolution")?;
        self.pre_measure.tagged(writer, b"preMeasure")?;

        for t in &self.time_signatures {
            t.tagged(writer, b"timeSig")?;
        }

        for t in &self.tempos {
            t.tagged(writer, b"tempo")?;
        }

        Ok(())
    }
}

impl WriteXml for TimeSignature {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.position.tagged(writer, b"posMes")?;
        self.numerator.tagged(writer, b"nume")?;
        self.denominator.tagged(writer, b"denomi")?;

        Ok(())
    }
}

impl WriteXml for Tempo {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.position.tagged(writer, b"posTick")?;
        self.value.tagged(writer, b"bpm")?;

        Ok(())
    }
}

impl WriteXml for VsTrack {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.track_no.tagged(writer, b"vsTrackNo")?;
        self.name.cdata_tag(writer, b"trackName")?;
        self.comment.cdata_tag(writer, b"comment")?;
        for p in &self.parts {
            p.tagged(writer, b"musicalPart")?;
        }

        Ok(())
    }
}

impl WriteXml for VsPart {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.position.tagged(writer, b"posTick")?;

        if let Some(play_time) = self.play_time {
            play_time.tagged(writer, b"playTime")?;
        }

        self.name
            .as_deref()
            .unwrap_or_default()
            .cdata_tag(writer, b"partName")?;
        self.comment
            .as_deref()
            .unwrap_or_default()
            .cdata_tag(writer, b"comment")?;

        self.style_plugin.tagged(writer, b"stylePlugin")?;
        self.style.tagged(writer, b"partStyle")?;

        for s in &self.singers {
            s.tagged(writer, b"singer")?;
        }

        for cc in &self.control_changes {
            cc.tagged(writer, b"cc")?;
        }

        for n in &self.notes {
            n.tagged(writer, b"note")?;
        }

        // self.plane.tagged(writer, b"plane")?;

        Ok(())
    }
}

impl WriteXml for ControlChange {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        use quick_xml::events::{BytesEnd, BytesStart, Event};
        self.pos.tagged(writer, b"p")?;

        let mut bstart = BytesStart::borrowed(b"v", "v".len());
        bstart.push_attribute((&b"id"[..], self.id.as_bytes()));
        writer.write_event(Event::Start(bstart))?;
        self.value.write_inner(writer)?;
        writer.write_event(Event::End(BytesEnd::borrowed(b"v")))?;

        Ok(())
    }
}

impl WriteXml for StylePlugin {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.id.tagged(writer, b"stylePluginID")?;
        self.name.tagged(writer, b"stylePluginName")?;
        self.version.tagged(writer, b"version")?;

        Ok(())
    }
}

impl WriteXml for Singer {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.position.tagged(writer, b"posTick")?;
        self.bs.tagged(writer, b"vBS")?;
        self.pc.tagged(writer, b"vPC")?;

        Ok(())
    }
}

impl WriteXml for Note {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.position.tagged(writer, b"posTick")?;
        self.duration.tagged(writer, b"durTick")?;
        self.note_num.tagged(writer, b"noteNum")?;
        self.velocity.tagged(writer, b"velocity")?;
        self.lyric.cdata_tag(writer, b"lyric")?;
        self.phoneme.cdata_tag(writer, b"phnms")?;
        self.style.tagged(writer, b"noteStyle")?;

        Ok(())
    }
}

impl WriteXml for Style {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        for v in &self.styles {
            v.tagged(writer, b"attr")?;
        }

        Ok(())
    }
}

impl WriteXml for StyleKey {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.value.write_inner(writer)?;
        Ok(())
    }

    fn props(&self) -> Vec<(&str, &str)> {
        vec![("id", &self.id)]
    }
}

impl WriteXml for SeTrack {
    fn write_inner<W: std::io::Write>(&self, _writer: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

impl WriteXml for KaraokeTrack {
    fn write_inner<W: std::io::Write>(&self, _writer: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

impl WriteXml for Aux {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.id.cdata_tag(writer, b"auxID")?;
        self.content.cdata_tag(writer, b"content")?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_vsqx3_serialize() {
    use quick_xml::Writer;
    use std::io::Cursor;

    let vsqx3 = include_str!("../test/v3.vsqx");
    let v: Vsqx3 = quick_xml::de::from_str(&vsqx3).unwrap();

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    v.tagged(&mut writer, b"vsq3").unwrap();
    let res = String::from_utf8(writer.into_inner().into_inner()).unwrap();
    let v2: Vsqx3 = quick_xml::de::from_str(&res).unwrap();

    assert_eq!(v, v2);
}
