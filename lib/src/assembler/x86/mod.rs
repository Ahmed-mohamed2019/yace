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

//! Documentation doesn't exist yet but we have some doctests.
//!
//! This code will compile:
//!
//! ```
//! # #![allow(uncommon_codepoints)]
//! # #![allow(non_camel_case_types)]
//! # use yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝘅𝟴𝟲::{𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏,
//! #                            𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ,
//! #                            𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32,
//! #                            𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32,
//! #                            𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓};
//! #
//! # struct 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec<u8>,
//! # }
//! #
//! # impl 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     pub const fn new() -> Self {
//! #         𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #             𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec::<u8>::new(),
//! #         }
//! #     }
//! # }
//! #
//! # impl 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = ();
//! #     type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = ();
//! #
//! #     fn emit_u8(&mut self, value: u8) -> Result<(), ()> {
//! #         self.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.push(value);
//! #         Ok(())
//! #     }
//! # }
//! #
//! # type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ =
//! #     𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32<'static, 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫>;
//! # let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
//! 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32::from(&mut raw_emitter)
//!     .add((
//!         𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔞𝔩,
//!         𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ
//!             .with_base(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔢𝔰𝔭)
//!             .with_index(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔯15𝔡)
//!             .with_disp(0x12345678),
//!     ))
//!     .expect("Testing assembler");
//! # assert_eq!(
//! #     &[0, 0, 1, 4, 1, 15, 1, 0x78, 0x56, 0x34, 0x12],
//! #     &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
//! # );
//! ```
//!
//! This code works fine, too:
//!
//! ```
//! # #![allow(uncommon_codepoints)]
//! # #![allow(non_camel_case_types)]
//! # use yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝘅𝟴𝟲::{𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏,
//! #                            𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ,
//! #                            𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32,
//! #                            𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32,
//! #                            𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓};
//! #
//! # struct 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec<u8>,
//! # }
//! #
//! # impl 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     pub const fn new() -> Self {
//! #         𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #             𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec::<u8>::new(),
//! #         }
//! #     }
//! # }
//! #
//! # impl 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = ();
//! #     type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = ();
//! #
//! #     fn emit_u8(&mut self, value: u8) -> Result<(), ()> {
//! #         self.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.push(value);
//! #         Ok(())
//! #     }
//! # }
//! #
//! # type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ =
//! #     𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32<'static, 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫>;
//! # let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
//! 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32::from(&mut raw_emitter)
//!     .add((
//!         𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔞𝔥,
//!         𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ
//!             .with_base(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔢𝔰𝔭)
//!             .with_index(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔢𝔟𝔭)
//!             .with_disp(0x12345678),
//!     ))
//!     .expect("Testing assembler");
//! # assert_eq!(
//! #     &[4, 0, 1, 4, 1, 5, 1, 0x78, 0x56, 0x34, 0x12],
//! #     &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
//! # );
//! ```
//!
//! But that one wouldn't compile because you can't use 𝔞𝔥 and 𝔯15𝔡 in the same instruction.
//!
//! ```compile_fail,E0277
//! # #![allow(uncommon_codepoints)]
//! # #![allow(non_camel_case_types)]
//! # use yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝘅𝟴𝟲::{𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏,
//! #                            𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ,
//! #                            𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32,
//! #                            𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32,
//! #                            𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓};
//! #
//! # struct 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec<u8>,
//! # }
//! #
//! # impl 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     pub const fn new() -> Self {
//! #         𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #             𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec::<u8>::new(),
//! #         }
//! #     }
//! # }
//! #
//! # impl 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = ();
//! #     type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = ();
//! #
//! #     fn emit_u8(&mut self, value: u8) -> Result<(), ()> {
//! #         self.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.push(value);
//! #         Ok(())
//! #     }
//! # }
//! #
//! # type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ =
//! #     𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32<'static, 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫>;
//! # let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
//! 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32::from(&mut raw_emitter)
//!     .add((
//!         𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔞𝔥,
//!         𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ
//!             .with_base(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔢𝔰𝔭)
//!             .with_index(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔯15𝔡)
//!             .with_disp(0x12345678),
//!     ))
//!     .expect("Testing assembler");
//! # assert_eq!(
//! #     &[4, 0, 1, 4, 1, 15, 1, 0x78, 0x56, 0x34, 0x12],
//! #     &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
//! # );
//! ```

use yace_codegen::𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘;

macro_rules! 𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖙𝖗𝖆𝖎𝖙 {
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        pub trait $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident {
           $($𝓮𝔁𝓽𝓻𝓪_𝓭𝓮𝓯𝓲𝓷𝓮𝓼:tt)*
        }
      ) => {
        𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! {
            𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖙𝖗𝖆𝖎𝖙! {
                $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
                pub trait $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
                where
                    Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ>,
                    Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]>,
                    Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ>,
                    Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ>,
                Ξ64[Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ>,]

                    Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ:
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ]>,
                    Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ:
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]>,
                    Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ:
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ]>,
                    Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ:
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]>,
                Ξ64[Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ:
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ>,]

                    Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ:
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ]>,
                    Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ:
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]>,
                    Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ:
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ]>,
                    Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ:
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]>,
                Ξ64[Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ:
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ>,]

                    Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ:
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ]>,
                    Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ:
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]>,
                    Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ:
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ86[    𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ86[    𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ]>,
                    Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ:
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]>,
                Ξ64[Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ:
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ>,]

                Ξ64[Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ:
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ>,]
                    Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ:
                Ξ64[    From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +]
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ]>,
                    Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ:
                Ξ64[    From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +]
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]>,
                Ξ64[Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ:
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ>,]

                Ξ64[Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ:
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ>,]
                    Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ:
                Ξ64[    From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +]
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ86[    𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ86[    𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ]>,
                    Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ:
                Ξ64[    From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +]
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]>,
                Ξ64[Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ:
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ>,]

                Ξ64[Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ:
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ>,]
                    Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ:
                Ξ64[    From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +]
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ86[    𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ86[    𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ]>,
                    Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ:
                Ξ64[    From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +]
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]>,
                Ξ64[Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ:
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ>,]

                Ξ64[Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ:
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ>,]
                    Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ:
                Ξ64[    From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +]
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ86[    𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ86[    𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ]>,
                    Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ:
                Ξ64[    From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +]
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]>,
                Ξ64[Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ:
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ>,]

                Ξ86[Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ:
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<Self, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆,

                    Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ:
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<Self, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆,]

                Ξ64[Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<Self, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<Self, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆,
                    Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ>,]
                    Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                Ξ64[    From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +]
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<Self, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +]
                        𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<Self, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆,

                Ξ64[Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<Self, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<Self, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆,
                    Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ>,
                    Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<Self, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<Self, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆,]

                    Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ86[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +]
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ]>,
                    Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ86[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +]
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]>,

                    Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                Ξ86[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +]
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]>,
                Ξ64[Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ>,
                    Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ>,]
                    Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                Ξ64[    From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +]
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                Ξ86[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +]
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ>,
                Ξ64[Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<Self, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<Self, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆,]
                    Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                Ξ64[    From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +]
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                Ξ86[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +]
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ]> +
                Ξ64[    From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +]
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +]
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<Self, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +]
                        𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<Self, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆,
                Ξ64[Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<Self, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<Self, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆,
                    Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        core::convert::TryFrom<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ> +
                        core::convert::TryFrom<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<Self, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<Self, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆,]

                    Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫:
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> +
                        𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆,

                    Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞:
                        Default +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒄𝒂𝒍𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒄𝒂𝒍𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞>,

                    Self::𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫:
                        core::convert::TryFrom<Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>,

                    Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫:
                        From<Self::𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>,

                    Self::𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫:
                        core::convert::TryFrom<Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> +
                        core::convert::TryFrom<Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>
            Χ𝔷𝔷[Ξ86[  + core::convert::TryFrom<Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>]]
                Ξ𝔷𝔷[  + core::convert::TryFrom<Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>],

                    Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫:
                        From<Self::𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> +
                        From<Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>
            Χ𝔷𝔷[Ξ86[  + From<Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>]]
                Ξ𝔷𝔷[  + From<Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>],

                    Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫:
                        From<Self::𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> +
                        From<Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>
            Χ𝔷𝔷[Ξ86[  + From<Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>]]
                Ξ𝔷𝔷[  + From<Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>],

            Χ𝔷𝔷[Ξ86[Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫:
                        core::convert::TryFrom<Self::𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>,
                    Self::𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫:
                        From<Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>,
                    Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫:
                        From<Self::𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> +
                        From<Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> +
                        From<Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>,]]
                Ξ𝔷𝔷[Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫:
                        core::convert::TryFrom<Self::𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>,
                    Self::𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫:
                        From<Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>,
                    Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫:
                        From<Self::𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> +
                        From<Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> +
                        From<Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>,]

                Ξ86[Option<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ>:
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ>,
                    Option<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ>:
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ>,]

                Ξ64[Option<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ>:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ>,]
                    Option<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ>:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ> +]
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ>,

                Ξ64[Option<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ>:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ>,
                    Option<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ>:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ> +
                        From<Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ>,

                    Option<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ>:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ>,]
                    Option<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ>:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                Ξ64[    From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> +]
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ>,

                Ξ64[Option<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ>:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ>,
                    Option<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ>:
                        From<Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        From<Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ>,]

                    Option<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>: From<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>,

                    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞:
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒄𝒂𝒍𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒄𝒂𝒍𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞>,

                Ξ86[𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ:
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ>,]

                    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ:
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::Ξ86[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ]>,

                Ξ64[𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ:
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> +
                        𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> +
                        𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<Self, 𝐭𝐚𝐫𝐠𝐞𝐭 = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ>,]

                    // Note: that part should be autogenerated from the instructions database.
                    Self:
                Ξ64[    𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ)> +]
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ])> +
                Ξ86[    𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, i16, 0>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, i16, 1>)> +]
                Ξ64[    𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 0>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 1>)> +]
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 0>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 1>)> +
                Ξ64[    𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 0>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 1>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 0>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 1>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 0>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 1>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 0>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 1>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 0>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 1>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 0>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 1>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 0>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 1>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 0>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 1>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ)> +]
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ], Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ])> +
                Ξ86[    𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, i16, 0>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, i16, 1>)> +]
                Ξ64[    𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 0>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 1>)> +]
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ], 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 0>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ], 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 1>)> +
                Ξ64[    𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 0>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 1>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 0>)> +
                        𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<(Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32, 1>)>,]

                {   type 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    type 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    type 𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    type 𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    type 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ;
                    type 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ;
                    type 𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ;
                    type 𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ;
                Ξ64[type 𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    type 𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    type 𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    type 𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;]

                    type 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    type 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    type 𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    type 𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    type 𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    type 𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    type 𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    type 𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;

                    type 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    type 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    type 𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    type 𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    type 𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    type 𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    type 𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    type 𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;

                Ξ64[type 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    type 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    type 𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    type 𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    type 𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    type 𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    type 𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    type 𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;]

                Ξ86[type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;]
                Ξ64[type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ;
                    type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ;]
                    type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                Ξ64[type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ;
                    type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ;
                    type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;]

                    type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ;
                    type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ;

                Ξ86[type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;]
                Ξ64[type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ;
                    type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ;]
                Ξ64[type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ;]
                    type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                Ξ64[type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ;]
                    type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                Ξ64[type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ;
                    type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;]

                    type 𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;

                    type 𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞;

                    type 𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    type 𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;

                    type 𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;

                    type 𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    type 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    type 𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;

                    // ₐᵥₓ512 doesn't extend register set in legacy mode and if ₐᵥₓ512 instructions are not used nothing is added
                    // to the biary.
                    // Thus we provide ₐᵥₓ512 support unconditiOnally in legacy mode, but only if it's requested in ₓ86_64 mode.
            Χ𝔷𝔷[Ξ86[type 𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    type 𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    type 𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;]]
                Ξ𝔷𝔷[type 𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    type 𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    type 𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;]

                    const 𝔞𝔩: Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    const 𝔠𝔩: Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    const 𝔡𝔩: Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    const 𝔟𝔩: Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    const 𝔞𝔥: Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ;
                    const 𝔠𝔥: Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ;
                    const 𝔡𝔥: Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ;
                    const 𝔟𝔥: Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ;
                Ξ64[const 𝔰𝔭𝔩: Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    const 𝔟𝔭𝔩: Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    const 𝔰𝔦𝔩: Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    const 𝔡𝔦𝔩: Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;

                    // Note: use of 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ and 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ means you can not easily convert, e.g., 𝔯8𝔟 into 𝔯8𝔴.
                    // This is a bit surprising.  Consider creation or 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_{8,16}ᵇⁱᵗₙₒⁱᶻ types?
                    const 𝔯8𝔟: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ;
                    const 𝔯9𝔟: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ;
                    const 𝔯10𝔟: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ;
                    const 𝔯11𝔟: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ;
                    const 𝔯12𝔟: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ;
                    const 𝔯13𝔟: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ;
                    const 𝔯14𝔟: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ;
                    const 𝔯15𝔟: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ;]

                    const 𝔞𝔵: Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    const 𝔠𝔵: Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    const 𝔡𝔵: Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    const 𝔟𝔵: Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    const 𝔰𝔭: Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    const 𝔟𝔭: Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    const 𝔰𝔦: Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    const 𝔡𝔦: Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;

                    // Note: use of 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ and 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ means you can not easily convert, e.g., 𝔯8𝔟 into 𝔯8𝔴.
                    // This is a bit surprising.  Consider creation or 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_{8,16}ᵇⁱᵗₙₒⁱᶻ types?
                Ξ64[const 𝔯8𝔴: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    const 𝔯9𝔴: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    const 𝔯10𝔴: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    const 𝔯11𝔴: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    const 𝔯12𝔴: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    const 𝔯13𝔴: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    const 𝔯14𝔴: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    const 𝔯15𝔴: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;]

                    const 𝔢𝔞𝔵: Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    const 𝔢𝔠𝔵: Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    const 𝔢𝔡𝔵: Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    const 𝔢𝔟𝔵: Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    const 𝔢𝔰𝔭: Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    const 𝔢𝔟𝔭: Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    const 𝔢𝔰𝔦: Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    const 𝔢𝔡𝔦: Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;

                Ξ64[const 𝔯8𝔡: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ;
                    const 𝔯9𝔡: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ;
                    const 𝔯10𝔡: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ;
                    const 𝔯11𝔡: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ;
                    const 𝔯12𝔡: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ;
                    const 𝔯13𝔡: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ;
                    const 𝔯14𝔡: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ;
                    const 𝔯15𝔡: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ;

                    const 𝔯𝔞𝔵: Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    const 𝔯𝔠𝔵: Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    const 𝔯𝔡𝔵: Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    const 𝔯𝔟𝔵: Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    const 𝔯𝔰𝔭: Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    const 𝔯𝔟𝔭: Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    const 𝔯𝔰𝔦: Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    const 𝔯𝔡𝔦: Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;

                    const 𝔯8: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ;
                    const 𝔯9: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ;
                    const 𝔯10: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ;
                    const 𝔯11: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ;
                    const 𝔯12: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ;
                    const 𝔯13: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ;
                    const 𝔯14: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ;
                    const 𝔯15: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ;]

                    const 𝔢𝔰: Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔠𝔰: Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔰𝔰: Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔡𝔰: Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔣𝔰: Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔤𝔰: Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;

                    const 𝔵1: Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞;
                    const 𝔵2: Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞;
                    const 𝔵4: Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞;
                    const 𝔵8: Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞;

                    const 𝔰𝔱: Self::𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔰𝔱0: Self::𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔰𝔱1: Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔰𝔱2: Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔰𝔱3: Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔰𝔱4: Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔰𝔱5: Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔰𝔱6: Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔰𝔱7: Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;

                    const 𝔪𝔪0: Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔪𝔪1: Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔪𝔪2: Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔪𝔪3: Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔪𝔪4: Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔪𝔪5: Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔪𝔪6: Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔪𝔪7: Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;

                    const 𝔵𝔪𝔪0: Self::𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪1: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪2: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪3: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪4: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪5: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪6: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪7: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;

                Ξ64[const 𝔵𝔪𝔪8: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪9: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪10: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪11: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪12: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪13: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪14: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪15: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;]

                Ξ𝔷𝔷[const 𝔵𝔪𝔪16: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪17: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪18: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪19: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪20: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪21: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪22: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪23: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪24: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪25: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪26: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪27: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪28: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪29: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪30: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔵𝔪𝔪31: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;]

                    const 𝔶𝔪𝔪0: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪1: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪2: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪3: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪4: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪5: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪6: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪7: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;

                Ξ64[const 𝔶𝔪𝔪8: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪9: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪10: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪11: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪12: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪13: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪14: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪15: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;]

                Ξ𝔷𝔷[const 𝔶𝔪𝔪16: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪17: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪18: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪19: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪20: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪21: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪22: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪23: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪24: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪25: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪26: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪27: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪28: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪29: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪30: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔶𝔪𝔪31: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;]

            Χ𝔷𝔷[Ξ86[const 𝔨0: Self::𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔨1: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔨2: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔨3: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔨4: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔨5: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔨6: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔨7: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;

                    const 𝔷𝔪𝔪0: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪1: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪2: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪3: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪4: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪5: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪6: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪7: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;]]

                Ξ𝔷𝔷[const 𝔨0: Self::𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔨1: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔨2: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔨3: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔨4: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔨5: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔨6: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔨7: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;

                    const 𝔷𝔪𝔪0: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪1: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪2: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪3: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪4: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪5: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪6: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪7: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪8: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪9: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪10: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪11: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪12: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪13: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪14: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪15: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪16: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪17: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪18: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪19: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪20: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪21: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪22: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪23: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪24: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪25: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪26: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪27: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪28: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪29: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪30: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    const 𝔷𝔪𝔪31: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;]

                Ξ86[const 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_16ᵇⁱᵗ: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<
                        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
                        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ,
                        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ,
                        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
                        0,
                    > = 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086 {
                        𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
                        𝖻𝖺𝗌𝖾: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ {},
                        𝗂𝗇𝖽𝖾𝗑: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ {},
                        𝖽𝗂𝗌𝗉: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
                    };]
                    const 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
                        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
                        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ,
                        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ,
                        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
                        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
                        0,
                    > = 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86 {
                        𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
                        𝖻𝖺𝗌𝖾: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ {},
                        𝗂𝗇𝖽𝖾𝗑: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ {},
                        𝗌𝖼𝖺𝗅𝖾: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
                        𝖽𝗂𝗌𝗉: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
                    };
                Ξ64[const 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_64ᵇⁱᵗ: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
                        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
                        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ,
                        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ,
                        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
                        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
                        0,
                    > = 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86 {
                        𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
                        𝖻𝖺𝗌𝖾: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ {},
                        𝗂𝗇𝖽𝖾𝗑: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ {},
                        𝗌𝖼𝖺𝗅𝖾: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
                        𝖽𝗂𝗌𝗉: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
                    };]

                    $($𝓮𝔁𝓽𝓻𝓪_𝓭𝓮𝓯𝓲𝓷𝓮𝓼)*
                }
            }
        }
    };
    ($( $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        pub trait $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident {
           $($𝓮𝔁𝓽𝓻𝓪_𝓭𝓮𝓯𝓲𝓷𝓮𝓼:tt)*
        }
      )*) => {
        $(  𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖙𝖗𝖆𝖎𝖙! {
                $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
                pub trait $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮 {
                    $($𝓮𝔁𝓽𝓻𝓪_𝓭𝓮𝓯𝓲𝓷𝓮𝓼)*
                }
            }
         )*
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        pub trait $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        where $($𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼_𝓪𝓷𝓭_𝓭𝓮𝓯𝓲𝓷𝓮𝓼:tt)*
      ) => {
        #[allow(non_upper_case_globals)]
        pub trait $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮: Sized
        where $($𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼_𝓪𝓷𝓭_𝓭𝓮𝓯𝓲𝓷𝓮𝓼)*
    };
}

