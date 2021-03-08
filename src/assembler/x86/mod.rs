macro_rules! 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖆𝖗𝖐𝖊𝖗_𝖙𝖗𝖆𝖎𝖙𝖘 {
    ($($𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident),*) => {
        $(
            pub trait $𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮<𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮>
            where Self: From<𝓼𝓸𝓾𝓻𝓬𝓮_𝓽𝔂𝓹𝓮>
            {
            }
            impl<T: 𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆>
                $𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞> for Option<T>
            {
            }
         )*
    }
}

// Marker traits for function arguments. Actual conversion uses Into<𝓽𝔂𝓹𝓮>.
𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖆𝖗𝖐𝖊𝖗_𝖙𝖗𝖆𝖎𝖙𝖘! {
    𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓,
    𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓,
    // Some instructions have special version when accumulator register is used.
    𝒊𝒔_𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒏𝒐_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓,
    𝒊𝒔_𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓,
    𝒊𝒔_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓,
    𝒊𝒔_𝒙𝟖𝟕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓,
    𝒊𝒔_𝒎𝒂𝒔𝒌_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓,
    𝒊𝒔_𝑺𝑰𝑴𝑫_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓
}

// Marker trait to prevenct conflict for Option<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞>: should it be Some(𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞{}) or None?
pub trait 𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆 {}

// Address includes some values which are optional and can be unfilled. We use empty type to mark these.
// Note: we can not use just an empty tuple because then we couldn't define From trait for it.
// Note2: scale and displacement must be obtainable from 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞.  Integer types are obtainable automatically.
pub struct 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {}

// Address type is just a combination of arguments and it's mostly used to simplify interface. There are three three address types:
//   1. Legacy 8086 address — segment, base, index and [16-bit] displacement.
//      All optional except displacemet (which can be zero).
//   2. Modern 32ᵇⁱᵗ/64ᵇⁱᵗ address — segment, base, index, scale and [32-bit] displacement.
//      All optional except for scale (which can be 1 and that's default) and displacement (which can be zero).
//   3. Gather address — segment, base, index, scale and [32-bit] displacement.
//      Similar to previous one, but index is vector register and, more importantly, it's not optional.
#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086
<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: Option<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
    pub 𝖻𝖺𝗌𝖾: Option<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
    pub 𝗂𝗇𝖽𝖾𝗑: Option<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
    pub 𝖽𝗂𝗌𝗉: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
}

impl<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>,
    >
    From<
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        >,
    > for 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>
