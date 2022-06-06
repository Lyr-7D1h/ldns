use core::fmt;

pub struct LowercaseFormatter<'a, 'b>(pub &'a mut fmt::Formatter<'b>);
impl<'a, 'b> fmt::Write for LowercaseFormatter<'a, 'b> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for ch in s.chars() {
            self.0.write_fmt(format_args!("{}", ch.to_lowercase()))?;
        }

        Ok(())
    }
}
pub struct UpperCaseFormatter<'a, 'b>(pub &'a mut fmt::Formatter<'b>);
impl<'a, 'b> fmt::Write for UpperCaseFormatter<'a, 'b> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for ch in s.chars() {
            self.0.write_fmt(format_args!("{}", ch.to_uppercase()))?;
        }

        Ok(())
    }
}