macro_rules! 𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖙𝖗𝖚𝖈𝖙 {
    // We may define different types of assemblers in the future but for now we only support ones
    // which accept reference to 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 and become 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓's themselves.
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        pub struct $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident {
            $([$($𝓮𝔁𝓽𝓻𝓪_𝓭𝓮𝓬𝓵𝓪𝓻𝓪𝓽𝓲𝓸𝓷𝓼:tt)*] [$($𝓮𝔁𝓽𝓻𝓪_𝓲𝓷𝓲𝓽:tt)*])?
        }
    ) => {
        pub struct $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> {
            pub 𝖾𝗆𝗂𝗍𝗍𝖾𝗋: &'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ mut 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮,
            $($($𝓮𝔁𝓽𝓻𝓪_𝓭𝓮𝓬𝓵𝓪𝓻𝓪𝓽𝓲𝓸𝓷𝓼)*)*
        }

        impl<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮> {
            #[inline(always)]
            pub fn from(new_emitter: &'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ mut 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮) -> Self {
                $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮::<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮> {
                    𝖾𝗆𝗂𝗍𝗍𝖾𝗋: new_emitter,
                    $($($𝓮𝔁𝓽𝓻𝓪_𝓲𝓷𝓲𝓽)*)*
                }
            }
        }

        #[doc(hidden)]
        impl<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓> 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮> {
            type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;
            type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;
            #[inline(always)]
            fn emit_i8(&mut self, value: i8) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
                self.𝖾𝗆𝗂𝗍𝗍𝖾𝗋.emit_i8(value)
            }
            #[inline(always)]
            fn emit_u8(&mut self, value: u8) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
                self.𝖾𝗆𝗂𝗍𝗍𝖾𝗋.emit_u8(value)
            }
            #[inline(always)]
            fn emit_i16(&mut self, value: i16) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
                self.𝖾𝗆𝗂𝗍𝗍𝖾𝗋.emit_i16(value)
            }
            #[inline(always)]
            fn emit_u16(&mut self, value: u16) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
                self.𝖾𝗆𝗂𝗍𝗍𝖾𝗋.emit_u16(value)
            }
            #[inline(always)]
            fn emit_i32(&mut self, value: i32) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
                self.𝖾𝗆𝗂𝗍𝗍𝖾𝗋.emit_i32(value)
            }
            #[inline(always)]
            fn emit_u32(&mut self, value: u32) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
                self.𝖾𝗆𝗂𝗍𝗍𝖾𝗋.emit_u32(value)
            }
            #[inline(always)]
            fn emit_i64(&mut self, value: i64) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
                self.𝖾𝗆𝗂𝗍𝗍𝖾𝗋.emit_i64(value)
            }
            #[inline(always)]
            fn emit_u64(&mut self, value: u64) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
                self.𝖾𝗆𝗂𝗍𝗍𝖾𝗋.emit_u64(value)
            }
        }
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖙𝖗𝖚𝖈𝖙! {
            $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
            impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
            for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>]
        }
    };
    ($( $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        pub struct $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident[$($𝓮𝔁𝓽𝓻𝓪_𝓲𝓷𝓯𝓸:tt)*] as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident {
            $($𝓮𝔁𝓽𝓻𝓪_𝓭𝓮𝓬𝓵𝓪𝓻𝓪𝓽𝓲𝓸𝓷𝓼:tt)*
        }
      )*) => {
        $(
            𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖙𝖗𝖚𝖈𝖙! {
                $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
                pub struct $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮[$($𝓮𝔁𝓽𝓻𝓪_𝓲𝓷𝓯𝓸)*] as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮 {
                    $($𝓮𝔁𝓽𝓻𝓪_𝓭𝓮𝓬𝓵𝓪𝓻𝓪𝓽𝓲𝓸𝓷𝓼)*
                }
            }
         )*
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt
    ) => {
        𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! {
            𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖙𝖗𝖚𝖈𝖙! {
                $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
                impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼 $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
                for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼
                {   type 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    type 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    type 𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    type 𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    type 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ;
                    type 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ;
                    type 𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ;
                    type 𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ;
                Ξ64[type 𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    type 𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    type 𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;
                    type 𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ;]

                    type 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    type 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    type 𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    type 𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    type 𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    type 𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    type 𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    type 𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;

                    type 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    type 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    type 𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    type 𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    type 𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    type 𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    type 𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;
                    type 𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ;

                Ξ64[type 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    type 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    type 𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    type 𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    type 𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    type 𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    type 𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    type 𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;]

                Ξ86[type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;
                    type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ;]
            Χ𝔦𝔷[Ξ64[type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386;
                    type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64;]
                Ξ86[type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386;]
                Ξ64[type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64;
                    type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ;
                    type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;]]
            Ξ𝔦𝔷[Ξ64[type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386;
                    type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64;]
                Ξ86[type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386;]
                Ξ64[type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64;
                    type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ;
                    type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;
                    type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;]]

                    type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ;
                    type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ;
                Ξ86[type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086;]
                Ξ64[type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086;
                    type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64;]
                Ξ86[type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086;]
                Ξ64[type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086;
                    type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64;]
                Ξ86[type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386;]
                Ξ64[type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386;
                    type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64;
                    type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ;
                    type 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ;]

                    type 𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;

                    type 𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞;

                    type 𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    type 𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;

                    type 𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;

                    type 𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
            Χ𝔷𝔷[Ξ86[type 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₚₑₙₜᵢᵤₘ3;]
                Ξ64[type 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64;]]
                Ξ𝔷𝔷[type 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512;]

            Χ𝔷𝔷[Ξ86[type 𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ;]
                Ξ64[type 𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64;]]
                Ξ𝔷𝔷[type 𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512;]

                    // ₐᵥₓ512 doesn't extend register set in legacy mode and if ₐᵥₓ512 instructions are not used nothing is added
                    // to the biary.
                    // Thus we provide ₐᵥₓ512 support unconditiOnally in legacy mode, but only if it's requested in ₓ86_64 mode.
                Ξ86[type 𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    type 𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    type 𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86;]
                Ξ𝔷𝔷[type 𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    type 𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;
                    type 𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64;]

                    const 𝔞𝔩: Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ = Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ::𝔞𝔩;
                    const 𝔠𝔩: Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ = Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ::𝔠𝔩;
                    const 𝔡𝔩: Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ = Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ::𝔡𝔩;
                    const 𝔟𝔩: Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ = Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ::𝔟𝔩;
                    const 𝔞𝔥: Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ = Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ::𝔞𝔥;
                    const 𝔠𝔥: Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ = Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ::𝔠𝔥;
                    const 𝔡𝔥: Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ = Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ::𝔡𝔥;
                    const 𝔟𝔥: Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ = Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ::𝔟𝔥;
                Ξ64[const 𝔰𝔭𝔩: Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ = Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ::𝔰𝔭𝔩;
                    const 𝔟𝔭𝔩: Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ = Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ::𝔟𝔭𝔩;
                    const 𝔰𝔦𝔩: Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ = Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ::𝔰𝔦𝔩;
                    const 𝔡𝔦𝔩: Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ = Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ::𝔡𝔦𝔩;

                    // Note: use of 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ and 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ means you can not easily convert, e.g., 𝔯8𝔟 into 𝔯8𝔴.
                    // This is a bit surprising.  Consider creation or 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_{8,16}ᵇⁱᵗₙₒⁱᶻ types?
                    const 𝔯8𝔟: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ::𝔯8𝔟;
                    const 𝔯9𝔟: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ::𝔯9𝔟;
                    const 𝔯10𝔟: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ::𝔯10𝔟;
                    const 𝔯11𝔟: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ::𝔯11𝔟;
                    const 𝔯12𝔟: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ::𝔯12𝔟;
                    const 𝔯13𝔟: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ::𝔯13𝔟;
                    const 𝔯14𝔟: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ::𝔯14𝔟;
                    const 𝔯15𝔟: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ::𝔯15𝔟;]

                    const 𝔞𝔵: Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ::𝔞𝔵;
                    const 𝔠𝔵: Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ::𝔠𝔵;
                    const 𝔡𝔵: Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ::𝔡𝔵;
                    const 𝔟𝔵: Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ::𝔟𝔵;
                    const 𝔰𝔭: Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ::𝔰𝔭;
                    const 𝔟𝔭: Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ::𝔟𝔭;
                    const 𝔰𝔦: Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ::𝔰𝔦;
                    const 𝔡𝔦: Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ::𝔡𝔦;

                    // Note: use of 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ and 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ means you can not easily convert, e.g., 𝔯8𝔟 into 𝔯8𝔴.
                    // This is a bit surprising.  Consider creation or 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_{8,16}ᵇⁱᵗₙₒⁱᶻ types?
                Ξ64[const 𝔯8𝔴: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ::𝔯8𝔴;
                    const 𝔯9𝔴: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ::𝔯9𝔴;
                    const 𝔯10𝔴: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ::𝔯10𝔴;
                    const 𝔯11𝔴: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ::𝔯11𝔴;
                    const 𝔯12𝔴: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ::𝔯12𝔴;
                    const 𝔯13𝔴: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ::𝔯13𝔴;
                    const 𝔯14𝔴: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ::𝔯14𝔴;
                    const 𝔯15𝔴: Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ = Self::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ::𝔯15𝔴;]

                    const 𝔢𝔞𝔵: Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ::𝔢𝔞𝔵;
                    const 𝔢𝔠𝔵: Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ::𝔢𝔠𝔵;
                    const 𝔢𝔡𝔵: Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ::𝔢𝔡𝔵;
                    const 𝔢𝔟𝔵: Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ::𝔢𝔟𝔵;
                    const 𝔢𝔰𝔭: Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ::𝔢𝔰𝔭;
                    const 𝔢𝔟𝔭: Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ::𝔢𝔟𝔭;
                    const 𝔢𝔰𝔦: Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ::𝔢𝔰𝔦;
                    const 𝔢𝔡𝔦: Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ = Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ::𝔢𝔡𝔦;

                Ξ64[const 𝔯8𝔡: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ::𝔯8𝔡;
                    const 𝔯9𝔡: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ::𝔯9𝔡;
                    const 𝔯10𝔡: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ::𝔯10𝔡;
                    const 𝔯11𝔡: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ::𝔯11𝔡;
                    const 𝔯12𝔡: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ::𝔯12𝔡;
                    const 𝔯13𝔡: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ::𝔯13𝔡;
                    const 𝔯14𝔡: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ::𝔯14𝔡;
                    const 𝔯15𝔡: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ::𝔯15𝔡;

                    const 𝔯𝔞𝔵: Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = Self::𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ::𝔯𝔞𝔵;
                    const 𝔯𝔠𝔵: Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = Self::𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ::𝔯𝔠𝔵;
                    const 𝔯𝔡𝔵: Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = Self::𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ::𝔯𝔡𝔵;
                    const 𝔯𝔟𝔵: Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = Self::𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ::𝔯𝔟𝔵;
                    const 𝔯𝔰𝔭: Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = Self::𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ::𝔯𝔰𝔭;
                    const 𝔯𝔟𝔭: Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = Self::𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ::𝔯𝔟𝔭;
                    const 𝔯𝔰𝔦: Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = Self::𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ::𝔯𝔰𝔦;
                    const 𝔯𝔡𝔦: Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ = Self::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ::𝔯𝔡𝔦;

                    const 𝔯8: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ::𝔯8;
                    const 𝔯9: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ::𝔯9;
                    const 𝔯10: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ::𝔯10;
                    const 𝔯11: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ::𝔯11;
                    const 𝔯12: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ::𝔯12;
                    const 𝔯13: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ::𝔯13;
                    const 𝔯14: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ::𝔯14;
                    const 𝔯15: Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ = Self::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ::𝔯15;]

                    const 𝔢𝔰: Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔢𝔰;
                    const 𝔠𝔰: Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔠𝔰;
                    const 𝔰𝔰: Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔰𝔰;
                    const 𝔡𝔰: Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔡𝔰;
                    const 𝔣𝔰: Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔣𝔰;
                    const 𝔤𝔰: Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔤𝔰;

                    const 𝔵1: Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞 = Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞::𝔵1;
                    const 𝔵2: Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞 = Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞::𝔵2;
                    const 𝔵4: Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞 = Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞::𝔵4;
                    const 𝔵8: Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞 = Self::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞::𝔵8;

                    const 𝔰𝔱: Self::𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔰𝔱;
                    const 𝔰𝔱0: Self::𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔰𝔱;
                    const 𝔰𝔱1: Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔰𝔱1;
                    const 𝔰𝔱2: Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔰𝔱2;
                    const 𝔰𝔱3: Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔰𝔱3;
                    const 𝔰𝔱4: Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔰𝔱4;
                    const 𝔰𝔱5: Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔰𝔱5;
                    const 𝔰𝔱6: Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔰𝔱6;
                    const 𝔰𝔱7: Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔰𝔱7;

                    const 𝔪𝔪0: Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔪𝔪0;
                    const 𝔪𝔪1: Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔪𝔪1;
                    const 𝔪𝔪2: Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔪𝔪2;
                    const 𝔪𝔪3: Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔪𝔪3;
                    const 𝔪𝔪4: Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔪𝔪4;
                    const 𝔪𝔪5: Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔪𝔪5;
                    const 𝔪𝔪6: Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔪𝔪6;
                    const 𝔪𝔪7: Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔪𝔪7;

                    const 𝔵𝔪𝔪0: Self::𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪0;
                    const 𝔵𝔪𝔪1: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪1;
                    const 𝔵𝔪𝔪2: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪2;
                    const 𝔵𝔪𝔪3: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪3;
                    const 𝔵𝔪𝔪4: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪4;
                    const 𝔵𝔪𝔪5: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪5;
                    const 𝔵𝔪𝔪6: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪6;
                    const 𝔵𝔪𝔪7: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪7;

                Ξ64[const 𝔵𝔪𝔪8: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪8;
                    const 𝔵𝔪𝔪9: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪9;
                    const 𝔵𝔪𝔪10: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪10;
                    const 𝔵𝔪𝔪11: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪11;
                    const 𝔵𝔪𝔪12: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪12;
                    const 𝔵𝔪𝔪13: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪13;
                    const 𝔵𝔪𝔪14: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪14;
                    const 𝔵𝔪𝔪15: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪15;]

                Ξ𝔷𝔷[const 𝔵𝔪𝔪16: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪16;
                    const 𝔵𝔪𝔪17: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪17;
                    const 𝔵𝔪𝔪18: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪18;
                    const 𝔵𝔪𝔪19: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪19;
                    const 𝔵𝔪𝔪20: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪20;
                    const 𝔵𝔪𝔪21: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪21;
                    const 𝔵𝔪𝔪22: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪22;
                    const 𝔵𝔪𝔪23: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪23;
                    const 𝔵𝔪𝔪24: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪24;
                    const 𝔵𝔪𝔪25: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪25;
                    const 𝔵𝔪𝔪26: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪26;
                    const 𝔵𝔪𝔪27: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪27;
                    const 𝔵𝔪𝔪28: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪28;
                    const 𝔵𝔪𝔪29: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪29;
                    const 𝔵𝔪𝔪30: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪30;
                    const 𝔵𝔪𝔪31: Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔵𝔪𝔪31;]

                    const 𝔶𝔪𝔪0: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪0;
                    const 𝔶𝔪𝔪1: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪1;
                    const 𝔶𝔪𝔪2: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪2;
                    const 𝔶𝔪𝔪3: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪3;
                    const 𝔶𝔪𝔪4: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪4;
                    const 𝔶𝔪𝔪5: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪5;
                    const 𝔶𝔪𝔪6: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪6;
                    const 𝔶𝔪𝔪7: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪7;

                Ξ64[const 𝔶𝔪𝔪8: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪8;
                    const 𝔶𝔪𝔪9: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪9;
                    const 𝔶𝔪𝔪10: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪10;
                    const 𝔶𝔪𝔪11: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪11;
                    const 𝔶𝔪𝔪12: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪12;
                    const 𝔶𝔪𝔪13: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪13;
                    const 𝔶𝔪𝔪14: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪14;
                    const 𝔶𝔪𝔪15: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪15;]

                Ξ𝔷𝔷[const 𝔶𝔪𝔪16: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪16;
                    const 𝔶𝔪𝔪17: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪17;
                    const 𝔶𝔪𝔪18: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪18;
                    const 𝔶𝔪𝔪19: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪19;
                    const 𝔶𝔪𝔪20: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪20;
                    const 𝔶𝔪𝔪21: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪21;
                    const 𝔶𝔪𝔪22: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪22;
                    const 𝔶𝔪𝔪23: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪23;
                    const 𝔶𝔪𝔪24: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪24;
                    const 𝔶𝔪𝔪25: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪25;
                    const 𝔶𝔪𝔪26: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪26;
                    const 𝔶𝔪𝔪27: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪27;
                    const 𝔶𝔪𝔪28: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪28;
                    const 𝔶𝔪𝔪29: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪29;
                    const 𝔶𝔪𝔪30: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪30;
                    const 𝔶𝔪𝔪31: Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔶𝔪𝔪31;]

            Χ𝔷𝔷[Ξ86[const 𝔨0: Self::𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔨0;
                    const 𝔨1: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔨1;
                    const 𝔨2: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔨2;
                    const 𝔨3: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔨3;
                    const 𝔨4: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔨4;
                    const 𝔨5: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔨5;
                    const 𝔨6: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔨6;
                    const 𝔨7: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔨7;

                    const 𝔷𝔪𝔪0: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪0;
                    const 𝔷𝔪𝔪1: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪1;
                    const 𝔷𝔪𝔪2: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪2;
                    const 𝔷𝔪𝔪3: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪3;
                    const 𝔷𝔪𝔪4: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪4;
                    const 𝔷𝔪𝔪5: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪5;
                    const 𝔷𝔪𝔪6: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪6;
                    const 𝔷𝔪𝔪7: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪7;]]

                Ξ𝔷𝔷[const 𝔨0: Self::𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔨0;
                    const 𝔨1: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔨1;
                    const 𝔨2: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔨2;
                    const 𝔨3: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔨3;
                    const 𝔨4: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔨4;
                    const 𝔨5: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔨5;
                    const 𝔨6: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔨6;
                    const 𝔨7: Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔨7;

                    const 𝔷𝔪𝔪0: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪0;
                    const 𝔷𝔪𝔪1: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪1;
                    const 𝔷𝔪𝔪2: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪2;
                    const 𝔷𝔪𝔪3: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪3;
                    const 𝔷𝔪𝔪4: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪4;
                    const 𝔷𝔪𝔪5: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪5;
                    const 𝔷𝔪𝔪6: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪6;
                    const 𝔷𝔪𝔪7: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪7;
                    const 𝔷𝔪𝔪8: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪8;
                    const 𝔷𝔪𝔪9: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪9;
                    const 𝔷𝔪𝔪10: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪10;
                    const 𝔷𝔪𝔪11: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪11;
                    const 𝔷𝔪𝔪12: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪12;
                    const 𝔷𝔪𝔪13: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪13;
                    const 𝔷𝔪𝔪14: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪14;
                    const 𝔷𝔪𝔪15: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪15;
                    const 𝔷𝔪𝔪16: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪16;
                    const 𝔷𝔪𝔪17: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪17;
                    const 𝔷𝔪𝔪18: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪18;
                    const 𝔷𝔪𝔪19: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪19;
                    const 𝔷𝔪𝔪20: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪20;
                    const 𝔷𝔪𝔪21: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪21;
                    const 𝔷𝔪𝔪22: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪22;
                    const 𝔷𝔪𝔪23: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪23;
                    const 𝔷𝔪𝔪24: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪24;
                    const 𝔷𝔪𝔪25: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪25;
                    const 𝔷𝔪𝔪26: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪26;
                    const 𝔷𝔪𝔪27: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪27;
                    const 𝔷𝔪𝔪28: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪28;
                    const 𝔷𝔪𝔪29: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪29;
                    const 𝔷𝔪𝔪30: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪30;
                    const 𝔷𝔪𝔪31: Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = Self::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫::𝔷𝔪𝔪31;]
                }
            }
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt)*] $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt)*]
        {   $($𝓭𝓮𝓯𝓲𝓷𝓮𝓼:tt)*
        }
    ) => {
       #[allow(non_upper_case_globals)]
       impl$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼)* $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
       for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)*
       {   $($𝓭𝓮𝓯𝓲𝓷𝓮𝓼)*
       }
    };
}

