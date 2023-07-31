use super::SelectionMode;

pub struct Diagnostic;

impl SelectionMode for Diagnostic {
    fn iter<'a>(
        &'a self,
        buffer: &'a crate::buffer::Buffer,
    ) -> anyhow::Result<Box<dyn Iterator<Item = super::ByteRange> + 'a>> {
        Ok(Box::new(buffer.diagnostics().into_iter().filter_map(
            |diagnostic| {
                Some(super::ByteRange(
                    buffer.position_to_byte(diagnostic.range.start).ok()?
                        ..buffer.position_to_byte(diagnostic.range.end).ok()?,
                ))
            },
        )))
    }
}
