use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub struct CodeWriter<'a> {
    filepath: &'a str,
    writer: BufWriter<File>,
}

impl<'a> CodeWriter<'a> {
    pub fn new(filepath: &str) -> CodeWriter {
        let mut f: File;
        match File::create(filepath) {
            Ok(fh) => f = fh,
            Err(e) => panic!("Could not create file path for code writer!"),
        }

        CodeWriter {
            filepath: filepath,
            writer: BufWriter::new(f),
        }
    }

    pub fn code(&mut self, code: &str) {
        self.writer.write(code.as_bytes());
        self.writer.write(b"\n");
    }

    pub fn comment(&mut self, comment: &str) {
        self.writer.write(b"// ");
        self.writer.write(comment.as_bytes());
        self.writer.write(b"\n");
    }

    pub fn doc(&mut self, comment: &str) {
        self.writer.write(b"/// ");
        self.writer.write(comment.as_bytes());
        self.writer.write(b"\n");
    }
}