where
    Option<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>: 𝒊𝒔_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
    Option<𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>:
        𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
    Option<𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>:
        𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
{
    #[inline(always)]
    fn from(
        new_address: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
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

#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86
<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: Default, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: Option<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
    pub 𝖻𝖺𝗌𝖾: Option<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
    pub 𝗂𝗇𝖽𝖾𝗑: Option<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
    pub 𝗌𝖼𝖺𝗅𝖾: 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
    pub 𝖽𝗂𝗌𝗉: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
}

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
    >
    From<
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        >,
    >
    for 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
    >
where
    Option<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>: 𝒊𝒔_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
    Option<𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>:
        𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
    Option<𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>:
        𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
{
    #[inline(always)]
    fn from(
        new_address: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔
<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮: Default, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: Option<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
    pub 𝖻𝖺𝗌𝖾: Option<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
    pub 𝗂𝗇𝖽𝖾𝗑: 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
    pub 𝗌𝖼𝖺𝗅𝖾: 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
    pub 𝖽𝗂𝗌𝗉: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
}

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
    >
    From<
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
        >,
    >
    for 𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
    >
where
    Option<𝓽𝓪𝓻𝓰𝓮𝓽_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>: 𝒊𝒔_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>,
    Option<𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>:
        𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
    𝓽𝓪𝓻𝓰𝓮𝓽_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮:
        𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>,
{
    #[inline(always)]
    fn from(
        new_address: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
            𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
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
pub const 𝔞𝔡𝔡𝔯𝔢𝔰𝔰: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
    𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞,
> = 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 {
    𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
    𝖻𝖺𝗌𝖾: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
    𝗂𝗇𝖽𝖾𝗑: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
    𝗌𝖼𝖺𝗅𝖾: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
    𝖽𝗂𝗌𝗉: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
};

// Fluent interface requires the ability to find out type from arguments: Rust doesn't try to do complex pruning when you have
// something like 𝔞𝔡𝔡𝔯𝔢𝔰𝔰.with_base(…).with_index(…).with_disp(…).
//
// Collect all arguments into 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 instead and then provide conversions into proper addess.
#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔
<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
    pub 𝗌𝖾𝗀𝗆𝖾𝗇𝗍: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    pub 𝖻𝖺𝗌𝖾: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    pub 𝗂𝗇𝖽𝖾𝗑: 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
    pub 𝗌𝖼𝖺𝗅𝖾: 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
    pub 𝖽𝗂𝗌𝗉: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
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

impl<𝓪𝓻𝓫𝓲𝓽𝓻𝓪𝓻𝔂_𝓽𝔂𝓹𝓮: 𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆> From<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞> for Option<𝓪𝓻𝓫𝓲𝓽𝓻𝓪𝓻𝔂_𝓽𝔂𝓹𝓮> {
    #[inline(always)]
    fn from(_: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞) -> Self {
        None
    }
}

impl<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>
{
    #[inline(always)]
    pub fn with_segment<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮>(
        self,
        new_segment: 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: new_segment,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>
{
    #[inline(always)]
    pub fn with_base<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>(
        self,
        new_base: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: new_base,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>
{
    #[inline(always)]
    pub fn with_index<𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮>(
        self,
        new_index: 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: new_index,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>
{
    #[inline(always)]
    pub fn with_scale<𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮>(
        self,
        new_scale: 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: new_scale,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

impl<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮>
    𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞>
{
    #[inline(always)]
    pub fn with_disp<𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>(
        self,
        new_disp: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
    ) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓽𝔂𝓹𝓮, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮, 𝓼𝓬𝓪𝓵𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
            𝗌𝖾𝗀𝗆𝖾𝗇𝗍: self.𝗌𝖾𝗀𝗆𝖾𝗇𝗍,
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝗂𝗇𝖽𝖾𝗑: self.𝗂𝗇𝖽𝖾𝗑,
            𝗌𝖼𝖺𝗅𝖾: self.𝗌𝖼𝖺𝗅𝖾,
            𝖽𝗂𝗌𝗉: new_disp,
        }
    }
}

pub mod 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿 {

    macro_rules! 𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖞𝖋𝖗𝖔𝖒_𝖋𝖔𝖗_𝖎𝖓𝖙 {
        ($𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮:ident {$𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷:expr}) => {
            #[cfg(test)]
            impl 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮> {
                fn test_i8() {
                }
            }
        };
        ($𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮:ident {$𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷:expr} $({$𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴:expr})+) => {
            impl std::convert::TryFrom<i8> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = ();
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: i8) -> Result<Self, Self::Error> {
                     $(if ($𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴)(value as u8) {return Err(())})*
                     // Note: we are using repr(i8) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { std::mem::transmute::<i8, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value)) })
                }
            }
            impl std::convert::TryFrom<u8> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = ();
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: u8) -> Result<Self, Self::Error> {
                     $(if ($𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴)(value) {return Err(())})*
                     // Note: we are using repr(i8) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { std::mem::transmute::<u8, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value)) })
                }
            }

            impl std::convert::TryFrom<i16> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = ();
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: i16) -> Result<Self, Self::Error> {
                     $(if ($𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴)(value as u16) {return Err(())})*
                     // Note: we are using repr(i8) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { std::mem::transmute::<i8, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as i8)) })
                }
            }
            impl std::convert::TryFrom<u16> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = ();
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: u16) -> Result<Self, Self::Error> {
                     $(if ($𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴)(value) {return Err(())})*
                     // Note: we are using repr(i8) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { std::mem::transmute::<u8, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as u8)) })
                }
            }

            impl std::convert::TryFrom<i32> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = ();
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: i32) -> Result<Self, Self::Error> {
                     $(if ($𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴)(value as u32) {return Err(())})*
                     // Note: we are using repr(i8) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { std::mem::transmute::<i8, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as i8)) })
                }
            }
            impl std::convert::TryFrom<u32> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = ();
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: u32) -> Result<Self, Self::Error> {
                     $(if ($𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴)(value) {return Err(())})*
                     // Note: we are using repr(i8) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { std::mem::transmute::<u8, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as u8)) })
                }
            }

            impl std::convert::TryFrom<i64> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = ();
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: i64) -> Result<Self, Self::Error> {
                     $(if ($𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴)(value as u64) {return Err(())})*
                     // Note: we are using repr(i8) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { std::mem::transmute::<i8, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as i8)) })
                }
            }
            impl std::convert::TryFrom<u64> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = ();
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: u64) -> Result<Self, Self::Error> {
                     $(if ($𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴)(value) {return Err(())})*
                     // Note: we are using repr(i8) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { std::mem::transmute::<u8, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as u8)) })
                }
            }

            #[cfg(has_i128)]
            impl std::convert::TryFrom<i128> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = ();
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: i128) -> Result<Self, Self::Error> {
                     $(if ($𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴)(value as u128) {return Err(())})*
                     // Note: we are using repr(i8) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { std::mem::transmute::<i8, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as i8)) })
                }
            }
            #[cfg(has_i128)]
            impl std::convert::TryFrom<u128> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = ();
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: u128) -> Result<Self, Self::Error> {
                     $(if ($𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴)(value) {return Err(())})*
                     // Note: we are using repr(i8) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { std::mem::transmute::<u8, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as u8)) })
                }
            }

            impl std::convert::TryFrom<isize> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = ();
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: isize) -> Result<Self, Self::Error> {
                     $(if ($𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴)(value as usize) {return Err(())})*
                     // Note: we are using repr(i8) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { std::mem::transmute::<i8, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as i8)) })
                }
            }
            impl std::convert::TryFrom<usize> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                type Error = ();
                #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                #[inline(always)]
                fn try_from(value: usize) -> Result<Self, Self::Error> {
                     $(if ($𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴)(value) {return Err(())})*
                     // Note: we are using repr(i8) here thus conversion is safe and it doesn't disable any optimizations:
                     //   Result<Self, Self::Error> is still one byte in size.
                     Ok(unsafe { std::mem::transmute::<u8, $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>(($𝓲𝓷𝓽_𝓬𝓸𝓷𝓿𝓮𝓻𝓼𝓲𝓸𝓷)(value as u8)) })
                }
            }

            impl<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮> std::convert::TryFrom<core::num::Wrapping<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>> for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 where $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮: std::convert::TryFrom<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮> {
                type Error = <$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 as std::convert::TryFrom<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>>::Error;
                #[inline(always)]
                fn try_from(value: core::num::Wrapping<𝓲𝓷𝓽_𝓽𝔂𝓹𝓮>) -> Result<Self, Self::Error> {
                    Self::try_from(value.0)
                }
            }

            #[cfg(test)]
            impl 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮> {
                #[allow(dead_code)]
                fn test_i8() {
                    for value in i8::MIN..=i8::MAX {
                      assert_eq!(std::convert::TryInto::<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>::try_into(value).ok(),
                                 std::convert::TryInto::<𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕::<$𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮>>::try_into(value).ok().map(|value| value.0))
                    }
                }
            }
        }
    }

    macro_rules! 𝖉𝖊𝖋𝖎𝖓𝖊_𝖊𝖓𝖚𝖒𝖘 {
        ($(
         [$({$𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴:expr})*]
         [$($𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮:ident),*]
         [$($𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮:ident $({$𝓮𝓷𝓾𝓶_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴:expr})*),*]
         pub enum $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮:ident {
            $($𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮:ident = $𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓮𝓷𝓬𝓸𝓭𝓲𝓷𝓰:expr),*
         })*) => {
            $(
                #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
                #[repr(i8)]
                pub enum $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮 {
                   $(
                       $𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮 = $𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓮𝓷𝓬𝓸𝓭𝓲𝓷𝓰,
                    )*
                }

                𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖞𝖋𝖗𝖔𝖒_𝖋𝖔𝖗_𝖎𝖓𝖙!($𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮 {|value| value} $({$𝓲𝓷𝓽_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴})*);

                $(
                    impl From<$𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮> for $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮 {
                        #[inline(always)]
                        fn from(value: $𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮) -> Self {
                            // Note: we are using repr(i8) here thus conversion is safe.
                            unsafe { std::mem::transmute::<$𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮, $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮>(value) }
                        }
                    }
                    impl From<$𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮> for Option<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮> {
                        #[inline(always)]
                        fn from(value: $𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮) -> Self {
                            // Note: we are using repr(i8) here thus conversion is safe.
                            Some(unsafe { std::mem::transmute::<$𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮, $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮>(value) })
                        }
                    }
                 )*

                $(
                    impl std::convert::TryFrom<$𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮> for $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮 {
                        // The only possible error here can be is “register doesn't belong to specific register class”.
                        //
                        // Returning Err(()) is enough to pass that infomation but makes Result smaller (although in real code it's
                        // almost always consumed with ok() thus we may pass some more info, but then if it's always consumed by ok()
                        // then what's the point of passing more into?).
                        type Error = ();

                        #[allow(clippy::erasing_op,clippy::redundant_closure_call)]
                        #[inline(always)]
                        fn try_from(value: $𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮) -> Result<Self, Self::Error> {
                            $(if ($𝓮𝓷𝓾𝓶_𝓼𝓪𝓯𝓮𝓽𝔂_𝓬𝓱𝓮𝓬𝓴)(value as i8) {return Err(())})*
                            // Note: we are using repr(i8) here thus conversion is safe and it doesn't disable any optimizations:
                            //   Result<Self, Self::Error> is still one byte in size.
                            Ok(unsafe { std::mem::transmute::<$𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮, $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮>(value) })
                        }
                    }
                 )*

                #[cfg(test)]
                impl std::convert::TryFrom<i8> for 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮> {
                    type Error = ();
                    fn try_from(value: i8) -> Result<Self, Self::Error> {
                        match value {
                            $(
                                value if value >=0 && value == $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮::$𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮 as i8 => Ok(𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕($𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮::$𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮)),
                             )*
                            _ => Err(())
                        }
                    }
                }

                #[cfg(test)]
                impl 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮> {
                    #[allow(dead_code)]
                    fn all_from(value: i8) -> Result<Self, ()> {
                        match value {
                            $(
                                value if value == $𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮::$𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮 as i8 => Ok(𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕($𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮::$𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮)),
                             )*
                            _ => Err(())
                        }
                    }
                }

                $(
                    #[cfg(test)]
                    impl From<𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>> for 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮> {
                        fn from(value: 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>) -> Self {
                            Self::all_from(value.0 as i8).unwrap()
                        }
                    }
                 )*

                $(
                    #[cfg(test)]
                    impl std::convert::TryFrom<𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>> for 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮> {
                        // The only possible error here can be is “register doesn't belong to specific register class”.
                        //
                        // Returning Err(()) is enough to pass that infomation but makes Result smaller (although in real code it's
                        // almost always consumed with ok() thus we may pass some more info, but then if it's always consumed by ok()
                        // then what's the point of passing more into?).
                        type Error = ();

                        fn try_from(value: 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>) -> Result<Self, Self::Error> {
                            // Certain enum values shouldn't be converted even if values match.
                            // E.g. both 𝔟𝔥 and 𝔢𝔦𝔷 have value -1, but they shouldn't be converted.
                            // Since this only needed that for tests we don't worry about efficiency.
                            if value.0 as i8 == -1 &&
                               (((stringify!($𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮).starts_with("𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫") ||
                                  stringify!($𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮).starts_with("𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫")) &&
                                 stringify!($𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮).starts_with("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ")) ||
                                (stringify!($𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮).starts_with("𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ") &&
                                 (stringify!($𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮).starts_with("𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫") ||
                                  stringify!($𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮).starts_with("𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫")))) {
                                return Err(())
                            }
                            Self::all_from(value.0 as i8)
                        }
                    }
                 )*

                #[cfg(test)]
                impl 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮> {
                    fn test_safe() {
                        $(
                            for value in i8::MIN..=i8::MAX {
                                if let Ok(value) = 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕::<$𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>::all_from(value) {
                                    assert_eq!($𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮::from(value.0), 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕::<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮>::from(value).0)
                                }
                            }
                         )*
                    }
                }

                #[cfg(test)]
                impl 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮> {
                    fn test_unsafe() {
                        $(
                            // Count number of successfully converted enum values.  Providing conversions which may never succeed
                            // is not beneficial: it just shifts detection of problems from compile-time to runtime.
                            let mut successfully_converted = 0;
                            for value in i8::MIN..=i8::MAX {
                                if let Ok(value) = 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕::<$𝓾𝓷𝓼𝓪𝓯𝓮_𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>::all_from(value) {
                                    let converted_safely = std::convert::TryInto::<𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕::<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮>>::try_into(value)
                                        .ok()
                                        .map(|value| value.0);
                                    let converted_unsafely = std::convert::TryInto::<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮>::try_into(value.0).ok();
                                    assert_eq!(converted_safely, converted_unsafely);
                                    if converted_safely.is_some() {
                                        successfully_converted += 1
                                    }
                                }
                            }
                            assert!(successfully_converted > 0);
                         )*
                    }
                }
             )*

            #[cfg(test)]
            mod 𝗲𝗻𝘂𝗺_𝘁𝗲𝘀𝘁𝘀 {
                use super::*;

                #[test]
                fn test_i8() {
                    $(
                        𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕::<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮>::test_i8();
                     )*
                }

                #[test]
                fn test_safe() {
                    $(
                        𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕::<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮>::test_safe();
                     )*
                }

                #[test]
                fn test_unsafe() {
                    $(
                        𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕::<$𝓮𝓷𝓾𝓶_𝓷𝓪𝓶𝓮>::test_unsafe();
                     )*
                }
            }
        };
    }

    // To ensure safety we provice separate types for different classes of registers.
    // But Rust compiler currently is not advanced enough to produce e.g. effective conversion from 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 to
    // 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64: while you only need to check that value is not 𝔢𝔰𝔭 and copy value without any processing
    // compiler currently does that using lookup tables.

    // To make sure this woulnd't happen we are providing that conversion only for tests: 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64>
    // would be converted to 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64> using safe code and matching values.

    // The actual, production, conversion is compared to that one on the full range of 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64.

    // This way we can guarantee that our tests are enough to ensure safety.
    // Note: since mistakes here may trigger undefined behavior tests have to be run with “cargo +nightly miri test”.

    #[cfg(test)]
    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    #[repr(transparent)]
    pub struct 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>(𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮);

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖊𝖓𝖚𝖒𝖘! {
        [{|value| value != 0}]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 0}
        ]
        pub enum 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ {
            𝔞𝔩 = 0
        }

        [{|value| value != 0}]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 0}
        ]
        pub enum 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {
            𝔞𝔵 = 0
        }

        [{|value| value != 0}]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 0}
        ]
        pub enum 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ {
            𝔢𝔞𝔵 = 0
        }

        [{|value| value != 0}]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 0}
        ]
        pub enum 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {
            𝔯𝔞𝔵 = 0
        }

        [{|value| value != 1}]
        [   𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 1}
        ]
        pub enum 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ {
            𝔠𝔩 = 1
        }

        [{|value| value != 1}]
        [   𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 1}
        ]
        pub enum 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {
            𝔠𝔵 = 1
        }

        [{|value| value != 1}]
        [   𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 1}
        ]
        pub enum 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ {
            𝔢𝔠𝔵 = 1
        }

        [{|value| value != 1}]
        [   𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 1},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 1}
        ]
        pub enum 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {
            𝔯𝔠𝔵 = 1
        }

        [{|value| value != 2}]
        [   𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 2},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 2},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 2},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 2},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 2},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 2}
        ]
        pub enum 𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {
            𝔡𝔵 = 2
        }

        [{|value| value != 2}]
        [   𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 2},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 2},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 2},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 2},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 2},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 2}
        ]
        pub enum 𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ {
            𝔢𝔡𝔵 = 2
        }

        [{|value| value != 2}]
        [   𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 2},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 2},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 2},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 2},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 2},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 2},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 2}
        ]
        pub enum 𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {
            𝔯𝔡𝔵 = 2
        }

        [{|value| value != 3}]
        [   𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 3}
        ]
        pub enum 𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {
            𝔟𝔵 = 3
        }

        [{|value| value != 3}]
        [   𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 3}
        ]
        pub enum 𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ {
            𝔢𝔟𝔵 = 3
        }

        [{|value| value != 3}]
        [   𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 3}
        ]
        pub enum 𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {
            𝔯𝔟𝔵 = 3
        }

        [{|value| value != 4}]
        [   𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 4}
        ]
        pub enum 𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {
            𝔰𝔭 = 4
        }

        [{|value| value != 4}]
        [   𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 4}
        ]
        pub enum 𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ {
            𝔢𝔰𝔭 = 4
        }

        [{|value| value != 4}]
        [   𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ
        ]
        [   𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 4}
        ]
        pub enum 𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {
            𝔯𝔰𝔭 = 4
        }

        [{|value| value != 5}]
        [   𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {|value| value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 5}
        ]
        pub enum 𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {
            𝔟𝔭 = 5
        }

        [{|value| value != 5}]
        [   𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {|value| value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 5}
        ]
        pub enum 𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ {
            𝔢𝔟𝔭 = 5
        }

        [{|value| value != 5}]
        [   𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {|value| value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 5}
        ]
        pub enum 𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {
            𝔯𝔟𝔭 = 5
        }

        [{|value| value != 6}]
        [   𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {|value| value != 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 6}
        ]
        pub enum 𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {
            𝔰𝔦 = 6
        }

        [{|value| value != 6}]
        [   𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {|value| value != 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 6}
        ]
        pub enum 𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ {
            𝔢𝔰𝔦 = 6
        }

        [{|value| value != 6}]
        [   𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {|value| value != 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 6}
        ]
        pub enum 𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {
            𝔯𝔰𝔦 = 6
        }

        [{|value| value != 7}]
        [   𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {|value| value != 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 7}
        ]
        pub enum 𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {
            𝔡𝔦 = 7
        }

        [{|value| value != 7}]
        [   𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {|value| value != 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 7}
        ]
        pub enum 𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ {
            𝔢𝔡𝔦 = 7
        }

        [{|value| value != 7}]
        [   𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {|value| value != 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 7}
        ]
        pub enum 𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {
            𝔯𝔡𝔦 = 7
        }

        []
        [𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != -1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != -1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != -1}
        ]
        pub enum 𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 {
            𝔢𝔦𝔷 = -1
        }

        []
        [𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != -1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != -1},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != -1}
        ]
        pub enum 𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 {
            𝔯𝔦𝔷 = -1
        }

        [{|value| value != 3 && value != 5}]
        [   𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 3 && value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 3 && value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 3 && value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 3 && value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 3 && value != 5},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 3 && value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value != 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value != 3 && value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value != 3 && value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value != 3 && value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value != 3 && value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value != 3 && value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value != 3 && value != 5},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value != 3 && value != 5}
        ]
        pub enum 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {
            𝔟𝔵 = 3,
            𝔟𝔭 = 5
        }

        [{|value| !(6..=7).contains(&value)}]
        [   𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value < 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| !(6..=7).contains(&value)},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| !(6..=7).contains(&value)},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value < 6},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| !(6..=7).contains(&value)},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| !(6..=7).contains(&value)},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| !(6..=7).contains(&value)},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| !(6..=7).contains(&value)},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value < 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| !(6..=7).contains(&value)},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value < 6},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| !(6..=7).contains(&value)},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| !(6..=7).contains(&value)}
        ]
        pub enum 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {
            𝔰𝔦 = 6,
            𝔡𝔦 = 7
        }

        [{|value| value == 4 || value > 7}]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value > 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value > 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value < 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| !(0..=7).contains(&value)},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| !(0..=7).contains(&value)},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value < 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value < 0 || value == 4 || value > 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value == 4 || value > 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value == 4 || value > 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value == 4 || value > 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value == 4 || value > 7}
        ]
        pub enum 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {
            𝔢𝔞𝔵 = 0,
            𝔢𝔠𝔵 = 1,
            𝔢𝔡𝔵 = 2,
            𝔢𝔟𝔵 = 3,
            𝔢𝔟𝔭 = 5,
            𝔢𝔰𝔦 = 6,
            𝔢𝔡𝔦 = 7
        }

        [{|value| value == 4 || value > 15}]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value < 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value < 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value < 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value < 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value < 0 || value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value == 4}
        ]
        pub enum 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {
            𝔢𝔞𝔵 = 0,
            𝔢𝔠𝔵 = 1,
            𝔢𝔡𝔵 = 2,
            𝔢𝔟𝔵 = 3,
            𝔢𝔟𝔭 = 5,
            𝔢𝔰𝔦 = 6,
            𝔢𝔡𝔦 = 7,
            𝔯8𝔡 = 8,
            𝔯9𝔡 = 9,
            𝔯10𝔡 = 10,
            𝔯11𝔡 = 11,
            𝔯12𝔡 = 12,
            𝔯13𝔡 = 13,
            𝔯14𝔡 = 14,
            𝔯15𝔡 = 15
        }

        [{|value| value == 4 || value > 15}]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value < 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value < 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value < 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value < 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value < 0 || value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value == 4}
        ]
        pub enum 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {
            𝔯𝔞𝔵 = 0,
            𝔯𝔠𝔵 = 1,
            𝔯𝔡𝔵 = 2,
            𝔯𝔟𝔵 = 3,
            𝔯𝔟𝔭 = 5,
            𝔯𝔰𝔦 = 6,
            𝔯𝔡𝔦 = 7,
            𝔯8 = 8,
            𝔯9 = 9,
            𝔯10 = 10,
            𝔯11 = 11,
            𝔯12 = 12,
            𝔯13 = 13,
            𝔯14 = 14,
            𝔯15 = 15
        }

        [{|value| value == 4 || value > 7}]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
            𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value > 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value > 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value > 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value > 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value < 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value < 0 || value == 4 || value > 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value == 4 || value > 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value == 4 || value > 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value == 4 || value > 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value == 4 || value > 7}
        ]
        pub enum 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {
            𝔢𝔦𝔷 = -1,
            𝔢𝔞𝔵 = 0,
            𝔢𝔠𝔵 = 1,
            𝔢𝔡𝔵 = 2,
            𝔢𝔟𝔵 = 3,
            𝔢𝔟𝔭 = 5,
            𝔢𝔰𝔦 = 6,
            𝔢𝔡𝔦 = 7
        }

        [{|value| value == 4 || value > 15}]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
            𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value < 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value < 0 || value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value == 4}
        ]
        pub enum 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {
            𝔢𝔦𝔷 = -1,
            𝔢𝔞𝔵 = 0,
            𝔢𝔠𝔵 = 1,
            𝔢𝔡𝔵 = 2,
            𝔢𝔟𝔵 = 3,
            𝔢𝔟𝔭 = 5,
            𝔢𝔰𝔦 = 6,
            𝔢𝔡𝔦 = 7,
            𝔯8𝔡 = 8,
            𝔯9𝔡 = 9,
            𝔯10𝔡 = 10,
            𝔯11𝔡 = 11,
            𝔯12𝔡 = 12,
            𝔯13𝔡 = 13,
            𝔯14𝔡 = 14,
            𝔯15𝔡 = 15
        }

        [{|value| value == 4 || value > 15}]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
            𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64
        ]
        [   𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value < 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value < 0 || value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value == 4},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value == 4}
        ]
        pub enum 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {
            𝔯𝔦𝔷 = -1,
            𝔯𝔞𝔵 = 0,
            𝔯𝔠𝔵 = 1,
            𝔯𝔡𝔵 = 2,
            𝔯𝔟𝔵 = 3,
            𝔯𝔟𝔭 = 5,
            𝔯𝔰𝔦 = 6,
            𝔯𝔡𝔦 = 7,
            𝔯8 = 8,
            𝔯9 = 9,
            𝔯10 = 10,
            𝔯11 = 11,
            𝔯12 = 12,
            𝔯13 = 13,
            𝔯14 = 14,
            𝔯15 = 15
        }

        [{|value| value > 3}]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ {|value| value != 3},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| !(0..=3).contains(&value)},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| !(0..=3).contains(&value)},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| !(0..=3).contains(&value)},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value > 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value > 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {|value| value > 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value > 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value > 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value > 3},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value > 3}
        ]
        pub enum 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {
            𝔞𝔥 = -4,
            𝔠𝔥 = -3,
            𝔡𝔥 = -2,
            𝔟𝔥 = -1,
            𝔞𝔩 = 0,
            𝔠𝔩 = 1,
            𝔡𝔩 = 2,
            𝔟𝔩 = 3
        }

        [{|value| value > 15}]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value < 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value < 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value < 0}
        ]
        pub enum 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {
            𝔞𝔥 = -4,
            𝔠𝔥 = -3,
            𝔡𝔥 = -2,
            𝔟𝔥 = -1,
            𝔞𝔩 = 0,
            𝔠𝔩 = 1,
            𝔡𝔩 = 2,
            𝔟𝔩 = 3,
            𝔰𝔭𝔩 = 4,
            𝔟𝔭𝔩 = 5,
            𝔰𝔦𝔩 = 6,
            𝔡𝔦𝔩 = 7,
            𝔯8𝔟 = 8,
            𝔯9𝔟 = 9,
            𝔯10𝔟 = 10,
            𝔯11𝔟 = 11,
            𝔯12𝔟 = 12,
            𝔯13𝔟 = 13,
            𝔯14𝔟 = 14,
            𝔯15𝔟 = 15
        }

        [{|value| value > 15}]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value < 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value < 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value < 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value < 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value < 0}
        ]
        pub enum 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {
            𝔞𝔩 = 0,
            𝔠𝔩 = 1,
            𝔡𝔩 = 2,
            𝔟𝔩 = 3,
            𝔰𝔭𝔩 = 4,
            𝔟𝔭𝔩 = 5,
            𝔰𝔦𝔩 = 6,
            𝔡𝔦𝔩 = 7,
            𝔯8𝔟 = 8,
            𝔯9𝔟 = 9,
            𝔯10𝔟 = 10,
            𝔯11𝔟 = 11,
            𝔯12𝔟 = 12,
            𝔯13𝔟 = 13,
            𝔯14𝔟 = 14,
            𝔯15𝔟 = 15
        }

        [{|value| value > 7}]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value > 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value > 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value < 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| !(0..=7).contains(&value)},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| !(0..=7).contains(&value)},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value < 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| !(0..=7).contains(&value)},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value > 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value > 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value > 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value > 7}
        ]
        pub enum 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086 {
            𝔞𝔵 = 0,
            𝔠𝔵 = 1,
            𝔡𝔵 = 2,
            𝔟𝔵 = 3,
            𝔰𝔭 = 4,
            𝔟𝔭 = 5,
            𝔰𝔦 = 6,
            𝔡𝔦 = 7
        }

        [{|value| value > 15}]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value < 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value < 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value < 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value < 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value < 0}
        ]
        pub enum 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {
            𝔞𝔵 = 0,
            𝔠𝔵 = 1,
            𝔡𝔵 = 2,
            𝔟𝔵 = 3,
            𝔰𝔭 = 4,
            𝔟𝔭 = 5,
            𝔰𝔦 = 6,
            𝔡𝔦 = 7,
            𝔯8𝔴 = 8,
            𝔯9𝔴 = 9,
            𝔯10𝔴 = 10,
            𝔯11𝔴 = 11,
            𝔯12𝔴 = 12,
            𝔯13𝔴 = 13,
            𝔯14𝔴 = 14,
            𝔯15𝔴 = 15
        }

        [{|value| value > 7}]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value > 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value > 7},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value < 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| !(0..=7).contains(&value)},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| !(0..=7).contains(&value)},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value < 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| !(0..=7).contains(&value)},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64 {|value| value > 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64 {|value| value > 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value > 7},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value > 7}
        ]
        pub enum 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {
            𝔢𝔞𝔵 = 0,
            𝔢𝔠𝔵 = 1,
            𝔢𝔡𝔵 = 2,
            𝔢𝔟𝔵 = 3,
            𝔢𝔰𝔭 = 4,
            𝔢𝔟𝔭 = 5,
            𝔢𝔰𝔦 = 6,
            𝔢𝔡𝔦 = 7
        }

        [{|value| value > 15}]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value < 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value < 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value < 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value < 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value < 0}
        ]
        pub enum 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {
            𝔢𝔞𝔵 = 0,
            𝔢𝔠𝔵 = 1,
            𝔢𝔡𝔵 = 2,
            𝔢𝔟𝔵 = 3,
            𝔢𝔰𝔭 = 4,
            𝔢𝔟𝔭 = 5,
            𝔢𝔰𝔦 = 6,
            𝔢𝔡𝔦 = 7,
            𝔯8𝔡 = 8,
            𝔯9𝔡 = 9,
            𝔯10𝔡 = 10,
            𝔯11𝔡 = 11,
            𝔯12𝔡 = 12,
            𝔯13𝔡 = 13,
            𝔯14𝔡 = 14,
            𝔯15𝔡 = 15
        }

        [{|value| value > 15}]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗᵣₑₓ_ₓ86_64,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64
        ]
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 {|value| value < 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 {|value| value < 0},
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {|value| value < 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_8086 {|value| value < 0},
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ_ₓ86_64 {|value| value < 0}
        ]
        pub enum 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ {
            𝔯𝔞𝔵 = 0,
            𝔯𝔠𝔵 = 1,
            𝔯𝔡𝔵 = 2,
            𝔯𝔟𝔵 = 3,
            𝔯𝔰𝔭 = 4,
            𝔯𝔟𝔭 = 5,
            𝔯𝔰𝔦 = 6,
            𝔯𝔡𝔦 = 7,
            𝔯8 = 8,
            𝔯9 = 9,
            𝔯10 = 10,
            𝔯11 = 11,
            𝔯12 = 12,
            𝔯13 = 13,
            𝔯14 = 14,
            𝔯15 = 15
        }

        [{|value| value != 0}]
        []
        [𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 {|value| value != 0}]
        pub enum 𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 {
            𝔰𝔱 = 0
        }

        [{|value| value > 7}]
        [𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫]
        []
        pub enum 𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 {
            𝔰𝔱0 = 0,
            𝔰𝔱1 = 1,
            𝔰𝔱2 = 2,
            𝔰𝔱3 = 3,
            𝔰𝔱4 = 4,
            𝔰𝔱5 = 5,
            𝔰𝔱6 = 6,
            𝔰𝔱7 = 7
        }

        [{|value| value != 0x26 &&
                  value != 0x2e &&
                  value != 0x36 &&
                  value != 0x3e &&
                  value != 0x64 &&
                  value != 0x65}]
        []
        []
        pub enum 𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 {
            𝔢𝔰 = 0x26,
            𝔠𝔰 = 0x2e,
            𝔰𝔰 = 0x36,
            𝔡𝔰 = 0x3e,
            𝔣𝔰 = 0x64,
            𝔤𝔰 = 0x65
        }

        [{|value| value > 7}]
        []
        []
        pub enum 𝒎𝒎𝒙_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 {
            𝔪𝔪0 = 0,
            𝔪𝔪1 = 1,
            𝔪𝔪2 = 2,
            𝔪𝔪3 = 3,
            𝔪𝔪4 = 4,
            𝔪𝔪5 = 5,
            𝔪𝔪6 = 6,
            𝔪𝔪7 = 7
        }


        [{|value| value == 0 || value > 7}]
        []
        [𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 {|value| value == 0}]
        pub enum 𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 {
            𝔨1 = 1,
            𝔨2 = 2,
            𝔨3 = 3,
            𝔨4 = 4,
            𝔨5 = 5,
            𝔨6 = 6,
            𝔨7 = 7
        }

        [{|value| value > 7}]
        [𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫]
        []
        pub enum 𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 {
            𝔨0 = 0,
            𝔨1 = 1,
            𝔨2 = 2,
            𝔨3 = 3,
            𝔨4 = 4,
            𝔨5 = 5,
            𝔨6 = 6,
            𝔨7 = 7
        }

        [{|value| value != 0}]
        []
        [   𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₚₑₙₜᵢᵤₘ3 {|value| value != 0},
            𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64 {|value| value != 0},
            𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512 {|value| value != 0},
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ {|value| value != 0},
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64 {|value| value != 0},
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512 {|value| value != 0},
            𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86 {|value| value != 0},
            𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64 {|value| value != 0}
        ]
        pub enum 𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 {
            𝔵𝔪𝔪0 = 0
        }

        [{|value| value > 7}]
        [   𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ,
            𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86
        ]
        [   𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64 {|value| value > 7},
            𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512 {|value| value > 7},
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64 {|value| value > 7},
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512 {|value| value > 7},
            𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64 {|value| value > 7}
        ]
        pub enum 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₚₑₙₜᵢᵤₘ3 {
            𝔵𝔪𝔪0 = 0,
            𝔵𝔪𝔪1 = 1,
            𝔵𝔪𝔪2 = 2,
            𝔵𝔪𝔪3 = 3,
            𝔵𝔪𝔪4 = 4,
            𝔵𝔪𝔪5 = 5,
            𝔵𝔪𝔪6 = 6,
            𝔵𝔪𝔪7 = 7
        }

        [{|value| value > 15}]
        [   𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
            𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₚₑₙₜᵢᵤₘ3,
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ,
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
            𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86
        ]
        [   𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512 {|value| value > 15},
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512 {|value| value > 15},
            𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64 {|value| value > 15}
        ]
        pub enum 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64 {
            𝔵𝔪𝔪0 = 0,
            𝔵𝔪𝔪1 = 1,
            𝔵𝔪𝔪2 = 2,
            𝔵𝔪𝔪3 = 3,
            𝔵𝔪𝔪4 = 4,
            𝔵𝔪𝔪5 = 5,
            𝔵𝔪𝔪6 = 6,
            𝔵𝔪𝔪7 = 7,
            𝔵𝔪𝔪8 = 8,
            𝔵𝔪𝔪9 = 9,
            𝔵𝔪𝔪10 = 10,
            𝔵𝔪𝔪11 = 11,
            𝔵𝔪𝔪12 = 12,
            𝔵𝔪𝔪13 = 13,
            𝔵𝔪𝔪14 = 14,
            𝔵𝔪𝔪15 = 15
        }

        [{|value| value > 31}]
        [   𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
            𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₚₑₙₜᵢᵤₘ3,
            𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ,
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512,
            𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86,
            𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64
        ]
        []
        pub enum 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512 {
            𝔵𝔪𝔪0 = 0,
            𝔵𝔪𝔪1 = 1,
            𝔵𝔪𝔪2 = 2,
            𝔵𝔪𝔪3 = 3,
            𝔵𝔪𝔪4 = 4,
            𝔵𝔪𝔪5 = 5,
            𝔵𝔪𝔪6 = 6,
            𝔵𝔪𝔪7 = 7,
            𝔵𝔪𝔪8 = 8,
            𝔵𝔪𝔪9 = 9,
            𝔵𝔪𝔪10 = 10,
            𝔵𝔪𝔪11 = 11,
            𝔵𝔪𝔪12 = 12,
            𝔵𝔪𝔪13 = 13,
            𝔵𝔪𝔪14 = 14,
            𝔵𝔪𝔪15 = 15,
            𝔵𝔪𝔪16 = 16,
            𝔵𝔪𝔪17 = 17,
            𝔵𝔪𝔪18 = 18,
            𝔵𝔪𝔪19 = 19,
            𝔵𝔪𝔪20 = 20,
            𝔵𝔪𝔪21 = 21,
            𝔵𝔪𝔪22 = 22,
            𝔵𝔪𝔪23 = 23,
            𝔵𝔪𝔪24 = 24,
            𝔵𝔪𝔪25 = 25,
            𝔵𝔪𝔪26 = 26,
            𝔵𝔪𝔪27 = 27,
            𝔵𝔪𝔪28 = 28,
            𝔵𝔪𝔪29 = 29,
            𝔵𝔪𝔪30 = 30,
            𝔵𝔪𝔪31 = 31
        }

        [{|value| value > 7}]
        [   𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
            𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₚₑₙₜᵢᵤₘ3,
            𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86
        ]
        [   𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64 {|value| value > 7},
            𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512 {|value| value > 7},
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64 {|value| value > 7},
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512 {|value| value > 7},
            𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64 {|value| value > 7}
        ]
        pub enum 𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ {
            𝔶𝔪𝔪0 = 0,
            𝔶𝔪𝔪1 = 1,
            𝔶𝔪𝔪2 = 2,
            𝔶𝔪𝔪3 = 3,
            𝔶𝔪𝔪4 = 4,
            𝔶𝔪𝔪5 = 5,
            𝔶𝔪𝔪6 = 6,
            𝔶𝔪𝔪7 = 7
        }

        [{|value| value > 15}]
        [   𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
            𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₚₑₙₜᵢᵤₘ3,
            𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ,
            𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86
        ]
        [   𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512 {|value| value > 15},
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512 {|value| value > 15},
            𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64 {|value| value > 15}
        ]
        pub enum 𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64 {
            𝔶𝔪𝔪0 = 0,
            𝔶𝔪𝔪1 = 1,
            𝔶𝔪𝔪2 = 2,
            𝔶𝔪𝔪3 = 3,
            𝔶𝔪𝔪4 = 4,
            𝔶𝔪𝔪5 = 5,
            𝔶𝔪𝔪6 = 6,
            𝔶𝔪𝔪7 = 7,
            𝔶𝔪𝔪8 = 8,
            𝔶𝔪𝔪9 = 9,
            𝔶𝔪𝔪10 = 10,
            𝔶𝔪𝔪11 = 11,
            𝔶𝔪𝔪12 = 12,
            𝔶𝔪𝔪13 = 13,
            𝔶𝔪𝔪14 = 14,
            𝔶𝔪𝔪15 = 15
        }

        [{|value| value > 31}]
        [   𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
            𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₚₑₙₜᵢᵤₘ3,
            𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
            𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512,
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ,
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
            𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86,
            𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64
        ]
        []
        pub enum 𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512 {
            𝔶𝔪𝔪0 = 0,
            𝔶𝔪𝔪1 = 1,
            𝔶𝔪𝔪2 = 2,
            𝔶𝔪𝔪3 = 3,
            𝔶𝔪𝔪4 = 4,
            𝔶𝔪𝔪5 = 5,
            𝔶𝔪𝔪6 = 6,
            𝔶𝔪𝔪7 = 7,
            𝔶𝔪𝔪8 = 8,
            𝔶𝔪𝔪9 = 9,
            𝔶𝔪𝔪10 = 10,
            𝔶𝔪𝔪11 = 11,
            𝔶𝔪𝔪12 = 12,
            𝔶𝔪𝔪13 = 13,
            𝔶𝔪𝔪14 = 14,
            𝔶𝔪𝔪15 = 15,
            𝔶𝔪𝔪16 = 16,
            𝔶𝔪𝔪17 = 17,
            𝔶𝔪𝔪18 = 18,
            𝔶𝔪𝔪19 = 19,
            𝔶𝔪𝔪20 = 20,
            𝔶𝔪𝔪21 = 21,
            𝔶𝔪𝔪22 = 22,
            𝔶𝔪𝔪23 = 23,
            𝔶𝔪𝔪24 = 24,
            𝔶𝔪𝔪25 = 25,
            𝔶𝔪𝔪26 = 26,
            𝔶𝔪𝔪27 = 27,
            𝔶𝔪𝔪28 = 28,
            𝔶𝔪𝔪29 = 29,
            𝔶𝔪𝔪30 = 30,
            𝔶𝔪𝔪31 = 31
        }

        [{|value| value > 7}]
        [   𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
            𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₚₑₙₜᵢᵤₘ3,
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ
        ]
        [   𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64 {|value| value > 7},
            𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512 {|value| value > 7},
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64 {|value| value > 7},
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512 {|value| value > 7},
            𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64 {|value| value > 7}
        ]
        pub enum 𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86 {
            𝔷𝔪𝔪0 = 0,
            𝔷𝔪𝔪1 = 1,
            𝔷𝔪𝔪2 = 2,
            𝔷𝔪𝔪3 = 3,
            𝔷𝔪𝔪4 = 4,
            𝔷𝔪𝔪5 = 5,
            𝔷𝔪𝔪6 = 6,
            𝔷𝔪𝔪7 = 7
        }

        [{|value| value > 31}]
        [   𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
            𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₚₑₙₜᵢᵤₘ3,
            𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
            𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512,
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ,
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
            𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512,
            𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86
        ]
        []
        pub enum 𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64 {
            𝔷𝔪𝔪0 = 0,
            𝔷𝔪𝔪1 = 1,
            𝔷𝔪𝔪2 = 2,
            𝔷𝔪𝔪3 = 3,
            𝔷𝔪𝔪4 = 4,
            𝔷𝔪𝔪5 = 5,
            𝔷𝔪𝔪6 = 6,
            𝔷𝔪𝔪7 = 7,
            𝔷𝔪𝔪8 = 8,
            𝔷𝔪𝔪9 = 9,
            𝔷𝔪𝔪10 = 10,
            𝔷𝔪𝔪11 = 11,
            𝔷𝔪𝔪12 = 12,
            𝔷𝔪𝔪13 = 13,
            𝔷𝔪𝔪14 = 14,
            𝔷𝔪𝔪15 = 15,
            𝔷𝔪𝔪16 = 16,
            𝔷𝔪𝔪17 = 17,
            𝔷𝔪𝔪18 = 18,
            𝔷𝔪𝔪19 = 19,
            𝔷𝔪𝔪20 = 20,
            𝔷𝔪𝔪21 = 21,
            𝔷𝔪𝔪22 = 22,
            𝔷𝔪𝔪23 = 23,
            𝔷𝔪𝔪24 = 24,
            𝔷𝔪𝔪25 = 25,
            𝔷𝔪𝔪26 = 26,
            𝔷𝔪𝔪27 = 27,
            𝔷𝔪𝔪28 = 28,
            𝔷𝔪𝔪29 = 29,
            𝔷𝔪𝔪30 = 30,
            𝔷𝔪𝔪31 = 31
        }
    }

    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    #[repr(i8)]
    pub enum 𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞 {
        𝔵1 = 1,
        𝔵2 = 2,
        𝔵4 = 4,
        𝔵8 = 8,
    }

    impl Default for 𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞 {
        #[inline(always)]
        fn default() -> Self {
            𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞::𝔵1
        }
    }

    impl From<super::𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞> for 𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞 {
        #[inline(always)]
        fn from(_: super::𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞) -> Self {
            Default::default()
        }
    }

    // Note: (value * 0 + 1) gives us 1 of the appropriate type.
    // Otherwise << would use 1 to determine type of the result and would end up with i32, not i8/u8 which we need.
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖞𝖋𝖗𝖔𝖒_𝖋𝖔𝖗_𝖎𝖓𝖙!(𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞 {|value| (value * 0 + 1) << value} {|value| value > 3});

    #[cfg(test)]
    impl std::convert::TryFrom<i8> for 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞> {
        type Error = ();
        #[allow(dead_code)]
        fn try_from(value: i8) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕(𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞::𝔵1)),
                1 => Ok(𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕(𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞::𝔵2)),
                2 => Ok(𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕(𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞::𝔵4)),
                3 => Ok(𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕(𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞::𝔵8)),
                _ => Err(()),
            }
        }
    }

    use super::𝒊𝒔_𝑺𝑰𝑴𝑫_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓;
    use super::𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓;
    use super::𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓;
    use super::𝒊𝒔_𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓;
    use super::𝒊𝒔_𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒏𝒐_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓;
    use super::𝒊𝒔_𝒎𝒂𝒔𝒌_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓;
    use super::𝒊𝒔_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓;
    use super::𝒊𝒔_𝒙𝟖𝟕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓;

    use super::𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆;

    macro_rules! 𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘 {
        ($𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident, $($𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮:ident),*) => {
            $(
                impl $𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮 for $𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {}
             )*
        };
        ($𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident for $𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮:ident, $($𝓼𝓸𝓾𝓻𝓬𝓮_𝓷𝓪𝓶𝓮:ident),*) => {
            $(
                impl $𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮<$𝓼𝓸𝓾𝓻𝓬𝓮_𝓷𝓪𝓶𝓮> for $𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮 {}
                impl $𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮<$𝓼𝓸𝓾𝓻𝓬𝓮_𝓷𝓪𝓶𝓮> for Option<$𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮> {}
             )*
        };
        ($𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident, $𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮:ident, [$($𝓼𝓸𝓾𝓻𝓬𝓮_𝓷𝓪𝓶𝓮:ident),*]) => {
            $(
                impl $𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮<$𝓼𝓸𝓾𝓻𝓬𝓮_𝓷𝓪𝓶𝓮> for $𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮 {}
                impl $𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮<$𝓼𝓸𝓾𝓻𝓬𝓮_𝓷𝓪𝓶𝓮> for Option<$𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮> {}
             )*
        };
        ($𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident, [$($𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮:ident),*] $𝓼𝓸𝓾𝓻𝓬𝓮_𝓷𝓪𝓶𝓮𝓼:tt) => {
            $(𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘!($𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮, $𝓽𝓪𝓻𝓰𝓮𝓽_𝓷𝓪𝓶𝓮, $𝓼𝓸𝓾𝓻𝓬𝓮_𝓷𝓪𝓶𝓮𝓼);)*
        }
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
        𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
        𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
        𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
        𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓,
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64
        ]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ
        ]
    }
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386
    }
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64
    }
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
        𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386
    }
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
        𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓,
        [   𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
            𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
        ]
    }
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒏𝒐_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓,
        [   𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64
        ]
        [   𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086
        ]
    }
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒏𝒐_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓 for 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒏𝒐_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓,
        [   𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64
        ]
        [   𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386
        ]
    }
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒏𝒐_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓 for 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒏𝒐_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓 for 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓,
        [   𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64
        ]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_8086
        ]
    }
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ_ₓ86_64
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓,
        [   𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64
        ]
        [   𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ,
            𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386
        ]
    }
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒐𝒑𝒕𝒊𝒐𝒏_𝒇𝒓𝒐𝒎_𝒖𝒏𝒇𝒊𝒍𝒍𝒆𝒅_𝒇𝒍𝒖𝒆𝒏𝒕_𝒗𝒂𝒍𝒖𝒆,
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒙𝟖𝟕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐬𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝒎𝒂𝒔𝒌_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝑺𝑰𝑴𝑫_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₚₑₙₜᵢᵤₘ3,
        𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₚₑₙₜᵢᵤₘ3
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝑺𝑰𝑴𝑫_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
        𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₚₑₙₜᵢᵤₘ3,
        𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝑺𝑰𝑴𝑫_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512,
        𝐱𝐦𝐦𝟎_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₚₑₙₜᵢᵤₘ3,
        𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
        𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝑺𝑰𝑴𝑫_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ,
        𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝑺𝑰𝑴𝑫_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
        𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ,
        𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝑺𝑰𝑴𝑫_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512,
        𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ,
        𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
        𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝑺𝑰𝑴𝑫_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86,
        𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86
    }

    𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖗𝖆𝖎𝖙_𝖋𝖔𝖗_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗𝖘! {
        𝒊𝒔_𝑺𝑰𝑴𝑫_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for 𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
        𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86,
        𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64
    }

    use super::𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086;
    use super::𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86;
    use super::𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔;

    pub type 𝐠𝐩_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_16ᵇⁱᵗ =
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ, i16>;

    pub type 𝐠𝐩_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_32ᵇⁱᵗ_80386 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
        𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
        i32,
    >;

    pub type 𝐠𝐩_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_32ᵇⁱᵗ_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_80386 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
        𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
        i32,
    >;

    pub type 𝐠𝐩_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_32ᵇⁱᵗ_ₓ86_64 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
        𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
        i32,
    >;

    pub type 𝐠𝐩_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_32ᵇⁱᵗ_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_ₓ86_64 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
        𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
        i32,
    >;

    pub type 𝐠𝐩_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_64ᵇⁱᵗ =
        𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ, 𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞, i32>;

    pub type 𝐠𝐩_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_64ᵇⁱᵗ_𝐰𝐢𝐭𝐡_𝐫𝐢𝐳 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐫𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
        i32,
    >;

    pub type 𝐠𝐚𝐭𝐡𝐞𝐫_𝐱𝐦𝐦_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_32ᵇⁱᵗ_ₐᵥₓ = 𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
        𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₚₑₙₜᵢᵤₘ3,
        𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
        i32,
    >;

    pub type 𝐠𝐚𝐭𝐡𝐞𝐫_𝐱𝐦𝐦_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_32ᵇⁱᵗ_ₓ86_64 = 𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
        𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
        𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
        i32,
    >;

    pub type 𝐠𝐚𝐭𝐡𝐞𝐫_𝐱𝐦𝐦_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_32ᵇⁱᵗ_ₐᵥₓ512 = 𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
        𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512,
        𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
        i32,
    >;

    pub type 𝐠𝐚𝐭𝐡𝐞𝐫_𝐲𝐦𝐦_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_32ᵇⁱᵗ_ₐᵥₓ = 𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
        𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ,
        𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
        i32,
    >;

    pub type 𝐠𝐚𝐭𝐡𝐞𝐫_𝐲𝐦𝐦_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_32ᵇⁱᵗ_ₓ86_64 = 𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
        𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
        𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
        i32,
    >;

    pub type 𝐠𝐚𝐭𝐡𝐞𝐫_𝐲𝐦𝐦_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_32ᵇⁱᵗ_ₐᵥₓ512 = 𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
        𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512,
        𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
        i32,
    >;

    pub type 𝐠𝐚𝐭𝐡𝐞𝐫_𝐳𝐦𝐦_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_32ᵇⁱᵗ_ₓ86 = 𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
        𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86,
        𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
        i32,
    >;

    pub type 𝐠𝐚𝐭𝐡𝐞𝐫_𝐳𝐦𝐦_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_32ᵇⁱᵗ_ₓ86_64 = 𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
        𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
        𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
        i32,
    >;

    pub type 𝐠𝐚𝐭𝐡𝐞𝐫_𝐱𝐦𝐦_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_64ᵇⁱᵗ_ₓ86_64 = 𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
        𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
        i32,
    >;

    pub type 𝐠𝐚𝐭𝐡𝐞𝐫_𝐱𝐦𝐦_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_64ᵇⁱᵗ_ₐᵥₓ512 = 𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512,
        𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
        i32,
    >;

    pub type 𝐠𝐚𝐭𝐡𝐞𝐫_𝐲𝐦𝐦_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_64ᵇⁱᵗ_ₓ86_64 = 𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
        𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
        i32,
    >;

    pub type 𝐠𝐚𝐭𝐡𝐞𝐫_𝐲𝐦𝐦_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_64ᵇⁱᵗ_ₐᵥₓ512 = 𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₐᵥₓ512,
        𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
        i32,
    >;

    pub type 𝐠𝐚𝐭𝐡𝐞𝐫_𝐳𝐦𝐦_𝐚𝐝𝐝𝐫𝐞𝐬𝐬_64ᵇⁱᵗ_ₓ86_64 = 𝒈𝒂𝒕𝒉𝒆𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
        𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_64ᵇⁱᵗ,
        𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_ₓ86_64,
        𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
        i32,
    >;
}
