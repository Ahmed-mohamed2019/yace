/*
 * Permission is hereby granted, free of charge, to any human obtaining a copy of this software and associated documentation files
 * (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify,
 * merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit humans to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
 * OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
 * LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */
use super::*;

struct 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
    𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec<u8>,
}

impl 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
    pub const fn new() -> Self {
        𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
            𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec::<u8>::new(),
        }
    }
}

impl 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = ();
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = ();

    fn emit_u8(&mut self, value: u8) -> Result<(), ()> {
        self.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.push(value);
        Ok(())
    }
}

#[test]
fn test_add_𝔞𝔩_𝔞𝔩() {
    type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ =
        𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16<'static, 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫>;
    let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
    𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16::from(&mut raw_emitter)
        .add((𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔞𝔩, 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔞𝔩))
        .expect("Testing assembler");
    assert_eq!(&[0, 0], &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]);
}

#[test]
fn test_add_𝔞𝔩_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_16ᵇⁱᵗ_𝔟𝔵_𝔰𝔦() {
    type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ =
        𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16<'static, 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫>;
    let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
    𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16::from(&mut raw_emitter)
        .add((
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔞𝔩,
            𝔞𝔡𝔡𝔯𝔢𝔰𝔰_16ᵇⁱᵗ
                .with_base(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔟𝔵)
                .with_index(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔰𝔦)
                .with_disp(0x1234i16),
        ))
        .expect("Testing assembler");
    assert_eq!(
        &[0, 0, 1, 3, 1, 6, 0x34, 0x12],
        &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
    );
}

#[test]
fn test_add_𝔞𝔩_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ_𝔢𝔰𝔭_𝔢𝔟𝔭() {
    type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ =
        𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16<'static, 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫>;
    let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
    𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16::from(&mut raw_emitter)
        .add((
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔞𝔩,
            𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ
                .with_base(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔢𝔰𝔭)
                .with_index(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔢𝔟𝔭)
                .with_disp(0x12345678),
        ))
        .expect("Testing assembler");
    assert_eq!(
        &[0, 0, 1, 4, 1, 5, 1, 0x78, 0x56, 0x34, 0x12],
        &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
    );
}

#[test]
fn test_add_𝔞𝔩_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_64ᵇⁱᵗ_𝔢𝔰𝔭_𝔢𝔟𝔭() {
    type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ =
        𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32<'static, 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫>;
    let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
    𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32::from(&mut raw_emitter)
        .add((
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔞𝔩,
            𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ
                .with_base(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔢𝔰𝔭)
                .with_index(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔢𝔟𝔭)
                .with_disp(0x12345678),
        ))
        .expect("Testing assembler");
    assert_eq!(
        &[0, 0, 1, 4, 1, 5, 1, 0x78, 0x56, 0x34, 0x12],
        &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
    );
}
