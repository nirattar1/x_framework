// a string library implementation, using vector of chars.

pub struct XString {
    m_buff: Vec<char>
}

impl XString {
    pub fn new() -> XString {
        XString{m_buff: Vec::new()}
    }

    pub fn from(slice: &str) -> XString {
        let mut buf = Vec::new();
        for c in slice.chars() {
            buf.push(c);
        }
        return XString {m_buff: buf};
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        for &c in self.m_buff.iter() {
            ret.push(c);
        }
        return ret;
    }

    pub fn len(&self) -> usize {
        self.m_buff.len()
    }

    pub fn concat(&mut self, other: XString) {
        for &c in other.m_buff.iter() {
            self.m_buff.push(c);
        }
    }
}
