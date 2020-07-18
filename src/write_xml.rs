use crate::Result;
use quick_xml::Writer;

pub(crate) trait WriteXml {
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()>;

    fn props(&self) -> Vec<(&str, &str)> {
        vec![]
    }

    fn tagged<W: std::io::Write>(&self, writer: &mut Writer<W>, tag_name: &[u8]) -> Result<()> {
        use quick_xml::events::{BytesEnd, BytesStart, Event};
        let mut bstart = BytesStart::borrowed(tag_name, tag_name.len());
        for (a0, a1) in self.props() {
            bstart.push_attribute((a0, a1));
        }
        writer.write_event(Event::Start(bstart))?;
        self.write_inner(writer)?;
        writer.write_event(Event::End(BytesEnd::borrowed(tag_name)))?;
        Ok(())
    }
}

impl<T> WriteXml for T
where
    T: std::fmt::Display,
{
    fn write_inner<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        use quick_xml::events::{BytesText, Event};

        let m = format!("{}", self);
        writer.write_event(Event::Text(BytesText::from_plain_str(&m)))?;

        Ok(())
    }
}

pub(crate) trait Cdata: WriteXml {
    fn cdata<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()>;

    fn cdata_tag<W: std::io::Write>(&self, writer: &mut Writer<W>, tag_name: &[u8]) -> Result<()> {
        use quick_xml::events::{BytesEnd, BytesStart, Event};
        writer.write_event(Event::Start(BytesStart::borrowed(tag_name, tag_name.len())))?;
        self.cdata(writer)?;
        writer.write_event(Event::End(BytesEnd::borrowed(tag_name)))?;

        Ok(())
    }
}

impl<T> Cdata for T
where
    T: std::fmt::Display + WriteXml,
{
    fn cdata<W: std::io::Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        use quick_xml::events::{BytesText, Event};

        let m = format!("{}", self);
        writer.write_event(Event::CData(BytesText::from_escaped_str(&m)))?;

        Ok(())
    }
}