macro_rules! 𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘 {
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident;
    ) => {
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
            $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
            impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
            for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>];
        }
    };
    ($( $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl[$($𝓮𝔁𝓽𝓻𝓪_𝓲𝓷𝓯𝓸:tt)*] for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident;
      )*) => {
        $(
            𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
                $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
                impl[$($𝓮𝔁𝓽𝓻𝓪_𝓲𝓷𝓯𝓸)*] for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮 as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮;
            }
         )*
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt;
    ) => {
        𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! {
            𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
                $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
                impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼 $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
                for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼

                𝖒𝖆𝖗𝖐 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕:
                    𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            Ξ86[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,]
            Ξ64[    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,]
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,

                    𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            Ξ64[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,]
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ

            Ξ64[  , 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ];

                𝖒𝖆𝖗𝖐 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕:
                    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 => 𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
                    𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 => 𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;

                𝖒𝖆𝖗𝖐 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆:
            Ξ86[    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,]

                    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            Ξ64[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,]
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ

            Ξ64[  , 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ];

                𝖒𝖆𝖗𝖐 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙:
            Ξ86[    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,]

                    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            Ξ64[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,]
        Ξ𝔦𝔷[Ξ64[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,]]
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ

            Ξ64[  , 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,]
        Ξ𝔦𝔷[Ξ64[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,]]
            Ξ64[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ];

                𝖒𝖆𝖗𝖐 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒄𝒂𝒍𝒆:
                    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 => 𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
                    𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞 => 𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞;

                𝖒𝖆𝖗𝖐 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏:
                    𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ =>Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ],
                    𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ],
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ],
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ],
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ],
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ],
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ],
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ],
            Ξ64[    𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ,
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ,
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ,
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ,]
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ =>Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ],
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ],
            Ξ86[    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,]
            Ξ64[    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ,]

                    𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ],
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ],
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ],
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ],
                    𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ],
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ],
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ],
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ],
            Ξ86[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,]
            Ξ64[    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ,]
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,

                    𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
            Ξ64[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,]
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ

            Ξ64[  , 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ];

                𝖒𝖆𝖗𝖐 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏:
                    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 => 𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
                    𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 => 𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫;

                𝖒𝖆𝖗𝖐 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏:
            Ξ86[    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,]

                    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
            Ξ64[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,]
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ

            Ξ64[  , 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ];

                𝖒𝖆𝖗𝖐 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏:
            Ξ86[    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,]

                    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ => Ξ86[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
            Ξ64[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,]
        Ξ𝔦𝔷[Ξ64[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,]]
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ

            Ξ64[  , 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,]
        Ξ𝔦𝔷[Ξ64[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,]]
            Ξ64[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ];

                𝖒𝖆𝖗𝖐 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒄𝒂𝒍𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏:
                    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 => 𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
                    𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞 => 𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞;

                𝖒𝖆𝖗𝖐 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓:
                    𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ => 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
                    𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ],
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ],
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ],
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ],
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ],
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ],
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ],
            Ξ64[    𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ,
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ,
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ,
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ,]
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ],
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ],
            Ξ86[    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,]
            Ξ64[    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ,]

                    𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ],
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ],
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ],
                    𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ],
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ],
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ],
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ],
            Ξ86[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,]
            Ξ64[    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗₙₒᵣₑₓ,]
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,

                    𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ],
            Ξ64[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵉⁱᶻ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,]
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ

            Ξ64[  , 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒʳⁱᶻ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ];

                𝖒𝖆𝖗𝖐 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆:
            Ξ86[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ<𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,]

            Ξ64[    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ<𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ<𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ<𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,]
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ<𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ

            Ξ64[  , 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ<𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ<𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ<𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ<𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> => 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ];

                𝖒𝖆𝖗𝖐 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙:
            Ξ86[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ<𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ> => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,]

            Ξ64[    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ> => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,]
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ> => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ

            Ξ64[  , 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ> => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
                    𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ> => 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ];

                𝖋𝖔𝖗𝖜𝖆𝖗𝖉 𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏 fn add_impl:
            Ξ64[    (𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ) => (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ),
                    (𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ) => (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, argument2),]
            Ξ86[    (𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ) => (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, argument2),]
            Ξ64[    (𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ) => (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, argument2),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ) => (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ) => (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, argument2),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ) => (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, argument2),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ) => (argument1, 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ) => (argument1, 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ),]
            Ξ86[    (𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, 0>) =>
                        (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, to_byte_ptr),
                    (𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, 1>) =>
                        (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, argument2),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, 0>) =>
                        (argument1, to_byte_ptr),]
            Ξ64[    (𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 0>) =>
                        (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, to_byte_ptr),
                    (𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 1>) =>
                        (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, argument2),]
                    (𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 0>) =>
                        (Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ], to_byte_ptr),
                    (𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 1>) =>
                        (Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ], argument2),
            Ξ64[    (𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 0>) =>
                        (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, to_byte_ptr),
                    (𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 1>) =>
                        (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, argument2),
                    (𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 0>) =>
                        (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, to_byte_ptr),
                    (𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 1>) =>
                        (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, argument2),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 0>) =>
                        (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, to_byte_ptr),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 1>) =>
                        (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, argument2),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 0>) =>
                        (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, to_byte_ptr),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 1>) =>
                        (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, argument2),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 0>) =>
                        (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, to_byte_ptr),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 1>) =>
                        (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, argument2),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 0>) =>
                        (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, to_byte_ptr),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 1>) =>
                        (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, argument2),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 0>) =>
                        (argument1, to_byte_ptr),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 0>) =>
                        (argument1, to_byte_ptr),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 0>) =>
                        (argument1, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 1>),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 1>) =>
                        (argument1, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 1>),]
                    (Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ], 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 0>) =>
                        (argument1, to_byte_ptr)
            Ξ64[  , (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 0>) =>
                        (argument1, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 1>),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 1>) =>
                        (argument1, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 1>),
                    (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 0>) =>
                        (argument1, to_byte_ptr)];

                impl 𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏 {
                    fn add_impl(self, (argument1, argument2): (Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ], Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ])) {
                        self.emit_u8(argument1 as u8)?;
                        self.emit_u8(argument2 as u8)
                    }
                }
            Ξ64[impl 𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏 {
                    fn add_impl(self, (argument1, argument2): (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ)) {
                        self.emit_u8(argument1 as u8)?;
                        self.emit_u8(argument2 as u8)
                    }
                }]
            Ξ86[impl 𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏 {
                    fn add_impl(self, (argument1, argument2): (
                            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, 1>)) {
                        self.emit_u8(argument1 as u8)?;
                        if let Some(register) = argument2.𝗌𝖾𝗀𝗆𝖾𝗇𝗍 {
                            self.emit_u8(1)?;
                            self.emit_u8(register as u8)?;
                        } else {
                            self.emit_u8(0)?;
                        }
                        if let Some(register) = argument2.𝖻𝖺𝗌𝖾 {
                            self.emit_u8(1)?;
                            self.emit_u8(register as u8)?;
                        } else {
                            self.emit_u8(0)?;
                        }
                        if let Some(register) = argument2.𝗂𝗇𝖽𝖾𝗑 {
                            self.emit_u8(1)?;
                            self.emit_u8(register as u8)?;
                        } else {
                            self.emit_u8(0)?;
                        }
                        self.emit_i16(argument2.𝖽𝗂𝗌𝗉)
                    }
                }]
            Ξ64[impl 𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏 {
                    fn add_impl(self, (argument1, argument2): (
                            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗₙₒᵣₑₓ, 1>)) {
                        self.emit_u8(argument1 as u8)?;
                        if let Some(register) = argument2.𝗌𝖾𝗀𝗆𝖾𝗇𝗍 {
                            self.emit_u8(1)?;
                            self.emit_u8(register as u8)?;
                        } else {
                            self.emit_u8(0)?;
                        }
                        if let Some(register) = argument2.𝖻𝖺𝗌𝖾 {
                            self.emit_u8(1)?;
                            self.emit_u8(register as u8)?;
                        } else {
                            self.emit_u8(0)?;
                        }
                        if let Some(register) = argument2.𝗂𝗇𝖽𝖾𝗑 {
                            self.emit_u8(1)?;
                            self.emit_u8(register as u8)?;
                        } else {
                            self.emit_u8(0)?;
                        }
                        self.emit_u8(argument2.𝗌𝖼𝖺𝗅𝖾 as u8)?;
                        self.emit_i32(argument2.𝖽𝗂𝗌𝗉)
                    }
                }]
                impl 𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏 {
                    fn add_impl(self, (argument1, argument2): (
                            Ξ86[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ]Ξ64[𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ], 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ, 1>)) {
                        self.emit_u8(argument1 as u8)?;
                        if let Some(register) = argument2.𝗌𝖾𝗀𝗆𝖾𝗇𝗍 {
                            self.emit_u8(1)?;
                            self.emit_u8(register as u8)?;
                        } else {
                            self.emit_u8(0)?;
                        }
                        if let Some(register) = argument2.𝖻𝖺𝗌𝖾 {
                            self.emit_u8(1)?;
                            self.emit_u8(register as u8)?;
                        } else {
                            self.emit_u8(0)?;
                        }
                        if let Some(register) = argument2.𝗂𝗇𝖽𝖾𝗑 {
                            self.emit_u8(1)?;
                            self.emit_u8(register as u8)?;
                        } else {
                            self.emit_u8(0)?;
                        }
                        self.emit_u8(argument2.𝗌𝖼𝖺𝗅𝖾 as u8)?;
                        self.emit_i32(argument2.𝖽𝗂𝗌𝗉)
                    }
                }
            Ξ64[impl 𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏 {
                    fn add_impl(self, (argument1, argument2): (
                            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₙₒᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗₙₒᵣₑₓ, 1>)) {
                        self.emit_u8(argument1 as u8)?;
                        if let Some(register) = argument2.𝗌𝖾𝗀𝗆𝖾𝗇𝗍 {
                            self.emit_u8(1)?;
                            self.emit_u8(register as u8)?;
                        } else {
                            self.emit_u8(0)?;
                        }
                        if let Some(register) = argument2.𝖻𝖺𝗌𝖾 {
                            self.emit_u8(1)?;
                            self.emit_u8(register as u8)?;
                        } else {
                            self.emit_u8(0)?;
                        }
                        if let Some(register) = argument2.𝗂𝗇𝖽𝖾𝗑 {
                            self.emit_u8(1)?;
                            self.emit_u8(register as u8)?;
                        } else {
                            self.emit_u8(0)?;
                        }
                        self.emit_u8(argument2.𝗌𝖼𝖺𝗅𝖾 as u8)?;
                        self.emit_i32(argument2.𝖽𝗂𝗌𝗉)
                    }
                }
                impl 𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏 {
                    fn add_impl(self, (argument1, argument2): (
                            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 1>)) {
                        self.emit_u8(argument1 as u8)?;
                        if let Some(register) = argument2.𝗌𝖾𝗀𝗆𝖾𝗇𝗍 {
                            self.emit_u8(1)?;
                            self.emit_u8(register as u8)?;
                        } else {
                            self.emit_u8(0)?;
                        }
                        if let Some(register) = argument2.𝖻𝖺𝗌𝖾 {
                            self.emit_u8(1)?;
                            self.emit_u8(register as u8)?;
                        } else {
                            self.emit_u8(0)?;
                        }
                        if let Some(register) = argument2.𝗂𝗇𝖽𝖾𝗑 {
                            self.emit_u8(1)?;
                            self.emit_u8(register as u8)?;
                        } else {
                            self.emit_u8(0)?;
                        }
                        self.emit_u8(argument2.𝗌𝖼𝖺𝗅𝖾 as u8)?;
                        self.emit_i32(argument2.𝖽𝗂𝗌𝗉)
                    }
                }]
            }
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt)*] $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt)*]
        𝖒𝖆𝖗𝖐 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆: $𝓼𝓸𝓾𝓻𝓬𝓮_𝓷𝓪𝓶𝓮:ident<$𝓲𝓷𝓭𝓮𝔁_𝓷𝓪𝓶𝓮:ident> => $𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮:ident;
    ) => {
        #[doc(hidden)]
        impl$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼)* 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<
            $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)*,
            <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓲𝓷𝓭𝓮𝔁_𝓷𝓪𝓶𝓮>
        for <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓼𝓸𝓾𝓻𝓬𝓮_𝓷𝓪𝓶𝓮 {
            type 𝐭𝐚𝐫𝐠𝐞𝐭 = <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮;
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt)*] $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt)*]
        𝖒𝖆𝖗𝖐 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙: $𝓼𝓸𝓾𝓻𝓬𝓮_𝓷𝓪𝓶𝓮:ident<$𝓫𝓪𝓼𝓮_𝓷𝓪𝓶𝓮:ident> => $𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮:ident;
    ) => {
        #[doc(hidden)]
        impl$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼)* 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<
            $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)*,
            <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓫𝓪𝓼𝓮_𝓷𝓪𝓶𝓮>
        for <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓼𝓸𝓾𝓻𝓬𝓮_𝓷𝓪𝓶𝓮 {
            type 𝐭𝐚𝐫𝐠𝐞𝐭 = <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮;
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt)*] $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt)*]
        𝖒𝖆𝖗𝖐 $𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 => $𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮:ident;
    ) => {
        #[doc(hidden)]
        impl$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼)* $𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷_𝓷𝓪𝓶𝓮<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)*>
        for 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {
            type 𝐭𝐚𝐫𝐠𝐞𝐭 = <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮;
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt)*] $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt)*]
        𝖒𝖆𝖗𝖐 $𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ => $𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮:ident;
    ) => {
        #[doc(hidden)]
        impl$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼)* $𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷_𝓷𝓪𝓶𝓮<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)*>
        for 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ {
            type 𝐭𝐚𝐫𝐠𝐞𝐭 = <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮;
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt)*] $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt)*]
        𝖒𝖆𝖗𝖐 $𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ => $𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮:ident;
    ) => {
        #[doc(hidden)]
        impl$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼)* $𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷_𝓷𝓪𝓶𝓮<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)*>
        for 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ {
            type 𝐭𝐚𝐫𝐠𝐞𝐭 = <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮;
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt)*] $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt)*]
        𝖒𝖆𝖗𝖐 $𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ => $𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮:ident;
    ) => {
        #[doc(hidden)]
        impl$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼)* $𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷_𝓷𝓪𝓶𝓮<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)*>
        for 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ {
            type 𝐭𝐚𝐫𝐠𝐞𝐭 = <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮;
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt)*] $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt)*]
        𝖒𝖆𝖗𝖐 $𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident: $𝓼𝓸𝓾𝓻𝓬𝓮_𝓷𝓪𝓶𝓮:ident => $𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮:ident;
    ) => {
        #[doc(hidden)]
        impl$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼)* $𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷_𝓷𝓪𝓶𝓮<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)*>
        for <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓼𝓸𝓾𝓻𝓬𝓮_𝓷𝓪𝓶𝓮 {
            type 𝐭𝐚𝐫𝐠𝐞𝐭 = <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮;
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt
        𝖒𝖆𝖗𝖐 $𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident:
            $($𝓼𝓸𝓾𝓻𝓬𝓮_𝓷𝓪𝓶𝓮:ident<$𝓮𝔁𝓽𝓻𝓪_𝓷𝓪𝓶𝓮:ident> => $𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮:ident),*;
        $($𝓸𝓽𝓱𝓮𝓻_𝓽𝓻𝓪𝓲𝓽𝓼:tt)*
    ) => {
        $(  𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
            $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
            impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼 $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
            for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼
            𝖒𝖆𝖗𝖐 $𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷_𝓷𝓪𝓶𝓮: $𝓼𝓸𝓾𝓻𝓬𝓮_𝓷𝓪𝓶𝓮<$𝓮𝔁𝓽𝓻𝓪_𝓷𝓪𝓶𝓮> => $𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮;
        }
        )*
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
            $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
            impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼 $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
            for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼
            $($𝓸𝓽𝓱𝓮𝓻_𝓽𝓻𝓪𝓲𝓽𝓼)*
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt
        𝖒𝖆𝖗𝖐 $𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident:
           $($𝓼𝓸𝓾𝓻𝓬𝓮_𝓷𝓪𝓶𝓮:ident => $𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮:ident),*;
        $($𝓸𝓽𝓱𝓮𝓻_𝓽𝓻𝓪𝓲𝓽𝓼:tt)*
    ) => {
        $(  𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
            $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
            impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼 $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
            for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼
            𝖒𝖆𝖗𝖐 $𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷_𝓷𝓪𝓶𝓮: $𝓼𝓸𝓾𝓻𝓬𝓮_𝓷𝓪𝓶𝓮 => $𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮;
        }
        )*
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
            $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
            impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼 $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
            for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼
            $($𝓸𝓽𝓱𝓮𝓻_𝓽𝓻𝓪𝓲𝓽𝓼)*
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt
        𝖋𝖔𝖗𝖜𝖆𝖗𝖉 $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident:
            ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮:ident, $𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮:ident) =>
            (argument1, $𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝓪𝓻𝓰𝓮𝓽_𝓽𝔂𝓹𝓮:ident);
    ) => {
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
            $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
            impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼 $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
            for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼
            impl $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮 {
                fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮(self, (argument1, argument2): ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮, $𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮)) {
                    self.add_impl((argument1,
                                   Into::<<Self as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝓪𝓻𝓰𝓮𝓽_𝓽𝔂𝓹𝓮>::into(argument2)))
                }
            }
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt
        𝖋𝖔𝖗𝖜𝖆𝖗𝖉 $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident:
            ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮:ident, $𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮:ident) =>
            ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝓪𝓻𝓰𝓮𝓽_𝓽𝔂𝓹𝓮:ident, argument2);
    ) => {
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
            $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
            impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼 $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
            for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼
            impl $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮 {
                fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮(self, (argument1, argument2): ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮, $𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮)) {
                    self.add_impl((Into::<<Self as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝓪𝓻𝓰𝓮𝓽_𝓽𝔂𝓹𝓮>::into(argument1),
                                  argument2))
                }
            }
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt
        𝖋𝖔𝖗𝖜𝖆𝖗𝖉 $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident:
            ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮:ident,
             $𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝔁86:ident<$𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮:ident, $𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮:ident, $𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮:literal>) =>
            ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝓪𝓻𝓰𝓮𝓽_𝓽𝔂𝓹𝓮:ident, argument2);
    ) => {
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
            $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
            impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼 $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
            for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼
            impl $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮 {
                fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮(self, (argument1, argument2): (
                        $𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮,
                        $𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝔁86<$𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮, $𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮, $𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>)) {
                    self.add_impl((Into::<<Self as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝓪𝓻𝓰𝓮𝓽_𝓽𝔂𝓹𝓮>::into(argument1),
                                  argument2))
                }
            }
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt
        𝖋𝖔𝖗𝖜𝖆𝖗𝖉 $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident:
            ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮:ident,
             $𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝔁86:ident<$𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮:ident, $𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮:ident, 0>) =>
            (argument1, $𝓽𝓸_𝓹𝓸𝓲𝓷𝓽𝓮𝓻_𝓽𝔂𝓹𝓮:ident);
    ) => {
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
            $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
            impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼 $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
            for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼
            impl $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮 {
                fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮(self, (argument1, argument2): (
                        $𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮,
                        $𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝔁86<$𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮, $𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮, 0>)) {
                    self.add_impl((argument1, argument2.$𝓽𝓸_𝓹𝓸𝓲𝓷𝓽𝓮𝓻_𝓽𝔂𝓹𝓮()))
                }
            }
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt
        𝖋𝖔𝖗𝖜𝖆𝖗𝖉 $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident:
            ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮:ident,
             𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<$𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮:ident, $𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮:ident, $𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓲𝔃𝓮:literal>) =>
            (argument1,
             𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<$𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝓪𝓻𝓰𝓮𝓽_𝓽𝔂𝓹𝓮:ident, $𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝓪𝓻𝓰𝓮𝓽_𝓽𝔂𝓹𝓮:ident, $𝓶𝓮𝓶𝓸𝓻𝔂_𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓲𝔃𝓮:literal>);
    ) => {
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
            $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
            impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼 $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
            for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼
            impl $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮 {
                fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮(self, (argument1, argument2): (
                        $𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮,
                        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<$𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮, $𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮, $𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓲𝔃𝓮>)) {
                    self.add_impl((
                        argument1,
                        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<<Self as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
                                      <Self as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝓪𝓻𝓰𝓮𝓽_𝓽𝔂𝓹𝓮,
                                      <Self as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝓪𝓻𝓰𝓮𝓽_𝓽𝔂𝓹𝓮,
                                      <Self as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
                                      i32,
                                      $𝓶𝓮𝓶𝓸𝓻𝔂_𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓲𝔃𝓮> {
                            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: argument2.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
                            𝖻𝖺𝗌𝖾: argument2.𝖻𝖺𝗌𝖾.map(Into::<<Self as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝓪𝓻𝓰𝓮𝓽_𝓽𝔂𝓹𝓮>::into),
                            𝗂𝗇𝖽𝖾𝗑: argument2.𝗂𝗇𝖽𝖾𝗑.map(Into::<<Self as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝓪𝓻𝓰𝓮𝓽_𝓽𝔂𝓹𝓮>::into),
                            𝗌𝖼𝖺𝗅𝖾: argument2.𝗌𝖼𝖺𝗅𝖾,
                            𝖽𝗂𝗌𝗉: argument2.𝖽𝗂𝗌𝗉
                        }))
                }
            }
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt
        𝖋𝖔𝖗𝖜𝖆𝖗𝖉 $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident:
            ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮:ident,
             $𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝔁86:ident<$𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮:ident, $𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮:ident, 0>) =>
            ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝓪𝓻𝓰𝓮𝓽_𝓽𝔂𝓹𝓮:ident, $𝓽𝓸_𝓹𝓸𝓲𝓷𝓽𝓮𝓻_𝓽𝔂𝓹𝓮:ident);
    ) => {
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
            $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
            impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼 $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
            for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼
            impl $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮 {
                fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮(self, (argument1, argument2): (
                        $𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮,
                        $𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝔁86<$𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮, $𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮, 0>)) {
                    self.add_impl((Into::<<Self as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝓪𝓻𝓰𝓮𝓽_𝓽𝔂𝓹𝓮>::into(argument1),
                                  argument2.$𝓽𝓸_𝓹𝓸𝓲𝓷𝓽𝓮𝓻_𝓽𝔂𝓹𝓮()))
                }
            }
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt
        𝖋𝖔𝖗𝖜𝖆𝖗𝖉 $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident:
            ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮:ident, $𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮:ident) =>
            ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝓪𝓻𝓰𝓮𝓽_𝓽𝔂𝓹𝓮:ident, $𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝓪𝓻𝓰𝓮𝓽_𝓽𝔂𝓹𝓮:ident);
    ) => {
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
            $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
            impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼 $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
            for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼
            impl $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮 {
                fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮(self, (argument1, argument2): ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮, $𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮)) {
                    self.add_impl((Into::<<Self as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝓪𝓻𝓰𝓮𝓽_𝓽𝔂𝓹𝓮>::into(argument1),
                                   Into::<<Self as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝓪𝓻𝓰𝓮𝓽_𝓽𝔂𝓹𝓮>::into(argument2)))
                }
            }
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt
        𝖋𝖔𝖗𝖜𝖆𝖗𝖉 $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident:
            $($𝓼𝓸𝓾𝓻𝓬𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼:tt => $𝓽𝓪𝓻𝓰𝓮𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼:tt),*;
        $($𝓸𝓽𝓱𝓮𝓻_𝓽𝓻𝓪𝓲𝓽𝓼:tt)*
    ) => {
        $(  𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
                $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
                impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼 $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
                for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼
                𝖋𝖔𝖗𝖜𝖆𝖗𝖉 $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮 fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:
                    $𝓼𝓸𝓾𝓻𝓬𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼 => $𝓽𝓪𝓻𝓰𝓮𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼;
            }
        )*
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
            $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
            impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼 $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
            for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼
            $($𝓸𝓽𝓱𝓮𝓻_𝓽𝓻𝓪𝓲𝓽𝓼)*
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt)*] $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt)*]
        impl $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident {
            fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident($𝓼𝓮𝓵𝓯_𝓲𝓭𝓮𝓷𝓽:ident,
                                       ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident, $𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident):
                                       ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮:ident, $𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮:ident)) {
                $($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓶𝓹𝓵𝓮𝓶𝓮𝓷𝓽𝓪𝓽𝓲𝓸𝓷:tt)*
            }
        }
    ) => {
        impl$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼)* $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮<
            (<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
             <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮)>
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* {
            type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;
            type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;
            #[inline(always)]
            fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮(
                &mut $𝓼𝓮𝓵𝓯_𝓲𝓭𝓮𝓷𝓽,
                ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽, $𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽): (
                    <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
                    <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮)
            ) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
                $($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓶𝓹𝓵𝓮𝓶𝓮𝓷𝓽𝓪𝓽𝓲𝓸𝓷)*
            }
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt)*] $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt)*]
        impl $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident {
            fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident(
                $𝓼𝓮𝓵𝓯_𝓲𝓭𝓮𝓷𝓽:ident,
                ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident, $𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident):
                ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮:ident,
                 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<$𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮:ident, $𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮:ident, $𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮:literal>)) {
                $($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓶𝓹𝓵𝓮𝓶𝓮𝓷𝓽𝓪𝓽𝓲𝓸𝓷:tt)*
            }
        }
    ) => {
        impl$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼)* $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮<
            (<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
             𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
                         <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮,
                         <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮,
                         i16,
                         $𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>)>
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* {
            type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;
            type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;
            #[inline(always)]
            fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮(
                &mut $𝓼𝓮𝓵𝓯_𝓲𝓭𝓮𝓷𝓽,
                ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽, $𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽): (
                    <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
                    𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
                                <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮,
                                <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮,
                                i16,
                                $𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>)
            ) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
                $($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓶𝓹𝓵𝓮𝓶𝓮𝓷𝓽𝓪𝓽𝓲𝓸𝓷)*
            }
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt)*] $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident[$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt)*]
        impl $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident {
            fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident(
                $𝓼𝓮𝓵𝓯_𝓲𝓭𝓮𝓷𝓽:ident,
                ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident, $𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident):
                ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮:ident,
                 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<$𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮:ident, $𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮:ident, $𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮:literal>)) {
                $($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓶𝓹𝓵𝓮𝓶𝓮𝓷𝓽𝓪𝓽𝓲𝓸𝓷:tt)*
            }
        }
    ) => {
        impl$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼)* $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮<
            (<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
             𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
                         <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮,
                         <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮,
                         <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
                         i32,
                         $𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>)>
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* {
            type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;
            type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;
            #[inline(always)]
            fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮(
                &mut $𝓼𝓮𝓵𝓯_𝓲𝓭𝓮𝓷𝓽,
                ($𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽, $𝓼𝓮𝓬𝓸𝓷𝓭_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽): (
                    <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓯𝓲𝓻𝓼𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
                    𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<<$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
                                <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮,
                                <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::$𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮,
                                <$𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$($𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼)* as $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮>::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
                                i32,
                                $𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>)
            ) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
                $($𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓶𝓹𝓵𝓮𝓶𝓮𝓷𝓽𝓪𝓽𝓲𝓸𝓷)*
            }
        }
    };
    (   $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮:ident$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼:tt
        $(impl $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident {
            fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮:ident$𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷_𝓱𝓮𝓪𝓭𝓮𝓻:tt $𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷_𝓫𝓸𝓭𝔂:tt
        })*
    ) => {
        $(  𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
                $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
                impl$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼 $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
                for $𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓷𝓪𝓶𝓮$𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓹𝓪𝓻𝓪𝓶𝓼
                impl $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮 {
                    fn $𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮$𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷_𝓱𝓮𝓪𝓭𝓮𝓻 $𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷_𝓫𝓸𝓭𝔂
                }
            }
        )*
    }
}

𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖙𝖗𝖆𝖎𝖙! {
    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₐᵥₓ512]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512 {
    }
}

𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖙𝖗𝖚𝖈𝖙! {
    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₐᵥₓ512]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐰𝐢𝐭𝐡_𝐫𝐢𝐳_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷 ₐᵥₓ512]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐰𝐢𝐭𝐡_𝐫𝐢𝐳_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512 {
    }
}

𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16;

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16;

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32;

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32;

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16;

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16;

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32;

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32;

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32;

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₐᵥₓ512]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512;

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐰𝐢𝐭𝐡_𝐫𝐢𝐳_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32;

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷 ₐᵥₓ512]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐰𝐢𝐭𝐡_𝐫𝐢𝐳_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512;
}

pub trait 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 {
    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;
    fn emit_u8(
        &mut self,
        value: u8,
    ) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
    #[inline(always)]
    fn emit_i8(
        &mut self,
        value: i8,
    ) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_u8(value as u8)
    }
    #[inline(always)]
    fn emit_u16(
        &mut self,
        value: u16,
    ) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_u8((value & 0xff) as u8)?;
        self.emit_u8((value >> 8) as u8)
    }
    #[inline(always)]
    fn emit_i16(
        &mut self,
        value: i16,
    ) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_u16(value as u16)
    }
    #[inline(always)]
    fn emit_u32(
        &mut self,
        value: u32,
    ) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_u16((value & 0xffff) as u16)?;
        self.emit_u16((value >> 16) as u16)
    }
    #[inline(always)]
    fn emit_i32(
        &mut self,
        value: i32,
    ) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_u32(value as u32)
    }
    #[inline(always)]
    fn emit_u64(
        &mut self,
        value: u64,
    ) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_u32((value & 0xffffffff) as u32)?;
        self.emit_u32((value >> 32) as u32)
    }
    #[inline(always)]
    fn emit_i64(
        &mut self,
        value: i64,
    ) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        self.emit_u64(value as u64)
    }
}

