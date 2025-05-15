#[allow(dead_code)]
#[derive(Debug)]
pub enum PrideFlag {
    Transgender,
    Lesbian,
    GayMen,
    Bisexual,
    Pansexual,
    NonBinary,
    Asexual,
    Genderfluid,
    Genderqueer,
    Agender,
    Aromantic,
    Demiboy,
    Demigirl,
    Bigender,
    Polysexual,
    Androgynous,
    Neutrois,
    Omnisexual,
    Abrosexual,
}

pub const ALL_FLAGS: [PrideFlag; 19] = [
    PrideFlag::Transgender,
    PrideFlag::Lesbian,
    PrideFlag::GayMen,
    PrideFlag::Bisexual,
    PrideFlag::Pansexual,
    PrideFlag::NonBinary,
    PrideFlag::Asexual,
    PrideFlag::Genderfluid,
    PrideFlag::Genderqueer,
    PrideFlag::Agender,
    PrideFlag::Aromantic,
    PrideFlag::Demiboy,
    PrideFlag::Demigirl,
    PrideFlag::Bigender,
    PrideFlag::Polysexual,
    PrideFlag::Androgynous,
    PrideFlag::Neutrois,
    PrideFlag::Omnisexual,
    PrideFlag::Abrosexual,
];
pub fn create_pride_flag_overlay(flag: &PrideFlag, width: u32, height: u32) -> Vec<u8> {
    let colors: Vec<[u8; 4]> = match flag {
        PrideFlag::Transgender => vec![
            [0x5B, 0xCE, 0xFA, 127],
            [0xF5, 0xA9, 0xB8, 127],
            [0xFF, 0xFF, 0xFF, 127],
            [0xF5, 0xA9, 0xB8, 127],
            [0x5B, 0xCE, 0xFA, 127],
        ],
        PrideFlag::Lesbian => vec![
            [0xD5, 0x2D, 0x00, 127],
            [0xEF, 0x76, 0x27, 127],
            [0xFF, 0x9A, 0x56, 127],
            [0xFF, 0xFF, 0xFF, 127],
            [0xD1, 0x62, 0xA4, 127],
            [0xB5, 0x56, 0x90, 127],
            [0xA3, 0x02, 0x62, 127],
        ],
        PrideFlag::GayMen => vec![
            [0x07, 0x8D, 0x70, 127],
            [0x26, 0xCE, 0xAA, 127],
            [0x98, 0xE8, 0xC1, 127],
            [0xFF, 0xFF, 0xFF, 127],
            [0x7B, 0xAD, 0xE2, 127],
            [0x50, 0x49, 0xCC, 127],
            [0x3D, 0x1A, 0x78, 127],
        ],
        PrideFlag::Bisexual => vec![
            [0xD6, 0x02, 0x70, 127],
            [0xD6, 0x02, 0x70, 127],
            [0x9B, 0x4F, 0x96, 127],
            [0x00, 0x38, 0xA8, 127],
            [0x00, 0x38, 0xA8, 127],
        ],
        PrideFlag::Pansexual => vec![
            [0xFF, 0x21, 0x8C, 127],
            [0xFF, 0xD8, 0x00, 127],
            [0x21, 0xB1, 0xFF, 127],
        ],
        PrideFlag::NonBinary => vec![
            [0xFC, 0xF4, 0x34, 127],
            [0xFF, 0xFF, 0xFF, 127],
            [0x9C, 0x59, 0xD1, 127],
            [0x00, 0x00, 0x00, 127],
        ],
        PrideFlag::Asexual => vec![
            [0x00, 0x00, 0x00, 127],
            [0xA4, 0xA4, 0xA4, 127],
            [0xFF, 0xFF, 0xFF, 127],
            [0x80, 0x00, 0x80, 127],
        ],
        PrideFlag::Genderfluid => vec![
            [0xFF, 0x76, 0xA4, 127],
            [0xFF, 0xFF, 0xFF, 127],
            [0xC0, 0x11, 0xD7, 127],
            [0x00, 0x00, 0x00, 127],
            [0x2F, 0x3C, 0xBE, 127],
        ],
        PrideFlag::Genderqueer => vec![
            [0xB5, 0x7E, 0xDC, 127],
            [0xFF, 0xFF, 0xFF, 127],
            [0x4A, 0x81, 0x23, 127],
        ],
        PrideFlag::Agender => vec![
            [0x00, 0x00, 0x00, 127],
            [0xBC, 0xC4, 0xC7, 127],
            [0xFF, 0xFF, 0xFF, 127],
            [0xB8, 0xF6, 0x84, 127],
            [0xFF, 0xFF, 0xFF, 127],
            [0xBC, 0xC4, 0xC7, 127],
            [0x00, 0x00, 0x00, 127],
        ],
        PrideFlag::Aromantic => vec![
            [0x3D, 0xA5, 0x42, 127],
            [0xA7, 0xD3, 0x79, 127],
            [0xFF, 0xFF, 0xFF, 127],
            [0xA9, 0xA9, 0xA9, 127],
            [0x00, 0x00, 0x00, 127],
        ],
        PrideFlag::Demiboy => vec![
            [0x7F, 0x7F, 0x7F, 127],
            [0xC4, 0xC4, 0xC4, 127],
            [0x9D, 0xD7, 0xEA, 127],
            [0xFF, 0xFF, 0xFF, 127],
            [0x9D, 0xD7, 0xEA, 127],
            [0xC4, 0xC4, 0xC4, 127],
            [0x7F, 0x7F, 0x7F, 127],
        ],
        PrideFlag::Demigirl => vec![
            [0x7F, 0x7F, 0x7F, 127],
            [0xC4, 0xC4, 0xC4, 127],
            [0xFD, 0xAD, 0xC8, 127],
            [0xFF, 0xFF, 0xFF, 127],
            [0xFD, 0xAD, 0xC8, 127],
            [0xC4, 0xC4, 0xC4, 127],
            [0x7F, 0x7F, 0x7F, 127],
        ],
        PrideFlag::Bigender => vec![
            [0xCA, 0x79, 0xA2, 127],
            [0xED, 0xA5, 0xCD, 127],
            [0xD6, 0xC7, 0xE8, 127],
            [0xFF, 0xFF, 0xFF, 127],
            [0x9A, 0xC7, 0xE8, 127],
            [0x6D, 0x82, 0xD1, 127],
        ],
        PrideFlag::Polysexual => vec![
            [0xF7, 0x14, 0xBA, 127],
            [0x01, 0xD6, 0x6A, 127],
            [0x15, 0x94, 0xF6, 127],
        ],
        PrideFlag::Androgynous => vec![
            [0xFE, 0x00, 0x7F, 127],
            [0x98, 0x32, 0xFF, 127],
            [0x00, 0xB8, 0xE7, 127],
        ],
        PrideFlag::Neutrois => vec![
            [0xFF, 0xFF, 0xFF, 127],
            [0xC0, 0xFF, 0xC0, 127],
            [0x20, 0x20, 0x20, 127],
        ],
        PrideFlag::Omnisexual => vec![
            [0xFE, 0x9A, 0xCE, 127],
            [0xFF, 0x53, 0xBF, 127],
            [0x20, 0x00, 0x44, 127],
            [0x67, 0x60, 0xFE, 127],
            [0x8E, 0xA6, 0xFF, 127],
        ],
        PrideFlag::Abrosexual => vec![
            [0x75, 0xCA, 0x91, 127],
            [0xB3, 0xE4, 0xC7, 127],
            [0xFF, 0xFF, 0xFF, 127],
            [0xE6, 0x95, 0xB5, 127],
            [0xD9, 0x44, 0x6C, 127],
        ],
    };

    create_flag_overlay(width, height, colors)
}


pub fn create_flag_overlay(width: u32, height: u32, colors: Vec<[u8; 4]>) -> Vec<u8> {
    let mut data = Vec::with_capacity((width * height * 4) as usize);
    let stripe_height = height / colors.len() as u32;

    for y in 0..height {
        let stripe_index = (y / stripe_height) as usize;
        let color = colors.get(stripe_index).unwrap_or_else(|| colors.last().unwrap());
        for _ in 0..width {
            data.extend_from_slice(color);
        }
    }

    data
}

