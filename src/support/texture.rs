// Pixmap consists of two chunks: name and data
// TODO: use shared_data_t for pixmap contents to avoid copying.
#[derive(Default)]
pub struct PixelMap
{
    name: String,
    w: u16, // Actual texture w & h
    h: u16,
    use_w: u16, // and how much of that is used for useful data
    use_h: u16,
    // uint8_t what1;
    // uint16_t what2;
    units: u32,
    unit_bytes: u32,
    // uint8_t* data;
}

#[derive(Default)]
pub struct Texture {

}

impl PixelMap {

}

impl Texture {
    pub fn load() -> Texture {
        Texture {}
    }
}