// We are implementing two-level scheme with a set of ₓₓₓ_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏 traits and a set of ₓₓₓ_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏
// traits.
//
// This allows us to avoid combinatiorial explosion: there are more than dozen of types which may represent just general purpose
// register argument and for three-argument instruction it would mean there are almost two thousand variants.
//
// ₓₓₓ_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏 uses traits below to convert arguments to less diverse set and then ₓₓₓ_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏
// implement the remaining combinations.
//
// If all arguments would be handled identically, then of course, it wouldn't make much sense to even have these two levels.
// Instead some instructions use not 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕 trait, but more specialized conversion traits.
//
// E.g. shift instructions use 𝒔𝒉𝒊𝒇𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕 trait which only support i8 and 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ arguments (and they
// just accept them without conversion).
//
// Thus way we both avoid the combinatorial explosion and guarantee that incorrect registers are excluded during the compilation
// time. Not only this makes debugging easier, this also means that we can stol thinking about reporting these particular errors.
//
// Note: Even with this approach we have some extra variants to implement (e.g. add have separate version for accumulator and
// 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫 and 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 because there are special version for 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫 and immediate, but overral it's less than 2x
// superfluous instructions.  This is considered acceptable.

pub trait 𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝓾𝓹𝓵𝓮> {
    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;
    fn add_impl(
        &mut self,
        arguments: 𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝓾𝓹𝓵𝓮,
    ) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
}

