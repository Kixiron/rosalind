#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Dna {
    A = 0,
    C = 1,
    G = 2,
    T = 3,
}

impl Dna {
    pub fn from_string(string: String) -> Option<Vec<Self>> {
        let mut dna = string.into_bytes();
        for byte in &mut dna {
            *byte = Self::from_byte(*byte) as u8;
        }

        let dna = {
            let (ptr, len, capacity) = dna.into_raw_parts();
            unsafe { Vec::from_raw_parts(ptr.cast::<Self>(), len, capacity) }
        };

        Some(dna)
    }

    pub fn from_str(string: &str) -> Vec<Self> {
        let mut dna = Vec::with_capacity(string.len());

        // TODO: We could do this simd-style, a single pass
        //       to validate all characters and then another
        //       to actually "parse" them
        for byte in string.bytes() {
            dna.push(Self::from_byte(byte));
        }

        dna
    }

    pub fn from_byte(byte: u8) -> Self {
        #[cold]
        #[inline(never)]
        fn invalid_dna_char(byte: u8) -> ! {
            panic!("invalid dna character: {}", byte as char)
        }

        match byte {
            b'A' => Self::A,
            b'C' => Self::C,
            b'G' => Self::G,
            b'T' => Self::T,
            invalid => invalid_dna_char(invalid),
        }
    }

    pub const fn to_char(self) -> char {
        match self {
            Self::A => 'A',
            Self::C => 'C',
            Self::G => 'G',
            Self::T => 'T',
        }
    }

    pub fn to_string(dna: &[Self]) -> String {
        let mut string = String::with_capacity(dna.len());
        for molecule in dna {
            string.push(molecule.to_char());
        }

        string
    }

    pub const fn reverse_compliment(self) -> Self {
        match self {
            Self::A => Self::T,
            Self::C => Self::G,
            Self::G => Self::C,
            Self::T => Self::A,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Rna {
    A = 0,
    C = 1,
    G = 2,
    U = 3,
}

impl Rna {
    pub fn from_dna(dna: Dna) -> Self {
        unsafe { std::mem::transmute::<Dna, Rna>(dna) }
    }

    pub fn from_dna_vec(dna: Vec<Dna>) -> Vec<Self> {
        let (ptr, len, capacity) = dna.into_raw_parts();
        unsafe { Vec::from_raw_parts(ptr.cast::<Self>(), len, capacity) }
    }

    pub fn from_dna_slice(dna: &[Dna]) -> Vec<Self> {
        Self::from_dna_vec(dna.to_vec())
    }

    pub const fn to_char(self) -> char {
        match self {
            Self::A => 'A',
            Self::C => 'C',
            Self::G => 'G',
            Self::U => 'U',
        }
    }

    pub fn to_string(rna: &[Self]) -> String {
        let mut string = String::with_capacity(rna.len());
        for molecule in rna {
            string.push(molecule.to_char());
        }

        string
    }
}
