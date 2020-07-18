//! V4形式のvsqxシリアライザー

// TODO: マクロで生やしたい

use super::*;
use crate::write_xml::*;
use crate::Result;

use quick_xml::Writer;

impl WriteXml for Vsqx4 {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.vender.cdata_tag(writer, b"vender")?;
        self.version.cdata_tag(writer, b"version")?;

        self.voice_table.tagged(writer, b"vVoiceTable")?;
        self.mixer.tagged(writer, b"mixer")?;
        self.master_track.tagged(writer, b"masterTrack")?;

        for t in &self.vs_track {
            t.tagged(writer, b"vsTrack")?;
        }

        self.mono_track.tagged(writer, b"monoTrack")?;
        self.stereo_track.tagged(writer, b"stTrack")?;

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
        self.bs.tagged(writer, b"bs")?;
        self.pc.tagged(writer, b"pc")?;
        self.id.cdata_tag(writer, b"id")?;
        self.name.cdata_tag(writer, b"name")?;

        self.parameters.tagged(writer, b"vPrm")?;

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

        for u in &self.mono_unit {
            u.tagged(writer, b"monoUnit")?;
        }

        for u in &self.stereo_unit {
            u.tagged(writer, b"stUnit")?;
        }

        Ok(())
    }
}

impl WriteXml for MasterUnit {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.output_device.tagged(writer, b"oDev")?;
        self.return_level.tagged(writer, b"rLvl")?;
        self.volume.tagged(writer, b"vol")?;

        Ok(())
    }
}

impl WriteXml for VsUnit {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.track_no.tagged(writer, b"tNo")?;
        self.input_gain.tagged(writer, b"iGin")?;
        self.send_level.tagged(writer, b"sLvl")?;
        self.is_send_enabled.tagged(writer, b"sEnable")?;
        self.mute.tagged(writer, b"m")?;
        self.solo.tagged(writer, b"s")?;
        self.pan.tagged(writer, b"pan")?;
        self.volume.tagged(writer, b"vol")?;

        Ok(())
    }
}

impl WriteXml for MonoUnit {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.input_gain.tagged(writer, b"iGin")?;
        self.send_level.tagged(writer, b"sLvl")?;
        self.is_send_enabled.tagged(writer, b"sEnable")?;
        self.mute.tagged(writer, b"m")?;
        self.solo.tagged(writer, b"s")?;
        self.pan.tagged(writer, b"pan")?;
        self.volume.tagged(writer, b"vol")?;

        Ok(())
    }
}

impl WriteXml for StereoUnit {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.input_gain.tagged(writer, b"iGin")?;
        self.mute.tagged(writer, b"m")?;
        self.solo.tagged(writer, b"s")?;
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
        self.position.tagged(writer, b"m")?;
        self.numerator.tagged(writer, b"nu")?;
        self.denominator.tagged(writer, b"de")?;

        Ok(())
    }
}

impl WriteXml for Tempo {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.position.tagged(writer, b"t")?;
        self.value.tagged(writer, b"v")?;

        Ok(())
    }
}

impl WriteXml for VsTrack {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.track_no.tagged(writer, b"tNo")?;
        self.name.cdata_tag(writer, b"name")?;
        self.comment.cdata_tag(writer, b"comment")?;
        for p in &self.parts {
            p.tagged(writer, b"vsPart")?;
        }

        Ok(())
    }
}

impl WriteXml for VsPart {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.position.tagged(writer, b"t")?;

        if let Some(play_time) = self.play_time {
            play_time.tagged(writer, b"playTime")?;
        }

        self.name
            .as_deref()
            .unwrap_or_default()
            .cdata_tag(writer, b"name")?;
        self.comment
            .as_deref()
            .unwrap_or_default()
            .cdata_tag(writer, b"comment")?;

        self.style_plugin.tagged(writer, b"sPlug")?;
        self.style.tagged(writer, b"pStyle")?;
        for s in &self.singers {
            s.tagged(writer, b"singer")?;
        }

        for cc in &self.control_changes {
            cc.tagged(writer, b"cc")?;
        }

        for n in &self.notes {
            n.tagged(writer, b"note")?;
        }

        self.plane.tagged(writer, b"plane")?;

        Ok(())
    }
}

impl WriteXml for ControlChange {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        use quick_xml::events::{BytesEnd, BytesStart, Event};
        self.pos.tagged(writer, b"t")?;

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
        self.id.cdata_tag(writer, b"id")?;
        self.name.cdata_tag(writer, b"name")?;
        self.version.cdata_tag(writer, b"version")?;

        Ok(())
    }
}

impl WriteXml for Singer {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.position.tagged(writer, b"t")?;
        self.bs.tagged(writer, b"bs")?;
        self.pc.tagged(writer, b"pc")?;

        Ok(())
    }
}

impl WriteXml for Note {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.position.tagged(writer, b"t")?;
        self.duration.tagged(writer, b"dur")?;
        self.note_num.tagged(writer, b"n")?;
        self.velocity.tagged(writer, b"v")?;
        self.lyric.cdata_tag(writer, b"y")?;
        self.phoneme.cdata_tag(writer, b"p")?;
        self.style.tagged(writer, b"nStyle")?;

        Ok(())
    }
}

impl WriteXml for Style {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        for v in &self.styles {
            v.tagged(writer, b"v")?;
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

impl WriteXml for MonoTrack {
    fn write_inner<W: std::io::Write>(&self, _writer: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

impl WriteXml for StereoTrack {
    fn write_inner<W: std::io::Write>(&self, _writer: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

impl WriteXml for Aux {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.id.cdata_tag(writer, b"id")?;
        self.content.cdata_tag(writer, b"content")?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_vsqx4_serialize() {
    use quick_xml::Writer;
    use std::io::Cursor;

    let vsqx4 = include_str!("../test/v4.vsqx");
    let v: Vsqx4 = quick_xml::de::from_str(&vsqx4).unwrap();

    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 4);
    v.tagged(&mut writer, b"vsq4").unwrap();
    let res = String::from_utf8(writer.into_inner().into_inner()).unwrap();
    let v2: Vsqx4 = quick_xml::de::from_str(&res).unwrap();

    assert_eq!(v, v2);
}