pub trait 𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝓾𝓹𝓵𝓮> {
    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;
    fn add(
        &mut self,
        arguments: 𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽_𝓽𝓾𝓹𝓵𝓮,
    ) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, 𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽1_𝓽𝔂𝓹𝓮, 𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽2_𝓽𝔂𝓹𝓮> 𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<(𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽1_𝓽𝔂𝓹𝓮, 𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽2_𝓽𝔂𝓹𝓮)>
for 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮
where 𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽1_𝓽𝔂𝓹𝓮: 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
      𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽2_𝓽𝔂𝓹𝓮: 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
      𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<
          (<𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽1_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
           <𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽2_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭)>
{
    #[allow(clippy::type_complexity)]
    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = <𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as 𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<
        (<𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽1_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
         <𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽2_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭)>>::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;
    #[allow(clippy::type_complexity)]
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = <𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮 as 𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒎𝒑𝒍𝒆𝒎𝒆𝒏𝒕𝒂𝒕𝒊𝒐𝒏<
        (<𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽1_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
         <𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽2_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭)>>::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;
    #[inline(always)]
    fn add(&mut self, (argument1, argument2): (𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽1_𝓽𝔂𝓹𝓮, 𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽2_𝓽𝔂𝓹𝓮)) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞> {
        let argument1_impl: <𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽1_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭 = argument1.into();
        let argument2_impl: <𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽2_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭 = argument2.into();
        self.add_impl((argument1_impl, argument2_impl))
    }
}

pub trait 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>: Sized
where Self::𝐭𝐚𝐫𝐠𝐞𝐭: From<Self>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

// Immediates are always signed on 𝔵86.
impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for i8
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = i8;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for i16
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = i16;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for i32
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = i32;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for i64
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = i64;
}

#[allow(non_upper_case_globals)]
impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>;
}

#[allow(non_upper_case_globals)]
impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: Default, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>;
}

#[allow(non_upper_case_globals)]
impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
     𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮: 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮: 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Into<i16>,
     const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
    where Option<<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮 as 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>: From<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
          Option<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>: From<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
          Option<<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>: From<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>
{
    #[allow(clippy::type_complexity)]
    type 𝐭𝐚𝐫𝐠𝐞𝐭 =
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮 as 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                    <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                    <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                    i16,
                    𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>;
}

#[allow(non_upper_case_globals)]
impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
     𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮: 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮: 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒄𝒂𝒍𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Into<i32>,
     const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
    where Option<<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮 as 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>: From<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
          Option<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>: From<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
          Option<<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>: From<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>
{
    #[allow(clippy::type_complexity)]
    type 𝐭𝐚𝐫𝐠𝐞𝐭 =
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮 as 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                    <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                    <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                    <𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮 as 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒄𝒂𝒍𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                    i32,
                    𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>;
}

pub trait 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>: Sized
where Option<Self::𝐭𝐚𝐫𝐠𝐞𝐭>: From<Self>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

pub trait 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>: Sized
where Option<Self::𝐭𝐚𝐫𝐠𝐞𝐭>: From<Self>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

pub trait 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>: Sized
where Option<Self::𝐭𝐚𝐫𝐠𝐞𝐭>: From<Self>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

pub trait 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒄𝒂𝒍𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>: Sized
where Self::𝐭𝐚𝐫𝐠𝐞𝐭: From<Self>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭: Default;
}

// Same as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏 but accumulator (𝔞𝔩, 𝔞𝔵, 𝔢𝔞𝔵, 𝔯𝔞𝔵, but not 𝔞𝔥) is not merged with other registers.
// Note: it's variant of 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏, and not 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕 because of peculiarity of the
// 𝔵86 ISA: all instructions with special accumulator-only encoding (for explicit, not implicit argument!) also have 8ᵇⁱᵗ version.
pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<
    𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
>: Sized where Self::𝐭𝐚𝐫𝐠𝐞𝐭: From<Self>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

// Immediates are always signed on 𝔵86.
impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for i8 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = i8;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for i16 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = i16;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for i32 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = i32;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for i64 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = i64;
}

#[allow(non_upper_case_globals)]
impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>;
}

#[allow(non_upper_case_globals)]
impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: Default, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>;
}

#[allow(non_upper_case_globals)]
impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
     𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮: 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮: 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Into<i16>,
     const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
    where Option<<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>: From<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
          Option<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>: From<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
          Option<<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>: From<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
          <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭:
              𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>,
          <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭:
              𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>,
          Option<<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                  as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
                                     <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭>:
              From<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
          Option<<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                  as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
                                     <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭>:
              From<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>,
          Option<<<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                  as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
                                      <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭>:
              From<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
          Option<<<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                  as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
                                      <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭>:
              From<<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>
{
    #[allow(clippy::type_complexity)]
    type 𝐭𝐚𝐫𝐠𝐞𝐭 =
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                     <<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                      as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                     <<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                      as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                     i16,
                     𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>;
}

#[allow(non_upper_case_globals)]
impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
     𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮: 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮: 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒄𝒂𝒍𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Into<i32>,
     const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
    where Option<<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>: From<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
          Option<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>: From<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
          Option<<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>: From<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
          <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭:
              𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>,
          <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭:
              𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>,
          Option<<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                  as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
                                     <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭>:
              From<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
          Option<<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                  as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
                                     <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭>:
              From<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>,
          Option<<<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                  as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
                                      <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭>:
              From<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
          Option<<<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                  as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
                                      <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭>:
              From<<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>
{
    #[allow(clippy::type_complexity)]
    type 𝐭𝐚𝐫𝐠𝐞𝐭 =
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                    <<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                     as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                    <<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                     as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                    <𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒄𝒂𝒍𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                    i32,
                    𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>;
}

// In legacy modes this is the same as 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕. But in 64ᵇⁱᵗ mode situation is significantly more complex:
// not only there are three separate register clases (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₗₒ, 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ, and other 8ᵇⁱᵗ registers), this
// also affects other arguments!  E.g. address must use 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_XXᵇⁱᵗₙₒᵣₑₓ for base and 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_XXᵇⁱᵗₙₒᵣₑₓ
// for index if other argument is of 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗₕᵢ class.
//
// Thankfully most modern instructions (like 3-operand 𝔞𝔡𝔠𝔵 or 𝔟𝔢𝔵𝔱𝔯 instructions) don't have an 8ᵇⁱᵗ variants.
//
// Note: for instructions which 𝗱𝗼 have an 8-bit variants (like 𝔞𝔡𝔡 or 𝔰𝔲𝔟) we have to use that argument even if we are not
// dealing with 8ᵇⁱᵗ instructions.  This is because 𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏 and 𝒂𝒅𝒅_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏 traits are used
// before we know if we are dealing with 8ᵇⁱᵗ arguments or not.

pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
    𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
>: Sized where Self::𝐭𝐚𝐫𝐠𝐞𝐭: From<Self>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

// Immediates are always signed on 𝔵86.
impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for i8 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = i8;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for i16 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = i16;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for i32 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = i32;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for i64 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = i64;
}

#[allow(non_upper_case_globals)]
impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>;
}

#[allow(non_upper_case_globals)]
impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: Default, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>;
}

#[allow(non_upper_case_globals)]
impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
     𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮: 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮: 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Into<i16>,
     const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
    where Option<<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>: From<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
          Option<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>: From<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
          Option<<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>: From<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
          <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭:
              𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>,
          <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭:
              𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>,
          Option<<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                  as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
                                     <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭>:
              From<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
          Option<<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                  as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
                                     <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭>:
              From<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>,
          Option<<<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                  as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
                                      <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭>:
              From<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
          Option<<<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                  as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
                                      <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭>:
              From<<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>
{
    #[allow(clippy::type_complexity)]
    type 𝐭𝐚𝐫𝐠𝐞𝐭 =
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                     <<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                      as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                     <<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                      as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                     i16,
                     𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>;
}

#[allow(non_upper_case_globals)]
impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
     𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮: 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮: 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒄𝒂𝒍𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Into<i32>,
     const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
    where Option<<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>: From<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
          Option<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>: From<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
          Option<<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>: From<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
          <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭:
              𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>,
          <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭:
              𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>,
          Option<<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                  as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
                                     <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭>:
              From<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
          Option<<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                  as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
                                     <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭>:
              From<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>,
          Option<<<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                  as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
                                      <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭>:
              From<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
          Option<<<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                  as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
                                      <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭>:
              From<<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>
{
    #[allow(clippy::type_complexity)]
    type 𝐭𝐚𝐫𝐠𝐞𝐭 =
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                    <<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                     as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, <𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                    <<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭
                     as 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, <𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                    <𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮 as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒄𝒂𝒍𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                    i32,
                    𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>;
}

pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
    𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
>: Sized where Option<Self::𝐭𝐚𝐫𝐠𝐞𝐭>: From<Self>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
    𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
>: Sized where Option<Self::𝐭𝐚𝐫𝐠𝐞𝐭>: From<Self>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒊𝒏𝒅𝒆𝒙_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
    𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
>: Sized where Option<Self::𝐭𝐚𝐫𝐠𝐞𝐭>: From<Self>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒔𝒄𝒂𝒍𝒆_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
    𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮,
>: Sized where Self::𝐭𝐚𝐫𝐠𝐞𝐭: From<Self>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭: Default;
}

// Additional step for the ᵣₑₓ/ₙₒᵣₑₓ types handling: we want to provide two types: ₙₒᵣₑₓ compliant and ᵣₑₓ-requiring.
// If base is ₙₒᵣₑₓ but index is ᵣₑₓ rhen we make base ᵣₑₓ. Same with index.
pub trait 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>: Sized
where Option<Self::𝐭𝐚𝐫𝐠𝐞𝐭>: From<Self>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

pub trait 𝒓𝒆𝒙_𝒆𝒙𝒑𝒂𝒏𝒅_𝒊𝒏𝒅𝒆𝒙<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>: Sized
where Option<Self::𝐭𝐚𝐫𝐠𝐞𝐭>: From<Self>
{
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

// Marker trait to prevenct conflict for Option<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞>: should it be Some(𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞{}) or None?
pub trait 𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆 {}

// Address includes some values which are optional and can be unfilled. We use empty type to mark these.
// Note: we can not use just an empty tuple because then we couldn't define From trait for it.
// Note2: scale and displacement must be obtainable from 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞.  Integer types are obtainable automatically.
pub struct 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {}
// All assembler support two addresses: 16ᵇⁱᵗ and 32ᵇⁱᵗ in legacy mode or 32ᵇⁱᵗ and 64ᵇⁱᵗ in ₓ86_64 mode.
// We provide different address constants for these three modes — that way there are no ambiguit even if simple 𝔞𝔡𝔡𝔯𝔢𝔰𝔰 [0] is used.
pub struct 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ {} // We don't really need that because of 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086 vs 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86 difference. Maybe remove?
pub struct 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ {}
pub struct 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ {}

// Address type is just a combination of arguments and it's mostly used to simplify interface. There are three address types:
//   1. Legacy 8086 address — segment, base, index and 16ᵇⁱᵗ displacement.
//      All optional except displacemet (which can be zero).
//   2. Modern 32ᵇⁱᵗ/64ᵇⁱᵗ address — segment, base, index, scale and 32ᵇⁱᵗ displacement.
//      All optional except for scale (which can be 1 and that's default) and displacement (which can be zero).
//   3. Gather address — segment, base, index, scale and 32ᵇⁱᵗ displacement.
//      Similar to previous one, but index is vector register and, more importantly, it's not optional.
#[allow(non_upper_case_globals)]
#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086
<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: Option<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
    pub 𝖻𝖺𝗌𝖾: Option<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
    pub 𝗂𝗇𝖽𝖾𝗑: Option<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
    pub 𝖽𝗂𝗌𝗉: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
}

impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default>
    𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 0>
{
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_byte_ptr(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 1> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 1> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_word_ptr(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 2> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 2> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_dword_ptr(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 4> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 4> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_qword_ptr(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 8> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 8> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    // We couldn't distingush far pointer from near pointer by size since 16ᵇⁱᵗ far pointer have the same size as 32ᵇⁱᵗ near pointer
    // and they both can be used in 16ᵇⁱᵗ and 32ᵇⁱᵗ modes.  Use negative sizes instead.
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_far_ptr16(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { usize::MAX - 3 }> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { usize::MAX - 3 }> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_far_ptr32(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { usize::MAX - 5 }> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { usize::MAX - 5 }> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    // Generic memory region. Mostly for completeness because all instructions which use arbitrarily sized regions don't need that
    // for disambigution.
    #[allow(clippy::wrong_self_convention)]
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    pub fn to_sized_ptr<const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>,
        const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize,
    >
    From<
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        >,
    > for 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
where
    Option<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>: From<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
    Option<𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>: From<𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
    Option<𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>: From<𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
{
    #[inline(always)]
    fn from(
        new_address: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        >,
    ) -> Self {
        Self {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_address.𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into(),
            𝖻𝖺𝗌𝖾: new_address.𝖻𝖺𝗌𝖾.into(),
            𝗂𝗇𝖽𝖾𝗑: new_address.𝗂𝗇𝖽𝖾𝗑.into(),
            𝖽𝗂𝗌𝗉: new_address.𝖽𝗂𝗌𝗉.into(),
        }
    }
}

#[allow(non_upper_case_globals)]
#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86
<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: Default, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: Option<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
    pub 𝖻𝖺𝗌𝖾: Option<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
    pub 𝗂𝗇𝖽𝖾𝗑: Option<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
    pub 𝗌𝖼𝖺𝗅𝖾: 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
    pub 𝖽𝗂𝗌𝗉: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
}

impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: Default, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default>
    𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 0>
{
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_byte_ptr(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 1> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 1> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_word_ptr(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 2> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 2> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_dword_ptr(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 4> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 4> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_qword_ptr(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 8> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 8> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    // We couldn't distingush far pointer from near pointer by size since 16ᵇⁱᵗ far pointer have the same size as 32ᵇⁱᵗ near pointer
    // and they both can be used in 16ᵇⁱᵗ and 32ᵇⁱᵗ modes.  Use negative sizes instead.
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_far_ptr32(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { usize::MAX - 5 }> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { usize::MAX - 5 }> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_far_ptr64(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { usize::MAX - 9 }> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, { usize::MAX - 9 }> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    // Generic memory region. Mostly for completeness because all instructions which use arbitrarily sized regions don't need that
    // for disambigution.
    #[allow(clippy::wrong_self_convention)]
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    pub fn to_sized_ptr<const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>(
        self,
    ) -> 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: Default,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮>,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>,
        const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize,
    >
    From<
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        >,
    >
    for 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
where
    Option<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>: From<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
    Option<𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>: From<𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
    Option<𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>: From<𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
{
    #[inline(always)]
    fn from(
        new_address: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        >,
    ) -> Self {
        Self {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_address.𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into(),
            𝖻𝖺𝗌𝖾: new_address.𝖻𝖺𝗌𝖾.into(),
            𝗂𝗇𝖽𝖾𝗑: new_address.𝗂𝗇𝖽𝖾𝗑.into(),
            𝗌𝖼𝖺𝗅𝖾: new_address.𝗌𝖼𝖺𝗅𝖾.into(),
            𝖽𝗂𝗌𝗉: new_address.𝖽𝗂𝗌𝗉.into(),
        }
    }
}

#[allow(non_upper_case_globals)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔
<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: Default, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: Option<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
    pub 𝖻𝖺𝗌𝖾: Option<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
    pub 𝗂𝗇𝖽𝖾𝗑: 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
    pub 𝗌𝖼𝖺𝗅𝖾: 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
    pub 𝖽𝗂𝗌𝗉: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
}

#[allow(non_upper_case_globals)]
impl<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: Default,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮>,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>,
        const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize,
    >
    From<
        𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        >,
    >
    for 𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
where
    Option<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>: From<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
    Option<𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>: From<𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
    𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮: From<𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
{
    #[inline(always)]
    fn from(
        new_address: 𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        >,
    ) -> Self {
        Self {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_address.𝗌𝖾𝗀𝗆𝖾𝗇𝗍.into(),
            𝖻𝖺𝗌𝖾: new_address.𝖻𝖺𝗌𝖾.into(),
            𝗂𝗇𝖽𝖾𝗑: new_address.𝗂𝗇𝖽𝖾𝗑.into(),
            𝗌𝖼𝖺𝗅𝖾: new_address.𝗌𝖼𝖺𝗅𝖾.into(),
            𝖽𝗂𝗌𝗉: new_address.𝖽𝗂𝗌𝗉.into(),
        }
    }
}

#[allow(non_upper_case_globals)]
pub const 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_16ᵇⁱᵗ: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    0,
> = 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086 {
    𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
    𝖻𝖺𝗌𝖾: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ {},
    𝗂𝗇𝖽𝖾𝗑: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ {},
    𝖽𝗂𝗌𝗉: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
};

#[allow(non_upper_case_globals)]
pub const 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    0,
> = 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86 {
    𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
    𝖻𝖺𝗌𝖾: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ {},
    𝗂𝗇𝖽𝖾𝗑: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ {},
    𝗌𝖼𝖺𝗅𝖾: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
    𝖽𝗂𝗌𝗉: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
};

#[allow(non_upper_case_globals)]
pub const 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_64ᵇⁱᵗ: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    0,
> = 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86 {
    𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
    𝖻𝖺𝗌𝖾: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ {},
    𝗂𝗇𝖽𝖾𝗑: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ {},
    𝗌𝖼𝖺𝗅𝖾: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
    𝖽𝗂𝗌𝗉: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
};

// Fluent interface requires the ability to find out type from arguments: Rust doesn't try to do complex pruning when you have
// something like 𝔞𝔡𝔡𝔯𝔢𝔰𝔰.with_base(…).with_index(…).with_disp(…).
//
// Collect all arguments into 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_{8086,ₓ86} instead and then provide conversions into proper addess.
#[allow(non_upper_case_globals)]
#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086
<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    pub 𝖻𝖺𝗌𝖾: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    pub 𝗂𝗇𝖽𝖾𝗑: 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
    pub 𝖽𝗂𝗌𝗉: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
}

#[allow(non_upper_case_globals)]
#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86
<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    pub 𝖻𝖺𝗌𝖾: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    pub 𝗂𝗇𝖽𝖾𝗑: 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
    pub 𝗌𝖼𝖺𝗅𝖾: 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
    pub 𝖽𝗂𝗌𝗉: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
}

// Note: 𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 have the same structure as 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86 but index is not optioal.
// You have to use .with_index or code wouldn't compile.
#[allow(non_upper_case_globals)]
#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔
<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    pub 𝖻𝖺𝗌𝖾: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    pub 𝗂𝗇𝖽𝖾𝗑: 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
    pub 𝗌𝖼𝖺𝗅𝖾: 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
    pub 𝖽𝗂𝗌𝗉: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
}

#[allow(non_upper_case_globals)]
impl<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮>
{
    #[inline(always)]
    pub fn with_segment<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>(
        self,
        new_segment: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_segment,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
{
    #[inline(always)]
    pub fn with_base<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>(
        self,
        new_base: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: new_base,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
{
    #[inline(always)]
    pub fn with_index<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>(
        self,
        new_index: 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: new_index,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
{
    #[inline(always)]
    pub fn with_disp<𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>(
        self,
        new_disp: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: new_disp,
        }
    }
}

impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 0>
{
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_byte_ptr(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 1> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 1> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_word_ptr(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 2> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 2> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_dword_ptr(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 4> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 4> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_qword_ptr(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 8> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 8> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    // We couldn't distingush far pointer from near pointer by size since 16ᵇⁱᵗ far pointer have the same size as 32ᵇⁱᵗ near pointer
    // and they both can be used in 16ᵇⁱᵗ and 32ᵇⁱᵗ modes.  Use negative sizes instead.
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_far_ptr16(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        { usize::MAX - 3 },
    > {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<
            𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            { usize::MAX - 3 },
        > {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_far_ptr32(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        { usize::MAX - 5 },
    > {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<
            𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            { usize::MAX - 5 },
        > {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
    // Generic memory region. Mostly for completeness because all instructions which use arbitrarily sized regions don't need that
    // for disambigution.
    #[allow(clippy::wrong_self_convention)]
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    pub fn to_sized_ptr<const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
{
    #[inline(always)]
    pub fn with_segment<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>(
        self,
        new_segment: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    > {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<
            𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        > {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_segment,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
{
    #[inline(always)]
    pub fn with_base<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>(
        self,
        new_base: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    > {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<
            𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        > {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: new_base,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
{
    #[inline(always)]
    pub fn with_base<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>(
        self,
        new_base: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    > {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<
            𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        > {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: new_base,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
{
    #[inline(always)]
    pub fn with_index<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>(
        self,
        new_index: 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    > {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<
            𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        > {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: new_index,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
{
    #[inline(always)]
    pub fn with_index<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>(
        self,
        new_index: 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    > {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<
            𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        > {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: new_index,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
{
    #[inline(always)]
    pub fn with_scale<𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮>(
        self,
        new_scale: 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    > {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<
            𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        > {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: new_scale,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
{
    #[inline(always)]
    pub fn with_disp<𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>(
        self,
        new_disp: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    > {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<
            𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        > {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: new_disp,
        }
    }
}

impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 0>
{
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_byte_ptr(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 1> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 1> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_word_ptr(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 2> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 2> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_dword_ptr(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 4> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 4> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_qword_ptr(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 8> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, 8> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    // We couldn't distingush far pointer from near pointer by size since 16ᵇⁱᵗ far pointer have the same size as 32ᵇⁱᵗ near pointer
    // and they both can be used in 16ᵇⁱᵗ and 32ᵇⁱᵗ modes.  Use negative sizes instead.
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_far_ptr32(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        { usize::MAX - 5 },
    > {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<
            𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            { usize::MAX - 5 },
        > {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_far_ptr64(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        { usize::MAX - 9 },
    > {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<
            𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            { usize::MAX - 9 },
        > {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
    // Generic memory region. Mostly for completeness because all instructions which use arbitrarily sized regions don't need that
    // for disambigution.
    #[allow(clippy::wrong_self_convention)]
    #[allow(non_upper_case_globals)]
    #[inline(always)]
    pub fn to_sized_ptr<const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>(
        self,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    > {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<
            𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        > {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
{
    #[inline(always)]
    pub fn with_segment<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>(
        self,
        new_segment: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    ) -> 𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    > {
        𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<
            𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        > {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_segment,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
{
    #[inline(always)]
    pub fn with_base<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>(
        self,
        new_base: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    ) -> 𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    > {
        𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<
            𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        > {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: new_base,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
{
    #[inline(always)]
    pub fn with_base<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>(
        self,
        new_base: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    ) -> 𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    > {
        𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<
            𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        > {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: new_base,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
{
    #[inline(always)]
    pub fn with_index<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>(
        self,
        new_index: 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
    ) -> 𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    > {
        𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<
            𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        > {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: new_index,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
{
    #[inline(always)]
    pub fn with_scale<𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮>(
        self,
        new_scale: 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
    ) -> 𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    > {
        𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<
            𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        > {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: new_scale,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, const 𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮: usize>
    𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    >
{
    #[inline(always)]
    pub fn with_disp<𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>(
        self,
        new_disp: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
    ) -> 𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
    > {
        𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<
            𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
            𝓶𝓮𝓶𝓸𝓻𝔂_𝓼𝓲𝔃𝓮,
        > {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: new_disp,
        }
    }
}

macro_rules! 𝖉𝖊𝖋𝖎𝖓𝖊_𝖎𝖓𝖙𝖊𝖌𝖊𝖗_𝖋𝖗𝖔𝖒 {
    ($($𝓲𝓷𝓽_𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮:ident),*) => {
        $(
            impl From<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞> for $𝓲𝓷𝓽_𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                #[inline(always)]
                fn from(_: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞) -> $𝓲𝓷𝓽_𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                    0
                }
            }
         )*
    }
}

𝖉𝖊𝖋𝖎𝖓𝖊_𝖎𝖓𝖙𝖊𝖌𝖊𝖗_𝖋𝖗𝖔𝖒!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);

#[cfg(has_i128)]
𝖉𝖊𝖋𝖎𝖓𝖊_𝖎𝖓𝖙𝖊𝖌𝖊𝖗_𝖋𝖗𝖔𝖒!(i128, u128);

impl<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮> From<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞> for core::num::Wrapping<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>
where 𝓲𝓷𝓽_𝓽𝔂𝓹𝓮: From<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞>
{
    #[inline(always)]
    fn from(value: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞) -> Self {
        Self(value.into())
    }
}

// Any 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 type can be converted into the appropriate Option (and will end up as None, of course).
impl<𝓪𝓻𝓫𝓲𝓽𝓻𝓪𝓻𝔂_𝓽𝔂𝓹𝓮: 𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆> From<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞> for Option<𝓪𝓻𝓫𝓲𝓽𝓻𝓪𝓻𝔂_𝓽𝔂𝓹𝓮> {
    #[inline(always)]
    fn from(_: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞) -> Self {
        None
    }
}

impl<𝓪𝓻𝓫𝓲𝓽𝓻𝓪𝓻𝔂_𝓽𝔂𝓹𝓮: 𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆> From<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ> for Option<𝓪𝓻𝓫𝓲𝓽𝓻𝓪𝓻𝔂_𝓽𝔂𝓹𝓮> {
    #[inline(always)]
    fn from(_: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_16ᵇⁱᵗ) -> Self {
        None
    }
}

impl<𝓪𝓻𝓫𝓲𝓽𝓻𝓪𝓻𝔂_𝓽𝔂𝓹𝓮: 𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆> From<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ> for Option<𝓪𝓻𝓫𝓲𝓽𝓻𝓪𝓻𝔂_𝓽𝔂𝓹𝓮> {
    #[inline(always)]
    fn from(_: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_32ᵇⁱᵗ) -> Self {
        None
    }
}

impl<𝓪𝓻𝓫𝓲𝓽𝓻𝓪𝓻𝔂_𝓽𝔂𝓹𝓮: 𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆> From<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ> for Option<𝓪𝓻𝓫𝓲𝓽𝓻𝓪𝓻𝔂_𝓽𝔂𝓹𝓮> {
    #[inline(always)]
    fn from(_: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞_64ᵇⁱᵗ) -> Self {
        None
    }
}

#[path = "basic_assembler.rs"]
pub mod 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿;

#[cfg(test)]
struct 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
    𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec<u8>,
}

#[cfg(test)]
impl 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
    pub const fn new() -> Self {
        𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
            𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec::<u8>::new(),
        }
    }
}

#[cfg(test)]
impl 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
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
