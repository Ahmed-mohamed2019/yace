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

#ifndef 𝔜𝔄ℭ𝔈_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_𝔛86_𝔅𝔄𝔖ℑℭ_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_ℌ
#define 𝔜𝔄ℭ𝔈_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_𝔛86_𝔅𝔄𝔖ℑℭ_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_ℌ

#include <array>
#if __has_include(<compare>)
#include <compare>
#endif
#include <tuple>
#include <type_traits>

#include "yace/foundation.h"

#include "yace/assembler/x86/options.h"

namespace 𝘆𝗮𝗰𝗲::𝘅𝟴𝟲 {

namespace 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿 {

// AVX512 is pretty significant change and unlike other extension it affects existing instructions, not just adds new ones.
// Note: this is about 16 “extra” registers 𝔵𝔪𝔪16 to 𝔵𝔪𝔪31, 𝔶𝔪𝔪16 to 𝔶𝔪𝔪31, 𝔷𝔪𝔪16 to 𝔷𝔪𝔪31. Registers 𝔷𝔪𝔪0 to 𝔷𝔪𝔪7 (𝔷𝔪𝔪0 to 𝔷𝔪𝔪15
// in 𝔵86-64 mode) could still be used even if 𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬::𝔲𝔫𝔰𝔲𝔭𝔭𝔬𝔯𝔱𝔢𝔡 types are used.
enum class 𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬 { 𝔲𝔫𝔰𝔲𝔭𝔭𝔬𝔯𝔱𝔢𝔡, 𝔰𝔲𝔭𝔭𝔬𝔯𝔱𝔢𝔡 };

template <auto 𝔁86_𝓸𝓹𝓽𝓲𝓸𝓷𝓼, typename = void>
inline constexpr 𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬 𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰 = 𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬::𝔲𝔫𝔰𝔲𝔭𝔭𝔬𝔯𝔱𝔢𝔡;

template <auto 𝔁86_𝓸𝓹𝓽𝓲𝓸𝓷𝓼>
inline constexpr 𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬 𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰<
    𝔁86_𝓸𝓹𝓽𝓲𝓸𝓷𝓼,
    std::enable_if_t<
        𝔁86_𝓸𝓹𝓽𝓲𝓸𝓷𝓼->x86_mode >= ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32 and 𝔁86_𝓸𝓹𝓽𝓲𝓸𝓷𝓼->𝔁86_𝓸𝓹𝓽𝓲𝓸𝓷𝓼->avx512_supported>> =
    𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬::𝔰𝔲𝔭𝔭𝔬𝔯𝔱𝔢𝔡;

}  // namespace 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿

// Basic assembler defines: registers, functions, etc.  Suitable for all cases where
// you deal with concrete, physical registers.
template <
    typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻,
    auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼 = &::𝘆𝗮𝗰𝗲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔡𝔢𝔣𝔞𝔲𝔩𝔱,
    ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 = 𝔵86_𝔪𝔬𝔡𝔢<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>,
    ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>>
class 𝒃𝒂𝒔𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓;

namespace 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿 {

// On x86 most “special” registers (x87, mmx, xmm and so on) are “just registers” (xmm registers are bottom part of ymm registers
// which are bottom part of zmm registers, but that's it), but general purpose registers are significantly more complex. There are
// 20 8-bit ones and 16 16-bit/32-bit/64-bit ones which immediately makes 8-bit registers special. Also: many instruction only
// accept certain registers (or, when we talk about address, 𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯). To facilitate that the following heerarchy is used with
// 16-bit registers:
//                         ┌──────────────────────────────────────┐
//                         │ 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 │
//                         └──────────────────────────────────────┘
//                           ^
//                           │
//                           │
//                         ┌──────────────────────────────┐     ┌───────────────────────────────────────┐
//                         │  𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓  │ ──> │ 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 │
//                         └──────────────────────────────┘     └───────────────────────────────────────┘
//┌──────────────────────────────┐           │                  ┌─────────────────────────────┐
//│    𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓    │           │                  │        𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓        │
//└──────────────────────────────┘           │                  └─────────────────────────────┘
//                           ∧               │                    ∧
//                           │               │                    │
//                           │               v                    │
//┌──────────────────┐     ┌──────────────────────────────┐     ┌─────────────────────────────┐     ┌───────────────────────┐
//│ 𝒄𝒐𝒖𝒏𝒕𝒆𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 │ <── │                              │ ──> │    𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓    │ ──> │ 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 │
//└──────────────────┘     │         𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓          │     └─────────────────────────────┘     └───────────────────────┘
//┌──────────────────┐     │                              │     ┌─────────────────────────────┐
//│  𝒅𝒂𝒕𝒂_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓   │ <── │                              │ ──> │    𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓     │
//└──────────────────┘     └──────────────────────────────┘     └─────────────────────────────┘
//                           │
//                           │
//                           ∨
//                         ┌──────────────────────────────┐     ┌─────────────────────────────┐
//                         │    𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓    │ ──> │ 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓  │
//                         └──────────────────────────────┘     └─────────────────────────────┘
//                           │
//                           │
//                           ∨
//                         ┌──────────────────────────────┐
//                         │    𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓     │
//                         └──────────────────────────────┘
//
// Note that 𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 is not a descendant of 𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 and, similarly, 𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 is
// not a descendant of 𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓. Instead they have constructors which allow conversions. This is done to avoid
// multiple inherinance for efficiency, Also note that “𝔦𝔭”/“𝔢𝔦𝔭”/“𝔯𝔦𝔭” register is not a "general purpose" register on x86.
//
// 32-bit and 64-bit version is much simpler:
// ┌───────────────────────────────────────┐     ┌────────────────────────────┐     ┌─────────────┐     ┌────────────────────────┐
// │ 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 │ <── │ 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 │ ──> │ 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 │ ──> │ 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 │
// └───────────────────────────────────────┘     └────────────────────────────┘     └─────────────┘     └────────────────────────┘
//                                     ┌─────────────────────────────┐               │
//                                     │    𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓    │               │
//                                     └─────────────────────────────┘               │
//                                       ∧                                           │
//                                       │                                           │
//                                       │                                           v
//┌──────────────────────────────┐     ┌──────────────────────────────────────────────────────┐     ┌────────────────────────┐
//│        𝒅𝒂𝒕𝒂_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓         │ <── │                                                      │ ──> │  𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓  │
//└──────────────────────────────┘     │                𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓                │     └────────────────────────┘
//┌──────────────────────────────┐     │                                                      │     ┌────────────────────────┐
//│  𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓  │ <── │                                                      │ ──> │ 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓  │
//└──────────────────────────────┘     └──────────────────────────────────────────────────────┘     └────────────────────────┘
//                                       │                                   │
//                                       │                                   │
//                                       ∨                                   ∨
//                                     ┌─────────────────────────────┐     ┌──────────────────┐
//                                     │        𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓        │     │ 𝒄𝒐𝒖𝒏𝒕𝒆𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 │
//                                     └─────────────────────────────┘     └──────────────────┘
// Finally: 𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 and 𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 are 16-bit only in the 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿. 𝒃𝒂𝒔𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓 imports
// 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 as 𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 and 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 as 𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 for consistency.

#define 𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝓷𝓪𝓶𝓮) \
  template <𝐬𝐢𝐳𝐞 𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮 = 𝔵86_𝔪𝔬𝔡𝔢<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>> \
  class 𝓷𝓪𝓶𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓
𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊, 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐);
𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊, 𝒈𝒑, 𝒉𝒊𝒈𝒉_𝒈𝒑, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, 𝒔𝒊𝒛𝒆𝒅_𝒏𝒐, 𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓);
𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙);
#undef 𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊

// Using clause can not be specialized thus we need this indirection class.
template <𝐬𝐢𝐳𝐞 𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮 = 𝔵86_𝔪𝔬𝔡𝔢<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>>
class 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒊𝒎𝒑𝒐𝒓𝒕𝒆𝒓 {
 public:
  using 𝐭𝐲𝐩𝐞 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>;
};
template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
class 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒊𝒎𝒑𝒐𝒓𝒕𝒆𝒓<32, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> {
 public:
  using 𝐭𝐲𝐩𝐞 = 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<32, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>;
};
template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
class 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒊𝒎𝒑𝒐𝒓𝒕𝒆𝒓<64, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> {
 public:
  using 𝐭𝐲𝐩𝐞 = 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<64, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>;
};

// That one is used in 𝐚𝐝𝐝𝐫𝐞𝐬𝐬 class to signify that address doesn't include register.
// Note that it's also used for comparisons, etc.
inline constexpr class 𝐧𝐨_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 final {
} 𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯 = {};

// This one is used in 𝐚𝐝𝐝𝐫𝐞𝐬𝐬 class to signify that address doesn't include register, but it must be encoded as absent in SIB.
inline constexpr class 𝐦𝐚𝐤𝐞_ₓ𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 final {
} 𝔪𝔞𝔨𝔢_ₓ𝔦𝔷_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯 = {};

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝓷𝓪𝓶𝓮, 𝓯𝓲𝓷𝓪𝓵, 𝓼𝓲𝔃𝓮, 𝓹𝓪𝓻𝓮𝓷𝓽, 𝓼𝓪𝓯𝓮_𝓽𝔂𝓹𝓮𝓼, 𝓭𝓮𝓵𝓮𝓽𝓮𝓭_𝓽𝔂𝓹𝓮𝓼, ...) \
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼 𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖝𝟴𝟲_𝖒𝖔𝖉𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊_𝖕𝖆𝖗𝖆𝖒𝖆𝖙𝖊𝖗> \
  class 𝓷𝓪𝓶𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> 𝓯𝓲𝓷𝓪𝓵 : public 𝓹𝓪𝓻𝓮𝓷𝓽##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> { \
    𝖕𝖊𝖗𝖋𝖔𝖗𝖒_𝖘𝖙𝖆𝖙𝖎𝖈_𝖈𝖍𝖊𝖈𝖐𝖘 \
\
   public: \
    template <typename 𝒯, typename = std::enable_if_t<std::numeric_limits<𝒯>::is_integer>> \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝓷𝓪𝓶𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(𝒯 index) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) \
        : 𝓹𝓪𝓻𝓮𝓷𝓽##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>(𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(index)) {} \
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜( \
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗, \
        𝖞𝖆𝖈𝖊_𝖕𝖆𝖗𝖊𝖓𝖘((𝓷𝓪𝓶𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝓼𝓲𝔃𝓮, 𝓹𝓪𝓻𝓮𝓷𝓽##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, ), (), 𝖞𝖆𝖈𝖊_𝖚𝖓𝖇𝖗𝖆𝖈𝖊 𝓼𝓪𝓯𝓮_𝓽𝔂𝓹𝓮𝓼)) \
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗_𝖊𝖝𝖙𝖗𝖆(𝓷𝓪𝓶𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝓼𝓲𝔃𝓮, 𝓹𝓪𝓻𝓮𝓷𝓽##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓) \
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜( \
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖚𝖓𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗, \
        𝖞𝖆𝖈𝖊_𝖕𝖆𝖗𝖊𝖓𝖘((𝓷𝓪𝓶𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝓼𝓲𝔃𝓮, 𝓹𝓪𝓻𝓮𝓷𝓽##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, ), (), 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐, 𝒈𝒑)) \
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖉𝖊𝖑𝖊𝖙𝖊𝖉_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗, 𝖞𝖆𝖈𝖊_𝖕𝖆𝖗𝖊𝖓𝖘((𝓷𝓪𝓶𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝓼𝓲𝔃𝓮, ), (), 𝖞𝖆𝖈𝖊_𝖚𝖓𝖇𝖗𝖆𝖈𝖊 𝓭𝓮𝓵𝓮𝓽𝓮𝓭_𝓽𝔂𝓹𝓮𝓼)) \
    𝓷𝓪𝓶𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓() = delete; \
\
   private: \
    template <typename 𝒯, typename = std::enable_if_t<std::numeric_limits<𝒯>::is_integer and not std::is_same_v<𝒯, 𝐛𝐨𝐨𝐥>>> \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(𝒯 index) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) -> 𝐢𝐧𝐭₈ { \
      𝖕𝖊𝖗𝖋𝖔𝖗𝖒_𝖗𝖚𝖓𝖙𝖎𝖒𝖊_𝖈𝖍𝖊𝖈𝖐(__VA_ARGS__); \
      return 𝐢𝐧𝐭₈(index); \
    } \
  }
#define 𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖝𝟴𝟲_𝖒𝖔𝖉𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊_𝖕𝖆𝖗𝖆𝖒𝖆𝖙𝖊𝖗 , ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮
#define 𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖉𝖊𝖑𝖊𝖙𝖊𝖉_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗(𝓷𝓪𝓶𝓮, 𝓼𝓲𝔃𝓮, 𝓸𝓽𝓱𝓮𝓻_𝓷𝓪𝓶𝓮) \
  template <𝐬𝐢𝐳𝐞 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮> \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝓷𝓪𝓶𝓮(const 𝓸𝓽𝓱𝓮𝓻_𝓷𝓪𝓶𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>& r) = delete;
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖚𝖓𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗(𝓷𝓪𝓶𝓮, 𝓼𝓲𝔃𝓮, 𝓹𝓪𝓻𝓮𝓷𝓽, 𝓸𝓽𝓱𝓮𝓻_𝓷𝓪𝓶𝓮) \
  template <𝐬𝐢𝐳𝐞 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮> \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝓷𝓪𝓶𝓮(const 𝓸𝓽𝓱𝓮𝓻_𝓷𝓪𝓶𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>& r) noexcept( \
      𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) \
      : 𝓹𝓪𝓻𝓮𝓷𝓽<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>(𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(𝐢𝐧𝐭₈(r))) {} /* NOLINT(bugprone-macro-parentheses) */
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗(𝓷𝓪𝓶𝓮, 𝓼𝓲𝔃𝓮, 𝓹𝓪𝓻𝓮𝓷𝓽, 𝓸𝓽𝓱𝓮𝓻_𝓷𝓪𝓶𝓮) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr 𝓷𝓪𝓶𝓮(const 𝓸𝓽𝓱𝓮𝓻_𝓷𝓪𝓶𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>& r) noexcept \
      : 𝓹𝓪𝓻𝓮𝓷𝓽<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>(r) {} /* NOLINT(bugprone-macro-parentheses) */
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗_𝖊𝖝𝖙𝖗𝖆(𝓷𝓪𝓶𝓮, 𝓼𝓲𝔃𝓮, 𝓹𝓪𝓻𝓮𝓷𝓽) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝓷𝓪𝓶𝓮(𝐧𝐨_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 no_register) noexcept \
      : 𝓹𝓪𝓻𝓮𝓷𝓽<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>(no_register) {} /* NOLINT(bugprone-macro-parentheses) */ \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝓷𝓪𝓶𝓮(𝐦𝐚𝐤𝐞_ₓ𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 make_ₓiz_register) noexcept \
      : 𝓹𝓪𝓻𝓮𝓷𝓽<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>(make_ₓiz_register) {} /* NOLINT(bugprone-macro-parentheses) */
#define 𝖕𝖊𝖗𝖋𝖔𝖗𝖒_𝖗𝖚𝖓𝖙𝖎𝖒𝖊_𝖈𝖍𝖊𝖈𝖐(...) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖔𝖓𝖊_𝖔𝖋(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, index, 𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖑𝖎𝖘𝖙(𝒯, __VA_ARGS__))

#define 𝖕𝖊𝖗𝖋𝖔𝖗𝖒_𝖘𝖙𝖆𝖙𝖎𝖈_𝖈𝖍𝖊𝖈𝖐𝖘 static_assert(𝔁86_𝓶𝓸𝓭𝓮 <= ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32);
// clang-format off
// NOLINTNEXTLINE(google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐,, 16, 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐, (𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆, 𝒔𝒊𝒛𝒆𝒅_𝒏𝒐), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙), 3 /*𝔟𝔵*/, 5 /*𝔟𝔭*/);
// NOLINTNEXTLINE(google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐,, 16, 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐, (𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓), 6 /*𝔟𝔵*/, 7 /*𝔟𝔭*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒔𝒊𝒛𝒆𝒅_𝒏𝒐, final, 16, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐, (𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), -1 /*𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯*/);

#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗(𝓷𝓪𝓶𝓮, 𝓼𝓲𝔃𝓮, 𝓹𝓪𝓻𝓮𝓷𝓽, 𝓸𝓽𝓱𝓮𝓻_𝓷𝓪𝓶𝓮)

#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗_𝖊𝖝𝖙𝖗𝖆
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗_𝖊𝖝𝖙𝖗𝖆(𝓷𝓪𝓶𝓮, 𝓼𝓲𝔃𝓮, 𝓹𝓪𝓻𝓮𝓷𝓽)

𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆,, 16, 𝒈𝒑, (𝔶𝔞𝔠𝔢_𝔡𝔲𝔪𝔪𝔶), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙), 3 /*𝔟𝔵*/, 5 /*𝔟𝔭*/);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙,, 16, 𝒈𝒑, (𝔶𝔞𝔠𝔢_𝔡𝔲𝔪𝔪𝔶), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓), 6 /*𝔟𝔵*/, 7 /*𝔟𝔭*/);

#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗(𝓷𝓪𝓶𝓮, 𝓼𝓲𝔃𝓮, 𝓹𝓪𝓻𝓮𝓷𝓽, 𝓸𝓽𝓱𝓮𝓻_𝓷𝓪𝓶𝓮) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr 𝓷𝓪𝓶𝓮(const 𝓸𝓽𝓱𝓮𝓻_𝓷𝓪𝓶𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>& r) noexcept \
      : 𝓹𝓪𝓻𝓮𝓷𝓽<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>(r) {} /* NOLINT(bugprone-macro-parentheses) */ \
  template <𝐬𝐢𝐳𝐞 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮> \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝓷𝓪𝓶𝓮(const 𝓸𝓽𝓱𝓮𝓻_𝓷𝓪𝓶𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>& r) noexcept \
      : 𝓹𝓪𝓻𝓮𝓷𝓽<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>(r) {} /* NOLINT(bugprone-macro-parentheses) */

// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒃𝒂𝒔𝒆, final, 16, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆, (𝒃𝒂𝒔𝒆), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙), 3 /*𝔟𝔵*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, final, 16, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆, (𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙), 5 /*𝔟𝔭*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, final, 16, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, (𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙), 6 /*𝔰𝔦*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, final, 16, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, (𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙), 7 /*𝔡𝔦*/);

#undef 𝖕𝖊𝖗𝖋𝖔𝖗𝖒_𝖘𝖙𝖆𝖙𝖎𝖈_𝖈𝖍𝖊𝖈𝖐𝖘
#define 𝖕𝖊𝖗𝖋𝖔𝖗𝖒_𝖘𝖙𝖆𝖙𝖎𝖈_𝖈𝖍𝖊𝖈𝖐𝖘

// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, final, 8, 𝒈𝒑, (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓), (𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 0 /*𝔞𝔩*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒄𝒐𝒖𝒏𝒕𝒆𝒓, final, 8, 𝒈𝒑, (𝒄𝒐𝒖𝒏𝒕𝒆𝒓), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 1 /*𝔠𝔩*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, final, 16, 𝒈𝒑, (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓), (𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 0 /*𝔞𝔵*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒄𝒐𝒖𝒏𝒕𝒆𝒓, final, 16, 𝒈𝒑, (𝒄𝒐𝒖𝒏𝒕𝒆𝒓), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 1 /*𝔠𝔵*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒅𝒂𝒕𝒂, final, 16, 𝒈𝒑, (𝒅𝒂𝒕𝒂), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 2 /*𝔡𝔵*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, final, 16, 𝒈𝒑, (𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 4 /*𝔰𝔭*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, final, 32, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓), (𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 0 /*𝔢𝔞𝔵*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒄𝒐𝒖𝒏𝒕𝒆𝒓, final, 32, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, (𝒄𝒐𝒖𝒏𝒕𝒆𝒓), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 1 /*𝔢𝔠𝔵*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒅𝒂𝒕𝒂, final, 32, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, (𝒅𝒂𝒕𝒂), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 2 /*𝔢𝔡𝔵*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒃𝒂𝒔𝒆, final, 32, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, (𝒃𝒂𝒔𝒆), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 3 /*𝔢𝔟𝔵*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, final, 32, 𝒈𝒑, (𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 4 /*𝔢𝔰𝔭*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, final, 32, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, (𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 5 /*𝔢𝔟𝔭*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, final, 32, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, (𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 6 /*𝔢𝔰𝔦*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, final, 32, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, (𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 7 /*𝔢𝔡𝔦*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, final, 64, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓), (𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 0 /*𝔯𝔞𝔵*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒄𝒐𝒖𝒏𝒕𝒆𝒓, final, 64, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, (𝒄𝒐𝒖𝒏𝒕𝒆𝒓), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 1 /*𝔯𝔠𝔵*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒅𝒂𝒕𝒂, final, 64, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, (𝒅𝒂𝒕𝒂), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 2 /*𝔯𝔡𝔵*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒃𝒂𝒔𝒆, final, 64, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, (𝒃𝒂𝒔𝒆), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 3 /*𝔯𝔟𝔵*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, final, 64, 𝒈𝒑, (𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 4 /*𝔯𝔰𝔭*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, final, 64, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, (𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 5 /*𝔯𝔟𝔭*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, final, 64, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, (𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 6 /*𝔯𝔰𝔦*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, final, 64, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, (𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 7 /*𝔯𝔡𝔦*/);

#undef 𝖕𝖊𝖗𝖋𝖔𝖗𝖒_𝖗𝖚𝖓𝖙𝖎𝖒𝖊_𝖈𝖍𝖊𝖈𝖐
#define 𝖕𝖊𝖗𝖋𝖔𝖗𝖒_𝖗𝖚𝖓𝖙𝖎𝖒𝖊_𝖈𝖍𝖊𝖈𝖐(...) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖎𝖓_𝖗𝖆𝖓𝖌𝖊(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, index, __VA_ARGS__)

// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒉𝒊𝒈𝒉_𝒈𝒑, final, 16, 𝒈𝒑, (𝒉𝒊𝒈𝒉_𝒈𝒑), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐), 8, 16);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒉𝒊𝒈𝒉_𝒈𝒑, final, 32, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, (𝒉𝒊𝒈𝒉_𝒈𝒑), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙), 8, 16);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒉𝒊𝒈𝒉_𝒈𝒑, final, 64, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, (𝒉𝒊𝒈𝒉_𝒈𝒑), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙), 8, 16);

#undef 𝖕𝖊𝖗𝖋𝖔𝖗𝖒_𝖗𝖚𝖓𝖙𝖎𝖒𝖊_𝖈𝖍𝖊𝖈𝖐
#define 𝖕𝖊𝖗𝖋𝖔𝖗𝖒_𝖗𝖚𝖓𝖙𝖎𝖒𝖊_𝖈𝖍𝖊𝖈𝖐(...) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖔𝖓𝖊_𝖔𝖋(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, index, 𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖑𝖎𝖘𝖙(𝒯, __VA_ARGS__))

#undef 𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖝𝟴𝟲_𝖒𝖔𝖉𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊_𝖕𝖆𝖗𝖆𝖒𝖆𝖙𝖊𝖗
#define 𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖝𝟴𝟲_𝖒𝖔𝖉𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊_𝖕𝖆𝖗𝖆𝖒𝖆𝖙𝖊𝖗

#define 𝔁86_𝓶𝓸𝓭𝓮 ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒃𝒂𝒔𝒆, final, 16, 𝒈𝒑, (𝒃𝒂𝒔𝒆), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 3 /*𝔟𝔵*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, final, 16, 𝒈𝒑, (𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 5 /*𝔟𝔭*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, final, 16, 𝒈𝒑, (𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 6 /*𝔰𝔦*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, final, 16, 𝒈𝒑, (𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 7 /*𝔡𝔦*/);
#undef 𝔁86_𝓶𝓸𝓭𝓮

#undef 𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖝𝟴𝟲_𝖒𝖔𝖉𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊_𝖕𝖆𝖗𝖆𝖒𝖆𝖙𝖊𝖗
#define 𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖝𝟴𝟲_𝖒𝖔𝖉𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊_𝖕𝖆𝖗𝖆𝖒𝖆𝖙𝖊𝖗 , ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮

#undef 𝖕𝖊𝖗𝖋𝖔𝖗𝖒_𝖗𝖚𝖓𝖙𝖎𝖒𝖊_𝖈𝖍𝖊𝖈𝖐
#define 𝖕𝖊𝖗𝖋𝖔𝖗𝖒_𝖗𝖚𝖓𝖙𝖎𝖒𝖊_𝖈𝖍𝖊𝖈𝖐(...) 𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖓𝖔𝖙_𝖊𝖖𝖚𝖆𝖑(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, index, 𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖑𝖎𝖘𝖙(𝒯, __VA_ARGS__))
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗(𝓷𝓪𝓶𝓮, 𝓼𝓲𝔃𝓮, 𝓹𝓪𝓻𝓮𝓷𝓽, 𝓸𝓽𝓱𝓮𝓻_𝓷𝓪𝓶𝓮) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr 𝓷𝓪𝓶𝓮(const 𝓸𝓽𝓱𝓮𝓻_𝓷𝓪𝓶𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>& r) noexcept \
      : 𝓹𝓪𝓻𝓮𝓷𝓽<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>(r) {} /* NOLINT(bugprone-macro-parentheses) */ \
  template <𝐬𝐢𝐳𝐞 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮> \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝓷𝓪𝓶𝓮(const 𝓸𝓽𝓱𝓮𝓻_𝓷𝓪𝓶𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>& r) noexcept \
      : 𝓹𝓪𝓻𝓮𝓷𝓽<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>(r) {} /* NOLINT(bugprone-macro-parentheses) */
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗_𝖊𝖝𝖙𝖗𝖆
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗_𝖊𝖝𝖙𝖗𝖆(𝓷𝓪𝓶𝓮, 𝓼𝓲𝔃𝓮, 𝓹𝓪𝓻𝓮𝓷𝓽) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝓷𝓪𝓶𝓮(𝐧𝐨_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 no_register) noexcept \
      : 𝓹𝓪𝓻𝓮𝓷𝓽<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>(no_register) {} /* NOLINT(bugprone-macro-parentheses) */ \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝓷𝓪𝓶𝓮(𝐦𝐚𝐤𝐞_ₓ𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 make_ₓiz_register) noexcept \
      : 𝓹𝓪𝓻𝓮𝓷𝓽<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>(make_ₓiz_register) {} /* NOLINT(bugprone-macro-parentheses) */

// NOLINTNEXTLINE(google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐,, 32, 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐, (𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, 𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), (𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓), 4 /*𝔢𝔰𝔭*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒔𝒊𝒛𝒆𝒅_𝒏𝒐, final, 32, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐, (𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 0 /*𝔞𝔩*/);
// NOLINTNEXTLINE(google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐,, 64, 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐, (𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙, 𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), (𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓), 4 /*𝔯𝔰𝔭*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒔𝒊𝒛𝒆𝒅_𝒏𝒐, final, 64, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐, (𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐), (𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓, 𝒅𝒂𝒕𝒂, 𝒃𝒂𝒔𝒆, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙, 𝒉𝒊𝒈𝒉_𝒈𝒑), 0 /*𝔞𝔩*/);

#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗_𝖊𝖝𝖙𝖗𝖆
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗_𝖊𝖝𝖙𝖗𝖆(𝓷𝓪𝓶𝓮, 𝓼𝓲𝔃𝓮, 𝓹𝓪𝓻𝓮𝓷𝓽)

// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙,, 32, 𝒈𝒑, (𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙), (𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓), 4 /*𝔢𝔰𝔭*/);
// NOLINTNEXTLINE(cppcoreguidelines-special-member-functions, hicpp-special-member-functions,google-explicit-constructor, hicpp-explicit-conversions)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙,, 64, 𝒈𝒑, (𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙), (𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓), 4 /*𝔯𝔰𝔭*/);

#undef 𝖕𝖊𝖗𝖋𝖔𝖗𝖒_𝖗𝖚𝖓𝖙𝖎𝖒𝖊_𝖈𝖍𝖊𝖈𝖐
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗_𝖊𝖝𝖙𝖗𝖆
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖚𝖓𝖘𝖆𝖋𝖊_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗
#undef 𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖉𝖊𝖑𝖊𝖙𝖊𝖉_𝖈𝖔𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗
#undef 𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖝𝟴𝟲_𝖒𝖔𝖉𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊_𝖕𝖆𝖗𝖆𝖒𝖆𝖙𝖊𝖗
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊
// clang-format on

// Tag for 8-bit registers - support %al/cl/dl/bl and %al/cl/dl/bl.
inline constexpr class 𝐧𝐨𝐫𝐞𝐱 final {
} 𝔫𝔬𝔯𝔢𝔵 = {};

// Tag for 8-bit registers - support %ah/ch/dh/bh.
inline constexpr class 𝐱𝐡_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 final {
} 𝔵𝔥_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯 = {};

// Note: “al” has index 0. There are also 4 negative indexes for “ah”, “ch”, “dh”, and “bh”.
inline constexpr 𝒔𝒉𝒐𝒓𝒕_𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆 𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯₈_𝔫𝔞𝔪𝔢𝔰₈_₈₀₈₆{𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖑𝖎𝖘𝖙(𝖞𝖆𝖈𝖊_𝖚𝟴, ah, ch, dh, bh, al, cl, dl, bl)};

inline constexpr 𝒔𝒉𝒐𝒓𝒕_𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆 𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯₈_𝔫𝔞𝔪𝔢𝔰₈_ₓ₈₆_₆₄{
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖑𝖎𝖘𝖙(𝖞𝖆𝖈𝖊_𝖚𝟴, ah, ch, dh, bh, al, cl, dl, bl, spl, bpl, sil, dil, r8b, r9b, r10b, r11b, r12b, r13b, r14b, r15b)};

template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
class 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<8, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> {
 public:
  template <typename 𝒯, typename = std::enable_if_t<std::numeric_limits<𝒯>::is_integer>>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(𝒯 index) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) : 𝗂𝗇𝖽𝖾𝗑{𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(index)} {}
  template <typename 𝒯, typename = std::enable_if_t<std::numeric_limits<𝒯>::is_integer>>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(𝒯 index, 𝐧𝐨𝐫𝐞𝐱 norex) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>)
      : 𝗂𝗇𝖽𝖾𝗑{𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(index, norex)} {}
  template <𝐬𝐢𝐳𝐞 𝓼𝓲𝔃𝓮>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(const 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>& r) noexcept(
      𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>)
      : 𝗂𝗇𝖽𝖾𝗑{𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(𝐢𝐧𝐭₈(r))} {}
  template <𝐬𝐢𝐳𝐞 𝓼𝓲𝔃𝓮>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(
      const 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>& r,
      𝐱𝐡_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 xh_register) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>)
      : 𝗂𝗇𝖽𝖾𝗑{𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(𝐢𝐧𝐭₈(r), xh_register)} {}
  𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓() = delete;

  // Note: when we comparing registers we are using 𝐮𝐢𝐧𝐭₈ for 8-bit registers and 𝐢𝐧𝐭₈ for 16-bit/32-bit/64-bit ones.
  // This allows us to sort 𝔞𝔩 before 𝔞𝔥, but, more importantly, it guarantees that 𝔟𝔥 is not treated as equal to 𝔦𝔭!
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓(𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷, 𝓻𝓮𝓽𝓾𝓻𝓷_𝓽𝔂𝓹𝓮) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto operator 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷(const 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓& r) const noexcept->𝓻𝓮𝓽𝓾𝓻𝓷_𝓽𝔂𝓹𝓮 { \
    return 𝐮𝐢𝐧𝐭₈(𝗂𝗇𝖽𝖾𝗑) 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷 𝐮𝐢𝐧𝐭₈(r.𝗂𝗇𝖽𝖾𝗑); /* NOLINTNEXTLINE(bugprone-macro-parentheses) */ \
  } \
  template <𝐬𝐢𝐳𝐞 𝓼𝓲𝔃𝓮> \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto operator 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷(const 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>& r) \
      const noexcept->𝓻𝓮𝓽𝓾𝓻𝓷_𝓽𝔂𝓹𝓮 { \
    return int(𝐮𝐢𝐧𝐭₈(𝗂𝗇𝖽𝖾𝗑)) 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷 int(𝐢𝐧𝐭₈(r)); /* NOLINTNEXTLINE(bugprone-macro-parentheses) */ \
  } \
  template <𝐬𝐢𝐳𝐞 𝓼𝓲𝔃𝓮> \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 friend constexpr auto operator 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷( \
      const 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>& rₓₓ, const 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓& r₈) noexcept->𝓻𝓮𝓽𝓾𝓻𝓷_𝓽𝔂𝓹𝓮 { \
    return int(𝐢𝐧𝐭₈(rₓₓ)) 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷 int(𝐮𝐢𝐧𝐭₈(r₈.𝗂𝗇𝖽𝖾𝗑)); /* NOLINTNEXTLINE(bugprone-macro-parentheses) */ \
  }
#ifdef __cpp_impl_three_way_comparison
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓, (==, 𝐛𝐨𝐨𝐥), (<=>, std::strong_ordering))
#else
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓, (==, 𝐛𝐨𝐨𝐥), (!=, 𝐛𝐨𝐨𝐥), (<, 𝐛𝐨𝐨𝐥), (<=, 𝐛𝐨𝐨𝐥), (>=, 𝐛𝐨𝐨𝐥), (>, 𝐛𝐨𝐨𝐥))
#endif
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓

  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr operator 𝐢𝐧𝐭₈() const noexcept { return 𝗂𝗇𝖽𝖾𝗑; }

  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 friend constexpr auto to_string(const 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓& reg) noexcept {
    if constexpr (𝔁86_𝓶𝓸𝓭𝓮 <= ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32) {
      return 𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯₈_𝔫𝔞𝔪𝔢𝔰₈_₈₀₈₆[reg.𝗂𝗇𝖽𝖾𝗑 + 4];
    } else {
      return 𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯₈_𝔫𝔞𝔪𝔢𝔰₈_ₓ₈₆_₆₄[reg.𝗂𝗇𝖽𝖾𝗑 + 4];
    }
  }

 private:
  template <typename 𝒯>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(𝒯 index) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) -> 𝐢𝐧𝐭₈ {
    static_assert(std::numeric_limits<𝒯>::is_integer);
    if constexpr (𝔁86_𝓶𝓸𝓭𝓮 <= ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32) {
      𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖎𝖓_𝖗𝖆𝖓𝖌𝖊(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, index, 0, 4);
    } else {
      𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖎𝖓_𝖗𝖆𝖓𝖌𝖊(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, index, 0, 16);
    }
    return 𝐢𝐧𝐭₈(index);
  }
  template <typename 𝒯>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(𝒯 index, 𝐧𝐨𝐫𝐞𝐱 /*norex*/) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) -> 𝐢𝐧𝐭₈ {
    static_assert(std::numeric_limits<𝒯>::is_integer);
    𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖎𝖓_𝖗𝖆𝖓𝖌𝖊(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, index, 0, 8);
    return 𝐢𝐧𝐭₈(index bitand 0x03) - 𝐢𝐧𝐭₈(index bitand 0x04);  // NOLINT(hicpp-signed-bitwise)
  }
  template <typename 𝒯, typename = std::enable_if_t<std::numeric_limits<𝒯>::is_integer>>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(𝒯 index, 𝐱𝐡_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 /*xh_register*/) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) -> 𝐢𝐧𝐭₈ {
    𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖎𝖓_𝖗𝖆𝖓𝖌𝖊(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, index, 0, 4);
    return 𝐢𝐧𝐭₈(index bitand 0x03) - 4;  // NOLINT(hicpp-signed-bitwise)
  }

  𝐢𝐧𝐭₈ 𝗂𝗇𝖽𝖾𝗑;
};

template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
inline constexpr 𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<8, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> 𝔞𝔩{0};
template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
inline constexpr 𝒄𝒐𝒖𝒏𝒕𝒆𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<8, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> 𝔠𝔩{1};
#define 𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗(𝓷𝓪𝓶𝓮, 𝓲𝓷𝓭𝓮𝔁) \
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮> \
  inline constexpr 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<8, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> 𝓷𝓪𝓶𝓮{𝓲𝓷𝓭𝓮𝔁, 𝔫𝔬𝔯𝔢𝔵}; /* NOLINTNEXTLINE(bugprone-macro-parentheses) */
𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗, (𝔡𝔩, 2), (𝔟𝔩, 3), (𝔞𝔥, 4), (𝔠𝔥, 5), (𝔡𝔥, 6), (𝔟𝔥, 7))
#undef 𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗
#define 𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗(𝓷𝓪𝓶𝓮, 𝓲𝓷𝓭𝓮𝔁) \
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮> \
  inline constexpr 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<8, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> 𝓷𝓪𝓶𝓮{𝓲𝓷𝓭𝓮𝔁}; /* NOLINTNEXTLINE(bugprone-macro-parentheses) */
𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗, (𝔰𝔭𝔩, 4), (𝔟𝔭𝔩, 5), (𝔰𝔦𝔩, 6), (𝔡𝔦𝔩, 7))
𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗, (𝔯8𝔟, 8), (𝔯9𝔟, 9), (𝔯10𝔟, 10), (𝔯11𝔟, 11), (𝔯12𝔟, 12), (𝔯13𝔟, 13), (𝔯14𝔟, 14), (𝔯15𝔟, 15))
#undef 𝖉𝖊𝖈𝖑𝖆𝖗𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗(𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁, 𝓷𝓪𝓶𝓮_𝓼𝓾𝓯𝓯𝓲𝔁, 𝓿𝓪𝓻_𝓹𝓻𝓮𝓯𝓲𝔁, 𝓿𝓪𝓻_𝓼𝓾𝓯𝓯𝓲𝔁, 𝓰𝓹_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼_𝓼𝓱𝓸𝓻𝓽, 𝓰𝓹_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼_𝓯𝓾𝓵𝓵) \
\
  /* Note: “ax” has index 0. There are also -1 reserved for “ip”. */ \
  inline constexpr 𝒔𝒉𝒐𝒓𝒕_𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆 𝓰𝓹_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼_𝓼𝓱𝓸𝓻𝓽{ \
      𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"iz", \
      u8"", \
      𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"ip", \
      u8"", \
      𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"ax", \
      𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"cx", \
      𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"dx", \
      𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"bx", \
      𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"sp", \
      𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"bp", \
      𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"si", \
      𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"di"}; \
\
  inline constexpr 𝒔𝒉𝒐𝒓𝒕_𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆 𝓰𝓹_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼_𝓯𝓾𝓵𝓵{𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"iz",  u8"", \
                                                        𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"ip",  u8"", \
                                                        𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"ax",  𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"cx", \
                                                        𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"dx",  𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"bx", \
                                                        𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"sp",  𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"bp", \
                                                        𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"si",  𝓷𝓪𝓶𝓮_𝓹𝓻𝓮𝓯𝓲𝔁 u8"di", \
                                                        u8"r8" 𝓷𝓪𝓶𝓮_𝓼𝓾𝓯𝓯𝓲𝔁,  u8"r9" 𝓷𝓪𝓶𝓮_𝓼𝓾𝓯𝓯𝓲𝔁, \
                                                        u8"r10" 𝓷𝓪𝓶𝓮_𝓼𝓾𝓯𝓯𝓲𝔁, u8"r11" 𝓷𝓪𝓶𝓮_𝓼𝓾𝓯𝓯𝓲𝔁, \
                                                        u8"r12" 𝓷𝓪𝓶𝓮_𝓼𝓾𝓯𝓯𝓲𝔁, u8"r13" 𝓷𝓪𝓶𝓮_𝓼𝓾𝓯𝓯𝓲𝔁, \
                                                        u8"r14" 𝓷𝓪𝓶𝓮_𝓼𝓾𝓯𝓯𝓲𝔁, u8"r15" 𝓷𝓪𝓶𝓮_𝓼𝓾𝓯𝓯𝓲𝔁}; \
\
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮> \
  class 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> { \
    𝖕𝖊𝖗𝖋𝖔𝖗𝖒_𝖘𝖙𝖆𝖙𝖎𝖈_𝖈𝖍𝖊𝖈𝖐𝖘 \
\
   public: \
    template <typename 𝒯, typename = std::enable_if_t<std::numeric_limits<𝒯>::is_integer>> \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(𝒯 index) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) \
        : 𝗂𝗇𝖽𝖾𝗑{𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(index)} {} \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(const 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<8, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>& r) noexcept( \
        𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) \
        : 𝗂𝗇𝖽𝖾𝗑{𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(𝐢𝐧𝐭₈(r))} {} \
    template <𝐬𝐢𝐳𝐞 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮> \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓( \
        const 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>& r) noexcept \
        : 𝗂𝗇𝖽𝖾𝗑{𝐢𝐧𝐭₈(r)} {} \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(𝐧𝐨_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 /*no_register*/) noexcept : 𝗂𝗇𝖽𝖾𝗑{-1} {} \
    /* Note: -4 is picked for ₓ𝔦𝔷 mark because it ensires than SIB would be generated properly */ \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(𝐦𝐚𝐤𝐞_ₓ𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 /*make_ₓiz_register*/) noexcept \
        : 𝗂𝗇𝖽𝖾𝗑{-4} {} \
    𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓() = delete; \
\
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓𝖘 \
\
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr operator 𝐢𝐧𝐭₈() const noexcept { return 𝗂𝗇𝖽𝖾𝗑; } \
\
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 friend constexpr auto to_string(𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 reg) noexcept { \
      if constexpr (𝔁86_𝓶𝓸𝓭𝓮 <= ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32) { \
        return (𝓰𝓹_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼_𝓼𝓱𝓸𝓻𝓽)[reg.𝗂𝗇𝖽𝖾𝗑 + 4]; \
      } else { \
        return (𝓰𝓹_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼_𝓯𝓾𝓵𝓵)[reg.𝗂𝗇𝖽𝖾𝗑 + 4]; \
      } \
    } \
\
   private: \
    template <typename 𝒯, typename = std::enable_if_t<std::numeric_limits<𝒯>::is_integer>> \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(𝒯 index) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) -> 𝐢𝐧𝐭₈ { \
      if constexpr (𝔁86_𝓶𝓸𝓭𝓮 <= ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32) { \
        𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖎𝖓_𝖗𝖆𝖓𝖌𝖊(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, index, 0, 8); \
      } else { \
        𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖎𝖓_𝖗𝖆𝖓𝖌𝖊(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, index, 0, 16); \
      } \
      return 𝐢𝐧𝐭₈(index); \
    } \
\
    𝐢𝐧𝐭₈ 𝗂𝗇𝖽𝖾𝗑; \
  }; \
\
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮> \
  class 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> : public 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> { \
   public: \
    template <typename 𝒯, typename = std::enable_if_t<std::numeric_limits<𝒯>::is_integer>> \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(𝒯 index) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) \
        : 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>(index) {} \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(const 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<8, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>& r) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) \
        : 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>(r) {} \
    template <𝐬𝐢𝐳𝐞 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮> \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(const 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>& r) noexcept \
        : 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>(r) {} \
    𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓() = delete; \
  }; \
\
  𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊𝖘(𝓼𝓲𝔃𝓮, 𝓿𝓪𝓻_𝓹𝓻𝓮𝓯𝓲𝔁, 𝓿𝓪𝓻_𝓼𝓾𝓯𝓯𝓲𝔁);

#ifdef __cpp_impl_three_way_comparison
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓𝖘 𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓, (==, 𝐛𝐨𝐨𝐥), (<=>, std::strong_ordering))
#else
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓𝖘 𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓, (==, 𝐛𝐨𝐨𝐥), (!=, 𝐛𝐨𝐨𝐥), (<, 𝐛𝐨𝐨𝐥), (<=, 𝐛𝐨𝐨𝐥), (>, 𝐛𝐨𝐨𝐥), (>=, 𝐛𝐨𝐨𝐥))
#endif
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓(𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷, 𝓻𝓮𝓽𝓾𝓻𝓷_𝓽𝔂𝓹𝓮) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto operator 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷(const 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓& r) const noexcept->𝓻𝓮𝓽𝓾𝓻𝓷_𝓽𝔂𝓹𝓮 { \
    return 𝗂𝗇𝖽𝖾𝗑 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷 r.𝗂𝗇𝖽𝖾𝗑; \
  } \
  template <𝐬𝐢𝐳𝐞 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮> \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto operator 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷(const 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>& r) \
      const noexcept->std::enable_if_t<𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮 != 8, 𝓻𝓮𝓽𝓾𝓻𝓷_𝓽𝔂𝓹𝓮> { \
    return 𝐢𝐧𝐭₈(𝗂𝗇𝖽𝖾𝗑) 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷 𝐢𝐧𝐭₈(r); \
  } \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto operator 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷(const 𝐧𝐨_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫& no_register) const noexcept->𝓻𝓮𝓽𝓾𝓻𝓷_𝓽𝔂𝓹𝓮 { \
    return operator 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷(𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(no_register)); \
  } \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 friend constexpr auto operator 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷( \
      const 𝐧𝐨_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫& no_register, const 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓& r) noexcept->𝓻𝓮𝓽𝓾𝓻𝓷_𝓽𝔂𝓹𝓮 { \
    return 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(no_register) 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷 r; \
  }

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊𝖘(𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 𝓼𝓾𝓯𝓯𝓲𝔁) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊( \
      𝖉𝖊𝖋𝖎𝖓𝖊_𝖑𝖔𝖜_𝖌𝖕_𝖗𝖊𝖌_𝖛𝖆𝖗, \
      (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝔞𝔵, 0), \
      (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝔠𝔵, 1), \
      (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 𝒅𝒂𝒕𝒂_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝔡𝔵, 2), \
      (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝔟𝔵, 3), \
      (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝔰𝔭, 4), \
      (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝔟𝔭, 5), \
      (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝔰𝔦, 6), \
      (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝔡𝔦, 7), \
      (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 𝒔𝒊𝒛𝒆𝒅_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝔦𝔷, 𝔪𝔞𝔨𝔢_ₓ𝔦𝔷_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯)); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊( \
      𝖉𝖊𝖋𝖎𝖓𝖊_𝖍𝖎𝖌𝖍_𝖌𝖕_𝖗𝖊𝖌_𝖛𝖆𝖗, (𝓼𝓲𝔃𝓮, 𝔯8, 𝓼𝓾𝓯𝓯𝓲𝔁, 8), (𝓼𝓲𝔃𝓮, 𝔯9, 𝓼𝓾𝓯𝓯𝓲𝔁, 9), (𝓼𝓲𝔃𝓮, 𝔯10, 𝓼𝓾𝓯𝓯𝓲𝔁, 10), (𝓼𝓲𝔃𝓮, 𝔯11, 𝓼𝓾𝓯𝓯𝓲𝔁, 11)); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊( \
      𝖉𝖊𝖋𝖎𝖓𝖊_𝖍𝖎𝖌𝖍_𝖌𝖕_𝖗𝖊𝖌_𝖛𝖆𝖗, (𝓼𝓲𝔃𝓮, 𝔯12, 𝓼𝓾𝓯𝓯𝓲𝔁, 12), (𝓼𝓲𝔃𝓮, 𝔯13, 𝓼𝓾𝓯𝓯𝓲𝔁, 13), (𝓼𝓲𝔃𝓮, 𝔯14, 𝓼𝓾𝓯𝓯𝓲𝔁, 14), (𝓼𝓲𝔃𝓮, 𝔯15, 𝓼𝓾𝓯𝓯𝓲𝔁, 15))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖑𝖔𝖜_𝖌𝖕_𝖗𝖊𝖌_𝖛𝖆𝖗(𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭) \
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮> \
  inline constexpr 𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> 𝓹𝓻𝓮𝓯𝓲𝔁##𝓷𝓪𝓶𝓮 { /* NOLINT(bugprone-macro-parentheses) */ \
    𝓲𝓭 \
  }
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖍𝖎𝖌𝖍_𝖌𝖕_𝖗𝖊𝖌_𝖛𝖆𝖗(𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓼𝓾𝓯𝓯𝓲𝔁, 𝓲𝓭) \
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮> \
  inline constexpr 𝒉𝒊𝒈𝒉_𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> 𝓷𝓪𝓶𝓮##𝓼𝓾𝓯𝓯𝓲𝔁 { /* NOLINT(bugprone-macro-parentheses) */ \
    𝓲𝓭 \
  }

𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗(16, u8"", u8"w", , 𝔴, 𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯₈_𝔫𝔞𝔪𝔢𝔰₁₆_₈₀₈₆, 𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯₈_𝔫𝔞𝔪𝔢𝔰₁₆_ₓ₈₆_₆₄);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗(32, u8"e", u8"d", 𝔢, 𝔡, 𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯₈_𝔫𝔞𝔪𝔢𝔰₃₂_₈₀₃₈₆, 𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯₈_𝔫𝔞𝔪𝔢𝔰₃₂_ₓ₈₆_₆₄);
#undef 𝖕𝖊𝖗𝖋𝖔𝖗𝖒_𝖘𝖙𝖆𝖙𝖎𝖈_𝖈𝖍𝖊𝖈𝖐𝖘
#define 𝖕𝖊𝖗𝖋𝖔𝖗𝖒_𝖘𝖙𝖆𝖙𝖎𝖈_𝖈𝖍𝖊𝖈𝖐𝖘 static_assert(𝔁86_𝓶𝓸𝓭𝓮 >= ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗(64, u8"r", u8"", 𝔯, , 𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯₈_𝔫𝔞𝔪𝔢𝔰₆₄_ₛₕₒᵣₜ, 𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯₈_𝔫𝔞𝔪𝔢𝔰₆₄);
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖍𝖎𝖌𝖍_𝖌𝖕_𝖗𝖊𝖌_𝖛𝖆𝖗
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖑𝖔𝖜_𝖌𝖕_𝖗𝖊𝖌_𝖛𝖆𝖗
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊𝖘
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓𝖘
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗
#undef 𝖕𝖊𝖗𝖋𝖔𝖗𝖒_𝖘𝖙𝖆𝖙𝖎𝖈_𝖈𝖍𝖊𝖈𝖐𝖘

template <typename 𝓣>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯 = false;

template <𝐬𝐢𝐳𝐞 𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_𝔤𝔭_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯<𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>> = true;

// 𝒙𝟖𝟕, 𝒎𝒎𝒙, and 𝒎𝒂𝒔𝒌 registers. There are always 8 of these and size is fixed. The only complication is 𝔨0 register which is only
// allowed as write mask (where it means “unmasked operation”), but not as read mask. Therefore class 𝒎𝒂𝒔𝒌_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 is not final and
// 𝔨1…𝔨7 belong to 𝒏𝒐𝒛𝒆𝒓𝒐_𝒎𝒂𝒔𝒌_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 descendant.
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮_𝓿𝓪𝓻_𝓹𝓻𝓮𝓯𝓲𝔁, 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮, 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮, 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓿𝓪𝓻, 𝓯𝓲𝓷𝓪𝓵) \
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼> \
  class 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 𝓯𝓲𝓷𝓪𝓵 { \
   public: \
    template <typename 𝒯, typename = std::enable_if_t<std::numeric_limits<𝒯>::is_integer>> \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(𝒯 index) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) \
        : 𝗂𝗇𝖽𝖾𝗑{𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(index)} {} \
    𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓() = delete; \
\
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓𝖘(𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮) \
\
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr operator 𝐢𝐧𝐭₈() const noexcept { return 𝗂𝗇𝖽𝖾𝗑; } \
\
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 friend constexpr auto to_string(const 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓& reg) noexcept { \
      return 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓿𝓪𝓻##_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔫𝔞𝔪𝔢𝔰[reg.𝗂𝗇𝖽𝖾𝗑]; \
    } \
\
   private: \
    template <typename 𝒯, typename = std::enable_if_t<std::numeric_limits<𝒯>::is_integer>> \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(𝒯 index) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) -> 𝐢𝐧𝐭₈ { \
      𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖎𝖓_𝖗𝖆𝖓𝖌𝖊(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, index, 0, 8); \
      return 𝐢𝐧𝐭₈(index); \
    } \
\
    𝐢𝐧𝐭₈ 𝗂𝗇𝖽𝖾𝗑; \
  }; \
\
  𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊𝖘(𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮_𝓿𝓪𝓻_𝓹𝓻𝓮𝓯𝓲𝔁, 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮, 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮); \
\
  template <typename 𝓣> \
  inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_##𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓿𝓪𝓻##_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯 = false; \
\
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼> \
  inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_##𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓿𝓪𝓻##_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯<𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>> = true

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊𝖘(𝓹, 𝓽, 𝓷) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊, (, 𝓽, 𝓷, 0), (𝓹, 𝓽, 𝓷, 1), (𝓹, 𝓽, 𝓷, 2), (𝓹, 𝓽, 𝓷, 3)) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊, (𝓹, 𝓽, 𝓷, 4), (𝓹, 𝓽, 𝓷, 5), (𝓹, 𝓽, 𝓷, 6), (𝓹, 𝓽, 𝓷, 7))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊(𝓹𝓻𝓮𝓯𝓲𝔁, 𝓽𝔂𝓹𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭) \
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼> \
  inline constexpr 𝓹𝓻𝓮𝓯𝓲𝔁##𝓽𝔂𝓹𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼> 𝓷𝓪𝓶𝓮##𝓲𝓭{𝓲𝓭};

#if defined(__cpp_impl_three_way_comparison) && !defined(_MSC_VER)
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓𝖘(𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 auto operator<=>(const 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓&) const = default; \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 auto operator==(const 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓&) const->𝐛𝐨𝐨𝐥 = default;
#else
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓𝖘(𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto operator==(const 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓& r) const noexcept->𝐛𝐨𝐨𝐥 { return 𝗂𝗇𝖽𝖾𝗑 == r.𝗂𝗇𝖽𝖾𝗑; } \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto operator!=(const 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓& r) const noexcept->𝐛𝐨𝐨𝐥 { return 𝗂𝗇𝖽𝖾𝗑 != r.𝗂𝗇𝖽𝖾𝗑; } \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto operator<(const 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓& r) const noexcept->𝐛𝐨𝐨𝐥 { return 𝗂𝗇𝖽𝖾𝗑 < r.𝗂𝗇𝖽𝖾𝗑; } \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto operator<=(const 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓& r) const noexcept->𝐛𝐨𝐨𝐥 { return 𝗂𝗇𝖽𝖾𝗑 <= r.𝗂𝗇𝖽𝖾𝗑; } \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto operator>(const 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓& r) const noexcept->𝐛𝐨𝐨𝐥 { return 𝗂𝗇𝖽𝖾𝗑 > r.𝗂𝗇𝖽𝖾𝗑; } \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto operator>=(const 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓽𝔂𝓹𝓮##_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓& r) const noexcept->𝐛𝐨𝐨𝐥 { return 𝗂𝗇𝖽𝖾𝗑 >= r.𝗂𝗇𝖽𝖾𝗑; }
#endif

inline constexpr 𝒔𝒉𝒐𝒓𝒕_𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆 𝔵87_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔫𝔞𝔪𝔢𝔰{
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖑𝖎𝖘𝖙(𝖞𝖆𝖈𝖊_𝖚𝟴, st(0), st(1), st(2), st(3), st(4), st(5), st(6), st(7))};
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(, 𝒙𝟖𝟕, 𝔰𝔱, 𝔵87, final);

inline constexpr 𝒔𝒉𝒐𝒓𝒕_𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆 𝔪𝔪𝔵_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔫𝔞𝔪𝔢𝔰{𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖑𝖎𝖘𝖙(𝖞𝖆𝖈𝖊_𝖚𝟴, mm0, mm1, mm2, mm3, mm4, mm5, mm6, mm7)};
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(, 𝒎𝒎𝒙, 𝔪𝔪, 𝔪𝔪𝔵, final);

template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>
class 𝒎𝒂𝒔𝒌_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓;

template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>
class 𝒏𝒐𝒛𝒆𝒓𝒐_𝒎𝒂𝒔𝒌_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 final : public 𝒎𝒂𝒔𝒌_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼> {
 public:
  template <typename 𝒯, typename = std::enable_if_t<std::numeric_limits<𝒯>::is_integer>>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒏𝒐𝒛𝒆𝒓𝒐_𝒎𝒂𝒔𝒌_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(𝒯 index) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>)
      : 𝒎𝒂𝒔𝒌_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>(𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(index)) {}
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒏𝒐𝒛𝒆𝒓𝒐_𝒎𝒂𝒔𝒌_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(𝒎𝒂𝒔𝒌_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼> r) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>)
      : 𝒎𝒂𝒔𝒌_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>(𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥_𝑓𝑟𝑜𝑚_𝑚𝑎𝑠𝑘_𝑟𝑒𝑔𝑖𝑠𝑡𝑒𝑟(r)) {}
  𝒏𝒐𝒛𝒆𝒓𝒐_𝒎𝒂𝒔𝒌_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓() = delete;

 private:
  template <typename 𝒯, typename = std::enable_if_t<std::numeric_limits<𝒯>::is_integer and not std::is_same_v<𝒯, 𝐛𝐨𝐨𝐥>>>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(𝒯 index) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) -> 𝐢𝐧𝐭₈ {
    𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖎𝖓_𝖗𝖆𝖓𝖌𝖊(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, index, 1, 8);
    return 𝐢𝐧𝐭₈(index);
  }
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥_𝑓𝑟𝑜𝑚_𝑚𝑎𝑠𝑘_𝑟𝑒𝑔𝑖𝑠𝑡𝑒𝑟(𝒎𝒂𝒔𝒌_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼> index) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) -> 𝐢𝐧𝐭₈ {
    𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖆𝖇𝖔𝖛𝖊(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝐢𝐧𝐭₈(index), 0);
    return 𝐢𝐧𝐭₈(index);
  }
};

inline constexpr 𝒔𝒉𝒐𝒓𝒕_𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆 𝔪𝔞𝔰𝔨_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔫𝔞𝔪𝔢𝔰{𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖑𝖎𝖘𝖙(𝖞𝖆𝖈𝖊_𝖚𝟴, k0, k1, k2, k3, k4, k5, k6, k7)};
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝒏𝒐𝒛𝒆𝒓𝒐_, 𝒎𝒂𝒔𝒌, 𝔨, 𝔪𝔞𝔰𝔨, );

#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊𝖘
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓𝖘
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊

template <
    𝐬𝐢𝐳𝐞 𝓼𝓲𝔃𝓮,
    auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼,
    ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮 = 𝔵86_𝔪𝔬𝔡𝔢<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>,
    𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼 = 𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>>
class 𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓;

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝓼𝓲𝔃𝓮, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₁, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₂, 𝓿𝓪𝓻_𝓹𝓻𝓮𝓯𝓲𝔁, 𝓼𝓲𝓶𝓭_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼_𝓼𝓱𝓸𝓻𝓽, 𝓼𝓲𝓶𝓭_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼_𝔁64, 𝓼𝓲𝓶𝓭_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼_𝓯𝓾𝓵𝓵) \
\
  inline constexpr 𝒔𝒉𝒐𝒓𝒕_𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆 𝓼𝓲𝓶𝓭_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼_𝓼𝓱𝓸𝓻𝓽{𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖑𝖎𝖘𝖙(𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖓𝖆𝖒𝖊, 0, 1, 2, 3, 4, 5, 6, 7)}; \
  inline constexpr 𝒔𝒉𝒐𝒓𝒕_𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆 𝓼𝓲𝓶𝓭_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼_𝔁64{ \
      𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖑𝖎𝖘𝖙(𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖓𝖆𝖒𝖊, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15)}; \
  inline constexpr 𝒔𝒉𝒐𝒓𝒕_𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆 𝓼𝓲𝓶𝓭_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼_𝓯𝓾𝓵𝓵{ \
      𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖑𝖎𝖘𝖙(𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖓𝖆𝖒𝖊, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15), \
      𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖑𝖎𝖘𝖙(𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖓𝖆𝖒𝖊, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31)}; \
\
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮, 𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼> \
  class 𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼> final { \
    static_assert( \
        (𝔁86_𝓶𝓸𝓭𝓮 >= ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32) or \
        (𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼 == 𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬::𝔲𝔫𝔰𝔲𝔭𝔭𝔬𝔯𝔱𝔢𝔡)); \
\
   public: \
    template <typename 𝒯, typename = std::enable_if_t<std::numeric_limits<𝒯>::is_integer>> \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(𝒯 index) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) : 𝗂𝗇𝖽𝖾𝗑{𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(index)} {} \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓( \
        const 𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₁, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼>& r) noexcept \
        : 𝗂𝗇𝖽𝖾𝗑{𝐢𝐧𝐭₈(r)} {} \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓( \
        const 𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₂, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼>& r) noexcept \
        : 𝗂𝗇𝖽𝖾𝗑{𝐢𝐧𝐭₈(r)} {} \
\
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓𝖘(𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₁, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₂) \
\
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr operator 𝐢𝐧𝐭₈() const noexcept { return 𝗂𝗇𝖽𝖾𝗑; } \
\
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 friend constexpr auto to_string(𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 reg) noexcept { \
      if constexpr (𝔁86_𝓶𝓸𝓭𝓮 <= ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32) { \
        return (𝓼𝓲𝓶𝓭_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼_𝓼𝓱𝓸𝓻𝓽)[reg.𝗂𝗇𝖽𝖾𝗑]; \
      } else if constexpr (𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼 == 𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬::𝔲𝔫𝔰𝔲𝔭𝔭𝔬𝔯𝔱𝔢𝔡) { \
        return (𝓼𝓲𝓶𝓭_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼_𝔁64)[reg.𝗂𝗇𝖽𝖾𝗑]; \
      } else { \
        return (𝓼𝓲𝓶𝓭_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼_𝓯𝓾𝓵𝓵)[reg.𝗂𝗇𝖽𝖾𝗑]; \
      } \
    } \
\
   private: \
    template <typename 𝒯, typename = std::enable_if_t<std::numeric_limits<𝒯>::is_integer>> \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(𝒯 index) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) -> 𝐢𝐧𝐭₈ { \
      if constexpr (𝔁86_𝓶𝓸𝓭𝓮 <= ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32) { \
        𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖎𝖓_𝖗𝖆𝖓𝖌𝖊(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, index, 0, 8); \
      } else { \
        𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖎𝖓_𝖗𝖆𝖓𝖌𝖊(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, index, 0, 32); \
      } \
      return 𝐢𝐧𝐭₈(index); \
    } \
\
    𝐢𝐧𝐭₈ 𝗂𝗇𝖽𝖾𝗑; \
  }; \
\
  𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊𝖘(𝓼𝓲𝔃𝓮, 𝓿𝓪𝓻_𝓹𝓻𝓮𝓯𝓲𝔁); \
\
  template <typename 𝓣> \
  inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_##𝓿𝓪𝓻_𝓹𝓻𝓮𝓯𝓲𝔁##𝔪𝔪_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯 = false; \
\
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮, 𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼> \
  inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_##𝓿𝓪𝓻_𝓹𝓻𝓮𝓯𝓲𝔁##𝔪𝔪_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯<𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼>> = true

#ifdef __cpp_impl_three_way_comparison
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓𝖘(𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₁, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₂) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓, (==, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₁, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₂, 𝐛𝐨𝐨𝐥), (<=>, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₁, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₂, std::strong_ordering))
#else
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓𝖘(𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₁, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₂) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜( \
      𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓, \
      (==, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₁, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₂, 𝐛𝐨𝐨𝐥), \
      (!=, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₁, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₂, 𝐛𝐨𝐨𝐥), \
      (<, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₁, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₂, 𝐛𝐨𝐨𝐥), \
      (<=, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₁, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₂, 𝐛𝐨𝐨𝐥), \
      (>, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₁, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₂, 𝐛𝐨𝐨𝐥), \
      (>=, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₁, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₂, 𝐛𝐨𝐨𝐥))
#endif
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓(𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₁, 𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₂, 𝓻𝓮𝓽𝓾𝓻𝓷_𝓽𝔂𝓹𝓮) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto operator 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷(const 𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓& r) const noexcept->𝓻𝓮𝓽𝓾𝓻𝓷_𝓽𝔂𝓹𝓮 { \
    return 𝗂𝗇𝖽𝖾𝗑 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷 r.𝗂𝗇𝖽𝖾𝗑; \
  } \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto operator 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷( \
      const 𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₁, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼>& r) const noexcept->𝓻𝓮𝓽𝓾𝓻𝓷_𝓽𝔂𝓹𝓮 { \
    return 𝐢𝐧𝐭₈(𝗂𝗇𝖽𝖾𝗑) 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷 𝐢𝐧𝐭₈(r); \
  } \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto operator 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷( \
      const 𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓽𝓱𝓮𝓻_𝓼𝓲𝔃𝓮₂, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼>& r) const noexcept->𝓻𝓮𝓽𝓾𝓻𝓷_𝓽𝔂𝓹𝓮 { \
    return 𝐢𝐧𝐭₈(𝗂𝗇𝖽𝖾𝗑) 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷 𝐢𝐧𝐭₈(r); \
  }

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊𝖘(𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊, (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 0), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 1), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 2), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 3)); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊, (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 4), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 5), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 6), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 7)); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊, (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 8), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 9), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 10), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 11)); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊, (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 12), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 13), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 14), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 15)); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊, (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 16), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 17), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 18), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 19)); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊, (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 20), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 21), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 22), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 23)); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊, (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 24), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 25), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 26), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 27)); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊, (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 28), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 29), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 30), (𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 31))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊(𝓼𝓲𝔃𝓮, 𝓹𝓻𝓮𝓯𝓲𝔁, 𝓲𝓭) \
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮, 𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼> \
  inline constexpr 𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼> 𝓹𝓻𝓮𝓯𝓲𝔁##𝔪𝔪##𝓲𝓭 { \
    𝓲𝓭 \
  }

#define 𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖓𝖆𝖒𝖊(𝔁) u8"xmm" #𝔁
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(128, 256, 512, 𝔵, 𝔵𝔪𝔪_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔫𝔞𝔪𝔢𝔰_𝔭𝔢𝔫𝔱𝔦𝔲𝔪₃, 𝔵𝔪𝔪_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔫𝔞𝔪𝔢𝔰_ₓ₈₆_₆₄, 𝔵𝔪𝔪_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔫𝔞𝔪𝔢𝔰_ₐᵥₓ₅₁₂);
#undef 𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖓𝖆𝖒𝖊

#define 𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖓𝖆𝖒𝖊(𝔁) u8"ymm" #𝔁
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(256, 128, 512, 𝔶, 𝔶𝔪𝔪_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔫𝔞𝔪𝔢𝔰_𝔰𝔞𝔫𝔡𝔶_𝔟𝔯𝔦𝔡𝔤𝔢_₃₂𝔟𝔦𝔱, 𝔶𝔪𝔪_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔫𝔞𝔪𝔢𝔰_ₓ₈₆_₆₄, 𝔶𝔪𝔪_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔫𝔞𝔪𝔢𝔰_ₐᵥₓ₅₁₂);
#undef 𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖓𝖆𝖒𝖊

#define 𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖓𝖆𝖒𝖊(𝔁) u8"zmm" #𝔁
𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(512, 128, 256, 𝔷, 𝔷𝔪𝔪_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔫𝔞𝔪𝔢𝔰_𝔨𝔫𝔦𝔤𝔥𝔱𝔰_𝔩𝔞𝔫𝔡𝔦𝔫𝔤_₃₂𝔟𝔦𝔱, 𝔷𝔪𝔪_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔫𝔞𝔪𝔢𝔰_ₓ₈₆_₆₄, 𝔷𝔪𝔪_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯_𝔫𝔞𝔪𝔢𝔰_ₐᵥₓ₅₁₂);
#undef 𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖓𝖆𝖒𝖊

#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊𝖘
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓𝖘
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊

template <typename 𝓣>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_𝔰𝔦𝔪𝔡_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯 = false;

// Note: 𝔦𝔰_𝔰𝔦𝔪𝔡_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯 doesn't recognize MMX registers as SIMD registers because these are separate and rarly used ones.
template <𝐬𝐢𝐳𝐞 𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮, 𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_𝔰𝔦𝔪𝔡_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯<𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼>> = true;

template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮 = 𝔵86_𝔪𝔬𝔡𝔢<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>>
class 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓;

// Note: we only use low 5 bits to look up in the table.  It's enough for valid programs and we don't care about invalid ones.
inline constexpr 𝒔𝒉𝒐𝒓𝒕_𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆 𝔰𝔢𝔤𝔪𝔢𝔫𝔱_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯₈_𝔫𝔞𝔪𝔢𝔰{
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖑𝖎𝖘𝖙(𝖞𝖆𝖈𝖊_𝖚𝟴, , , , , fs, gs, es, , , , , , , , cs, , , , , , , , ss, , , , , , , , ds, )};

#ifdef __cpp_impl_three_way_comparison
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓𝖘 𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓, (==, 𝐛𝐨𝐨𝐥), (<=>, std::strong_ordering))
#else
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓𝖘 𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓, (==, 𝐛𝐨𝐨𝐥), (!=, 𝐛𝐨𝐨𝐥), (<, 𝐛𝐨𝐨𝐥), (<=, 𝐛𝐨𝐨𝐥), (>, 𝐛𝐨𝐨𝐥), (>=, 𝐛𝐨𝐨𝐥))
#endif
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓(𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷, 𝓻𝓮𝓽𝓾𝓻𝓷_𝓽𝔂𝓹𝓮) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto operator 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷(const 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓& r) const noexcept->𝓻𝓮𝓽𝓾𝓻𝓷_𝓽𝔂𝓹𝓮 { \
    return 𝗂𝗇𝖽𝖾𝗑 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷 r.𝗂𝗇𝖽𝖾𝗑; \
  } \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto operator 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷(const 𝐧𝐨_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫& no_register) const noexcept->𝓻𝓮𝓽𝓾𝓻𝓷_𝓽𝔂𝓹𝓮 { \
    return operator 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷(𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(no_register)); \
  } \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 friend constexpr auto operator 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷( \
      const 𝐧𝐨_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫& no_register, const 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓& r) noexcept->𝓻𝓮𝓽𝓾𝓻𝓷_𝓽𝔂𝓹𝓮 { \
    return 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(no_register) 𝓬𝓸𝓶𝓹𝓪𝓻𝓲𝓼𝓸𝓷 r; \
  }

template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮 = 𝔵86_𝔪𝔬𝔡𝔢<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>>
class 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 {
 public:
  template <typename 𝒯, typename = std::enable_if_t<std::numeric_limits<𝒯>::is_integer>>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(𝒯 index) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>)
      : 𝗂𝗇𝖽𝖾𝗑{𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(index)} {}
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> r) noexcept(
      𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>)
      : 𝗂𝗇𝖽𝖾𝗑{𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(𝐢𝐧𝐭₈(r))} {}
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(𝐧𝐨_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 /*no_register*/) noexcept : 𝗂𝗇𝖽𝖾𝗑{0} {}

  𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓𝖘

  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr operator 𝐢𝐧𝐭₈() const noexcept { return 𝗂𝗇𝖽𝖾𝗑; }

  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 friend constexpr auto to_string(𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 reg) noexcept {
    return 𝔰𝔢𝔤𝔪𝔢𝔫𝔱_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯₈_𝔫𝔞𝔪𝔢𝔰[reg.𝗂𝗇𝖽𝖾𝗑 bitand 0xb000'11111];
  }

 private:
  template <typename 𝒯, typename = std::enable_if_t<std::numeric_limits<𝒯>::is_integer>>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑚𝑎𝑘𝑒_𝑖𝑛𝑑𝑒𝑥(𝒯 index) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) -> 𝐢𝐧𝐭₈ {
    if constexpr (𝔁86_𝓶𝓸𝓭𝓮 <= ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32) {
      𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖔𝖓𝖊_𝖔𝖋(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, index, 0x26 /*𝔢𝔰*/, 0x2e /*𝔠𝔰*/, 0x36 /*𝔰𝔰*/, 0x3e /*𝔡𝔰*/, 0x64 /*𝔣𝔰*/, 0x65 /*𝔤𝔰*/);
    } else {
      𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖔𝖓𝖊_𝖔𝖋(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, index, 0x64 /*𝔣𝔰*/, 0x65 /*𝔤𝔰*/);
    }
    return 𝐢𝐧𝐭₈(index);
  }

  𝐢𝐧𝐭₈ 𝗂𝗇𝖽𝖾𝗑;
};

#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖒𝖕𝖆𝖗𝖎𝖘𝖔𝖓𝖘

template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
class 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 final : public 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> {
 public:
  template <typename 𝒯, typename = std::enable_if_t<std::numeric_limits<𝒯>::is_integer>>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓(𝒯 index) noexcept(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>)
      : 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>(index) {}
};

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊(𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭) \
  template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮> \
  inline constexpr 𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> 𝓷𝓪𝓶𝓮 { /* NOLINT(bugprone-macro-parentheses) */ \
    𝓲𝓭 \
  }
𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊,
    (𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝔫𝔬_𝔰𝔢𝔤𝔪𝔢𝔫𝔱, 𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯),
    (𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝔢𝔰, 0x26),
    (𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝔠𝔰, 0x2e),
    (𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝔰𝔰, 0x36),
    (𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝔡𝔰, 0x3e),
    (𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝔣𝔰, 0x64),
    (𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝔤𝔰, 0x65));
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖆𝖗𝖎𝖆𝖇𝖑𝖊

template <typename 𝓣>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_𝔰𝔢𝔤𝔪𝔢𝔫𝔱_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯 = false;

template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_𝔰𝔢𝔤𝔪𝔢𝔫𝔱_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯<𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>> = true;

// Full x86 address includes, segment, base, index, scale and displacement.
enum 𝐬𝐜𝐚𝐥𝐞 : 𝐢𝐧𝐭₈ { 𝔵1 = 0, 𝔵2 = 1, 𝔵4 = 2, 𝔵8 = 3 };

template <typename 𝓣>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_𝔰𝔠𝔞𝔩𝔢 = false;

template <>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_𝔰𝔠𝔞𝔩𝔢<𝐬𝐜𝐚𝐥𝐞> = true;

// 𝔢𝔦𝔭/𝔯𝔦𝔭 are for special x𝔦𝔭-based addressing in 𝔵86_64 mode.
inline constexpr class 𝐞𝐢𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 final {
} 𝔢𝔦𝔭;

inline constexpr class 𝐫𝐢𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 final {
} 𝔯𝔦𝔭;

#if defined(__GNUC__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wpadded"
#endif

// Note: 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮 doesn't change anything in the address itself, but there
// are instructions which only differ by address size of their operand.
template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, typename 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻, typename 𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻, typename 𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻>
class 𝒂𝒅𝒅𝒓𝒆𝒔𝒔 final {
 public:
  const 𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻 segment{𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯};
  const 𝓫𝓪𝓼𝓮_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻 base{𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯};
  const 𝓲𝓷𝓭𝓮𝔁_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻 index{𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯};
  const 𝐬𝐜𝐚𝐥𝐞 scale = 𝔵1;
  const 𝐢𝐧𝐭₃₂ disp = 0;
};
// Original 8086 address have no SIB and thus no scale and displacement is 16-bit, not 32-bit (like for all other addresses).
template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
class 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
    𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮,
    𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>,
    𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<16, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>,
    𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<16, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>>
    final {
 public:
  const 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> segment{𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯};
  const 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<16, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> base{𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯};
  const 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<16, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> index{𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯};
  const 𝐢𝐧𝐭₁₆ disp = 0;
};
// 𝔢𝔦𝔭/𝔯𝔦𝔭 based addresses. Note: only valid in 𝔵86_64 mode.
template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
class 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, 𝐞𝐢𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, void> final {
 public:
  // Surprisingly enough segment override works with 𝑒𝑖𝑝_𝑎𝑑𝑑𝑟𝑒𝑠𝑠.
  const 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> segment{𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯};
  𝖞𝖆𝖈𝖊_𝖓𝖔_𝖚𝖓𝖎𝖖𝖚𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘 const 𝐞𝐢𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 base = 𝔢𝔦𝔭;
  const 𝐢𝐧𝐭₃₂ disp = 0;
};
template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
class 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, 𝐫𝐢𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, void> final {
 public:
  // Surprisingly enough segment override works with 𝑟𝑖𝑝_𝑎𝑑𝑑𝑟𝑒𝑠𝑠.
  const 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> segment{𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯};
  𝖞𝖆𝖈𝖊_𝖓𝖔_𝖚𝖓𝖎𝖖𝖚𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘 const 𝐫𝐢𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 base = 𝔯𝔦𝔭;
  const 𝐢𝐧𝐭₃₂ disp = 0;
};
// Special addresses for string instructions and xlat.  They can be specified (and, in case of 𝔠𝔪𝔭𝔰 and 𝔪𝔬𝔳𝔰 𝗵𝗮𝘃𝗲 to be specified to
// distinguish operands of different sizes), but they don't affect the encoding of instruction except for possible segment
// override).
template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝐬𝐢𝐳𝐞 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
class 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
    𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮,
    𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>,
    𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>,
    void>
    final {
 public:
  const 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> segment{𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯};
  const 𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> base = 𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>{4};
};
template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝐬𝐢𝐳𝐞 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
class 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
    𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮,
    𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>,
    𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>,
    void>
    final {
 public:
  const 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> segment{𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯};
  const 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> base = 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>{6};
};
template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝐬𝐢𝐳𝐞 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
class 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, void, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, void> final {
 public:
  const 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮> base =
      𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>{7};
};
// Special addresses for “mov absolute”. Only includes address and segment.  But works with 64-bit address in 64-bit mode.
template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
class 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, 𝐢𝐧𝐭₁₆, void> final {
  const 𝐢𝐧𝐭₁₆ disp = 0;
};
template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
class 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, 𝐢𝐧𝐭₃₂, void> final {
  const 𝐢𝐧𝐭₃₂ disp = 0;
};
template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
class 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, 𝐢𝐧𝐭₆₄, void> final {
  const 𝐢𝐧𝐭₆₄ disp = 0;
};

#if defined(__GNUC__)
#pragma GCC diagnostic pop
#endif

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘(𝓰𝓹_𝔀𝓲𝓭𝓽𝓱, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓾𝓯𝓯𝓲𝔁, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮) \
  template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮> \
  using 𝒂𝒅𝒅𝒓𝒆𝒔𝒔##𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓾𝓯𝓯𝓲𝔁 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔< \
      𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, \
      𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, \
      𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮<𝓰𝓹_𝔀𝓲𝓭𝓽𝓱, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>,   /* NOLINT(bugprone-macro-parentheses) */ \
      𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮<𝓰𝓹_𝔀𝓲𝓭𝓽𝓱, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>>; /* NOLINT(bugprone-macro-parentheses) */ \
  template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮> \
  using 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔##𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓾𝓯𝓯𝓲𝔁 = \
      𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, 𝐢𝐧𝐭##𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓾𝓯𝓯𝓲𝔁, void>; \
  template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮> \
  using 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔##𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓾𝓯𝓯𝓲𝔁 = \
      𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, void, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓰𝓹_𝔀𝓲𝓭𝓽𝓱, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, void>; \
  template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮> \
  using 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔##𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓾𝓯𝓯𝓲𝔁 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔< \
      𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, \
      𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, \
      𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓰𝓹_𝔀𝓲𝓭𝓽𝓱, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, \
      void>; \
  template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮> \
  using 𝒙𝒍𝒂𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔##𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓾𝓯𝓯𝓲𝔁 = \
      𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, 𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓰𝓹_𝔀𝓲𝓭𝓽𝓱, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, void>; \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊( \
      𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖊𝖈𝖙𝖔𝖗_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘, \
      (128, 𝒙, 𝓰𝓹_𝔀𝓲𝓭𝓽𝓱, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓾𝓯𝓯𝓲𝔁), \
      (256, 𝒚, 𝓰𝓹_𝔀𝓲𝓭𝓽𝓱, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓾𝓯𝓯𝓲𝔁), \
      (512, 𝒛, 𝓰𝓹_𝔀𝓲𝓭𝓽𝓱, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓾𝓯𝓯𝓲𝔁))
// Note: you have to use 32-bit address with gather instructions - even in 16-bit mode.
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖊𝖈𝖙𝖔𝖗_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘(𝓿𝓮𝓬𝓽𝓸𝓻_𝔀𝓲𝓭𝓽𝓱, 𝓿𝓮𝓬𝓽𝓸𝓻_𝔀𝓲𝓭𝓽𝓱_𝓹𝓻𝓮𝓯𝓲𝔁, 𝓰𝓹_𝔀𝓲𝓭𝓽𝓱, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓾𝓯𝓯𝓲𝔁)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘(16, ₁₆, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓)
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖊𝖈𝖙𝖔𝖗_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖊𝖈𝖙𝖔𝖗_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘(𝓿𝓮𝓬𝓽𝓸𝓻_𝔀𝓲𝓭𝓽𝓱, 𝓿𝓮𝓬𝓽𝓸𝓻_𝔀𝓲𝓭𝓽𝓱_𝓹𝓻𝓮𝓯𝓲𝔁, 𝓰𝓹_𝔀𝓲𝓭𝓽𝓱, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓾𝓯𝓯𝓲𝔁) \
  template < \
      𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, \
      auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, \
      ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮, \
      𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼> \
  using 𝒈𝒂𝒕𝒉𝒆𝒓_##𝓿𝓮𝓬𝓽𝓸𝓻_𝔀𝓲𝓭𝓽𝓱_𝓹𝓻𝓮𝓯𝓲𝔁##𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔##𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓾𝓯𝓯𝓲𝔁 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔< \
      𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, \
      𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, \
      𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓰𝓹_𝔀𝓲𝓭𝓽𝓱, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, \
      𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓿𝓮𝓬𝓽𝓸𝓻_𝔀𝓲𝓭𝓽𝓱, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼>>;
𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘(64, ₆₄, 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓)
𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘(32, ₃₂, 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓)
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖛𝖊𝖈𝖙𝖔𝖗_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘(𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮, 𝓼𝓹𝓮𝓬𝓲𝓯𝓲𝓬_𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮_𝓹𝓻𝓮𝓯𝓲𝔁) \
  template <𝐬𝐢𝐳𝐞 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮> \
  using 𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 = /* NOLINT(bugprone-macro-parentheses) */ \
      std::enable_if_t< \
          𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 16 or 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 32 or 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 64, \
          std::conditional_t< \
              𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 16, \
              𝓼𝓹𝓮𝓬𝓲𝓯𝓲𝓬_𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮_𝓹𝓻𝓮𝓯𝓲𝔁##₁₆<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, \
              std::conditional_t< \
                  𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 32, \
                  𝓼𝓹𝓮𝓬𝓲𝓯𝓲𝓬_𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮_𝓹𝓻𝓮𝓯𝓲𝔁##₃₂<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, \
                  𝓼𝓹𝓮𝓬𝓲𝓯𝓲𝓬_𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮_𝓹𝓻𝓮𝓯𝓲𝔁##₆₄<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>>>>
𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘(𝒈𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘(𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘(𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘(𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘(𝒙𝒍𝒂𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, 𝒙𝒍𝒂𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔);
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘(𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮, 𝓼𝓹𝓮𝓬𝓲𝓯𝓲𝓬_𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮_𝓹𝓻𝓮𝓯𝓲𝔁) \
  template < \
      𝐬𝐢𝐳𝐞 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, \
      𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, \
      auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, \
      ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮, \
      𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼> \
  using 𝓰𝓮𝓷𝓮𝓻𝓲𝓬_𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 = /* NOLINT(bugprone-macro-parentheses) */ \
      std::enable_if_t< \
          𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 32 or 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 64, \
          std::conditional_t< \
              𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 32, \
              𝓼𝓹𝓮𝓬𝓲𝓯𝓲𝓬_𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮_𝓹𝓻𝓮𝓯𝓲𝔁##₃₂<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼>, \
              𝓼𝓹𝓮𝓬𝓲𝓯𝓲𝓬_𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮_𝓹𝓻𝓮𝓯𝓲𝔁##₆₄<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼>>>
𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘(𝒈𝒂𝒕𝒉𝒆𝒓_𝒙𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, 𝒈𝒂𝒕𝒉𝒆𝒓_𝒙𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘(𝒈𝒂𝒕𝒉𝒆𝒓_𝒚𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, 𝒈𝒂𝒕𝒉𝒆𝒓_𝒚𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘(𝒈𝒂𝒕𝒉𝒆𝒓_𝒛𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, 𝒈𝒂𝒕𝒉𝒆𝒓_𝒛𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔);
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖉𝖉𝖗𝖊𝖘𝖘_𝖆𝖑𝖎𝖆𝖘

template <
    𝐬𝐢𝐳𝐞 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮,
    𝐬𝐢𝐳𝐞 𝓲𝓷𝓭𝓮𝔁_𝓼𝓲𝔃𝓮,
    𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮,
    auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼,
    ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮,
    𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼>
using 𝒗𝒆𝒄𝒕𝒐𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 = std::enable_if_t<
    𝓲𝓷𝓭𝓮𝔁_𝓼𝓲𝔃𝓮 == 128 or 𝓲𝓷𝓭𝓮𝔁_𝓼𝓲𝔃𝓮 == 256 or 𝓲𝓷𝓭𝓮𝔁_𝓼𝓲𝔃𝓮 == 512,
    std::conditional_t<
        𝓲𝓷𝓭𝓮𝔁_𝓼𝓲𝔃𝓮 == 128,
        𝒈𝒂𝒕𝒉𝒆𝒓_𝒙𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼>,
        std::conditional_t<
            𝓲𝓷𝓭𝓮𝔁_𝓼𝓲𝔃𝓮 == 256,
            𝒈𝒂𝒕𝒉𝒆𝒓_𝒚𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼>,
            𝒈𝒂𝒕𝒉𝒆𝒓_𝒛𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼>>>>;

template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
using 𝒆𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, 𝐞𝐢𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, void>;
template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
using 𝒓𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>, 𝐫𝐢𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, void>;
template <𝐬𝐢𝐳𝐞 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
using ₓ𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 = std::enable_if_t<
    𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 32 or 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 64,
    std::conditional_t<
        𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 32,
        𝒆𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>,
        𝒓𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>>>;

template <typename 𝓣>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_𝔤𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰 = false;
template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝐬𝐢𝐳𝐞 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_𝔤𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
    𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮,
    𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>,
    𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>,
    𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>>> = true;
template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝐬𝐢𝐳𝐞 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_𝔤𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
    𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮,
    𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>,
    𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>,
    𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>>> = true;

template <typename 𝓣>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_𝔳𝔢𝔠𝔱𝔬𝔯_𝔞𝔡𝔡𝔯𝔢𝔰𝔰 = false;
template <
    𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮,
    𝐬𝐢𝐳𝐞 𝓰𝓹_𝔀𝓲𝓭𝓽𝓱,
    𝐬𝐢𝐳𝐞 𝓿𝓮𝓬𝓽𝓸𝓻_𝔀𝓲𝓭𝓽𝓱,
    auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼,
    ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮,
    𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_𝔳𝔢𝔠𝔱𝔬𝔯_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒂𝒅𝒅𝒓𝒆𝒔𝒔<
    𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮,
    𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>,
    𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒐𝒓_𝒏𝒐_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓰𝓹_𝔀𝓲𝓭𝓽𝓱, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>,
    𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓿𝓮𝓬𝓽𝓸𝓻_𝔀𝓲𝓭𝓽𝓱, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼>>> = true;

template <typename 𝓣>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_𝔢𝔦𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰 = false;
template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_𝔢𝔦𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒆𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>> = true;

template <typename 𝓣>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_𝔯𝔦𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰 = false;
template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_𝔯𝔦𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮>> = true;

template <typename 𝓣>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_ₓ𝔦𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰 = 𝔦𝔰_𝔢𝔦𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝓣> or 𝔦𝔰_𝔯𝔦𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝓣>;

template <typename 𝓣>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_𝔞𝔡𝔡𝔯𝔢𝔰𝔰 = 𝔦𝔰_𝔤𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝓣> or 𝔦𝔰_𝔳𝔢𝔠𝔱𝔬𝔯_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝓣> or 𝔦𝔰_ₓ𝔦𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝓣>;

// Instruction info database is usually invoked using 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐 template alias. It adds void at the end used by
// std::enable_if.
enum class 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 : 𝐢𝐧𝐭₁₆;
template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮, typename... 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼>
class 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐_𝒅𝒂𝒕𝒂𝒃𝒂𝒔𝒆;
template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮, typename... 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼>
using 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐 = 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐_𝒅𝒂𝒕𝒂𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮, 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼..., void>;

template <::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞 𝔁86_𝓶𝓸𝓭𝓮, 𝐬𝐢𝐳𝐞 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓼𝓲𝔃𝓮 = 8, 𝐬𝐢𝐳𝐞 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 = 0, 𝐬𝐢𝐳𝐞 𝓸𝓹𝓬𝓸𝓭𝓮_𝓵𝓮𝓷𝓰𝓽𝓱>
𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑎𝑑𝑗𝑢𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒(const std::array<𝐮𝐢𝐧𝐭₈, 𝓸𝓹𝓬𝓸𝓭𝓮_𝓵𝓮𝓷𝓰𝓽𝓱>& array) {
  // Note: 0 is used by default - for instructions which don't affect memory.
  static_assert(::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑖𝑠_𝑜𝑛𝑒_𝑜𝑓(𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝐬𝐢𝐳𝐞(0), 𝐬𝐢𝐳𝐞(16), 𝐬𝐢𝐳𝐞(32), 𝐬𝐢𝐳𝐞(64)));
  static_assert(::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝑖𝑠_𝑜𝑛𝑒_𝑜𝑓(𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓼𝓲𝔃𝓮, 𝐬𝐢𝐳𝐞(8), 𝐬𝐢𝐳𝐞(16), 𝐬𝐢𝐳𝐞(32), 𝐬𝐢𝐳𝐞(64)));
  auto address_prefix = []() {
    if constexpr (𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 0) {
      return std::array<𝐮𝐢𝐧𝐭₈, 0>{};
    } else if constexpr (𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 16) {
      if constexpr (address_size(𝔁86_𝓶𝓸𝓭𝓮) == 16) {
        return std::array<𝐮𝐢𝐧𝐭₈, 0>{};
      } else {
        static_assert(address_size(𝔁86_𝓶𝓸𝓭𝓮) == 32);
        return std::array{𝐮𝐢𝐧𝐭₈(0x67)};
      }
    } else if constexpr (𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 32) {
      if constexpr (address_size(𝔁86_𝓶𝓸𝓭𝓮) == 32) {
        return std::array<𝐮𝐢𝐧𝐭₈, 0>{};
      } else {
        return std::array{𝐮𝐢𝐧𝐭₈(0x67)};
      }
    } else /* 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮 == 64 */ {
      static_assert(address_size(𝔁86_𝓶𝓸𝓭𝓮) == 64);
      return std::array<𝐮𝐢𝐧𝐭₈, 0>{};
    }
  }();
  if constexpr (𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓼𝓲𝔃𝓮 == 8) {
    return 𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(address_prefix, array);
  } else {
    auto array_copy = array;
    ++array_copy[𝓸𝓹𝓬𝓸𝓭𝓮_𝓵𝓮𝓷𝓰𝓽𝓱 - 1];
    if constexpr (𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓼𝓲𝔃𝓮 == 16) {
      if constexpr (operand_size(𝔁86_𝓶𝓸𝓭𝓮) == 16) {
        return 𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(address_prefix, array_copy);
      } else {
        return 𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(address_prefix, std::array{𝐮𝐢𝐧𝐭₈(0x66)}, array_copy);
      }
    } else if constexpr (𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓼𝓲𝔃𝓮 == 32) {
      if constexpr (operand_size(𝔁86_𝓶𝓸𝓭𝓮) == 16) {
        return 𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(address_prefix, std::array{𝐮𝐢𝐧𝐭₈(0x66)}, array_copy);
      } else {
        return 𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(address_prefix, array_copy);
      }
    } else /* 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓼𝓲𝔃𝓮 == 64 */ {
      static_assert(𝔁86_𝓶𝓸𝓭𝓮 >= ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32);
      return 𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(address_prefix, std::array{𝐮𝐢𝐧𝐭₈(0x48)}, array_copy);
    }
  }
}

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(...) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊( \
      𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊, 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘(__VA_ARGS__))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭, 𝓸𝓹𝓬𝓸𝓭𝓮𝓼, 𝓭𝓮𝓯𝓲𝓷𝓮_𝓮𝔁𝓽𝓻𝓪_𝓶𝓮𝓶𝓫𝓮𝓻𝓼) \
  template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻> \
  class 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐_𝒅𝒂𝒕𝒂𝒃𝒂𝒔𝒆< \
      𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, \
      𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓷𝓪𝓶𝓮, \
      typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::template 𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮>, \
      typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::template 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮>, \
      void> \
      final { \
   public: \
    static constexpr 𝐬𝐢𝐳𝐞 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔴𝔦𝔡𝔱𝔥 = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮; \
    static constexpr auto 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢 = 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢𝔰[𝓲𝓭]; \
    static constexpr auto 𝔬𝔭𝔠𝔬𝔡𝔢𝔰 = \
        𝑎𝑑𝑗𝑢𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::𝔵86_𝔪𝔬𝔡𝔢, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮>(std::array{𝖞𝖆𝖈𝖊_𝖚𝖓𝖇𝖗𝖆𝖈𝖊 𝓸𝓹𝓬𝓸𝓭𝓮𝓼}); \
    𝓭𝓮𝓯𝓲𝓷𝓮_𝓮𝔁𝓽𝓻𝓪_𝓶𝓮𝓶𝓫𝓮𝓻𝓼(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭, 𝓸𝓹𝓬𝓸𝓭𝓮𝓼) \
  }

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖓𝖔𝖔𝖕𝖊𝖗𝖆𝖓𝖉_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝓷𝓪𝓶𝓮, 𝓲𝓭, 𝓸𝓹𝓬𝓸𝓭𝓮𝓼, 𝓭𝓮𝓯𝓲𝓷𝓮_𝓮𝔁𝓽𝓻𝓪_𝓶𝓮𝓶𝓫𝓮𝓻𝓼) \
  template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻> \
  class 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐_𝒅𝒂𝒕𝒂𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓷𝓪𝓶𝓮, void> final { \
   public: \
    static constexpr auto 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢 = 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢𝔰[𝓲𝓭]; \
    static constexpr auto 𝔬𝔭𝔠𝔬𝔡𝔢𝔰 = 𝑎𝑑𝑗𝑢𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::𝔵86_𝔪𝔬𝔡𝔢>(std::array{𝖞𝖆𝖈𝖊_𝖚𝖓𝖇𝖗𝖆𝖈𝖊 𝓸𝓹𝓬𝓸𝓭𝓮𝓼}); \
    𝓭𝓮𝓯𝓲𝓷𝓮_𝓮𝔁𝓽𝓻𝓪_𝓶𝓮𝓶𝓫𝓮𝓻𝓼(𝓷𝓪𝓶𝓮, 𝓲𝓭, 𝓸𝓹𝓬𝓸𝓭𝓮𝓼) \
  }

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(...) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊, 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖎𝖟𝖊𝖘(__VA_ARGS__))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭, 𝓸𝓹𝓬𝓸𝓭𝓮𝓼, 𝓭𝓮𝓯𝓲𝓷𝓮_𝓮𝔁𝓽𝓻𝓪_𝓶𝓮𝓶𝓫𝓮𝓻𝓼) \
  template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻> \
  class 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐_𝒅𝒂𝒕𝒂𝒃𝒂𝒔𝒆< \
      𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, \
      𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓷𝓪𝓶𝓮, \
      typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::template 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮>, \
      typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::template 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮>, \
      void> \
      final { \
   public: \
    static constexpr auto 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢 = 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢𝔰[𝓲𝓭]; \
    static constexpr auto 𝔬𝔭𝔠𝔬𝔡𝔢𝔰 = 𝑎𝑑𝑗𝑢𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::𝔵86_𝔪𝔬𝔡𝔢, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮>(std::array{𝖞𝖆𝖈𝖊_𝖚𝖓𝖇𝖗𝖆𝖈𝖊 𝓸𝓹𝓬𝓸𝓭𝓮𝓼}); \
    static constexpr 𝐬𝐢𝐳𝐞 𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡 = 0; \
    static constexpr 𝐬𝐢𝐳𝐞 𝔯𝔢𝔤_𝔬𝔭𝔢𝔯𝔞𝔫𝔡 = 1; \
    𝓭𝓮𝓯𝓲𝓷𝓮_𝓮𝔁𝓽𝓻𝓪_𝓶𝓮𝓶𝓫𝓮𝓻𝓼(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭, 𝓸𝓹𝓬𝓸𝓭𝓮𝓼) \
  }

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(...) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊( \
      𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊, 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘(𝒈𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, __VA_ARGS__)); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊( \
      𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊, \
      𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆_₃₂(ₓ𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, __VA_ARGS__), \
      𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆_₆₄(ₓ𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, __VA_ARGS__))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊( \
    𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭, 𝓸𝓹𝓬𝓸𝓭𝓮𝓼, 𝓭𝓮𝓯𝓲𝓷𝓮_𝓮𝔁𝓽𝓻𝓪_𝓶𝓮𝓶𝓫𝓮𝓻𝓼) \
  template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻> \
  class 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐_𝒅𝒂𝒕𝒂𝒃𝒂𝒔𝒆< \
      𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, \
      𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓷𝓪𝓶𝓮, \
      typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::template 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮>, /* NOLINT(bugprone-macro-parentheses) */ \
      typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::template 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮>, \
      void> \
      final { \
   public: \
    static constexpr auto 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢 = 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢𝔰[𝓲𝓭]; \
    static constexpr auto 𝔬𝔭𝔠𝔬𝔡𝔢𝔰 = \
        𝑎𝑑𝑗𝑢𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::𝔵86_𝔪𝔬𝔡𝔢, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮>(std::array{𝖞𝖆𝖈𝖊_𝖚𝖓𝖇𝖗𝖆𝖈𝖊 𝓸𝓹𝓬𝓸𝓭𝓮𝓼}); \
    static constexpr 𝐬𝐢𝐳𝐞 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔴𝔦𝔡𝔱𝔥 = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮; \
    static constexpr 𝐬𝐢𝐳𝐞 𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡 = 0; \
    static constexpr 𝐬𝐢𝐳𝐞 𝔯𝔢𝔤_𝔬𝔭𝔢𝔯𝔞𝔫𝔡 = 1; \
    𝓭𝓮𝓯𝓲𝓷𝓮_𝓮𝔁𝓽𝓻𝓪_𝓶𝓮𝓶𝓫𝓮𝓻𝓼(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭, 𝓸𝓹𝓬𝓸𝓭𝓮𝓼) \
  };

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(...) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊( \
      𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊, 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘(𝒈𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, __VA_ARGS__)); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊( \
      𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊, \
      𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆_₃₂(ₓ𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, __VA_ARGS__), \
      𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆_₆₄(ₓ𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, __VA_ARGS__))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊( \
    𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭, 𝓸𝓹𝓬𝓸𝓭𝓮𝓼, 𝓭𝓮𝓯𝓲𝓷𝓮_𝓮𝔁𝓽𝓻𝓪_𝓶𝓮𝓶𝓫𝓮𝓻𝓼) \
  template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻> \
  class 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐_𝒅𝒂𝒕𝒂𝒃𝒂𝒔𝒆< \
      𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, \
      𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓷𝓪𝓶𝓮, \
      typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::template 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮>, \
      typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::template 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮>, /* NOLINT(bugprone-macro-parentheses) */ \
      void> \
      final { \
   public: \
    static constexpr auto 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢 = 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢𝔰[𝓲𝓭]; \
    static constexpr auto 𝔬𝔭𝔠𝔬𝔡𝔢𝔰 = \
        𝑎𝑑𝑗𝑢𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::𝔵86_𝔪𝔬𝔡𝔢, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮>(std::array{𝖞𝖆𝖈𝖊_𝖚𝖓𝖇𝖗𝖆𝖈𝖊 𝓸𝓹𝓬𝓸𝓭𝓮𝓼}); \
    static constexpr 𝐬𝐢𝐳𝐞 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔴𝔦𝔡𝔱𝔥 = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮; \
    static constexpr 𝐬𝐢𝐳𝐞 𝔯𝔢𝔤_𝔬𝔭𝔢𝔯𝔞𝔫𝔡 = 0; \
    static constexpr 𝐬𝐢𝐳𝐞 𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡 = 1; \
    𝓭𝓮𝓯𝓲𝓷𝓮_𝓮𝔁𝓽𝓻𝓪_𝓶𝓮𝓶𝓫𝓮𝓻𝓼(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓽𝔂𝓹𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭, 𝓸𝓹𝓬𝓸𝓭𝓮𝓼) \
  }

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(...) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊( \
      𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊, 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘(__VA_ARGS__))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭, 𝓸𝓹𝓬𝓸𝓭𝓮𝓼, 𝓭𝓮𝓯𝓲𝓷𝓮_𝓮𝔁𝓽𝓻𝓪_𝓶𝓮𝓶𝓫𝓮𝓻𝓼) \
  template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻> \
  class 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐_𝒅𝒂𝒕𝒂𝒃𝒂𝒔𝒆< \
      𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, \
      𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓷𝓪𝓶𝓮, \
      typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::template 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮>, \
      typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::template 𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮>, \
      void> \
      final { \
   public: \
    static constexpr 𝐬𝐢𝐳𝐞 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔴𝔦𝔡𝔱𝔥 = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮; \
    static constexpr auto 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢 = 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢𝔰[𝓲𝓭]; \
    static constexpr auto 𝔬𝔭𝔠𝔬𝔡𝔢𝔰 = \
        𝑎𝑑𝑗𝑢𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::𝔵86_𝔪𝔬𝔡𝔢, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮>(std::array{𝖞𝖆𝖈𝖊_𝖚𝖓𝖇𝖗𝖆𝖈𝖊 𝓸𝓹𝓬𝓸𝓭𝓮𝓼}); \
    𝓭𝓮𝓯𝓲𝓷𝓮_𝓮𝔁𝓽𝓻𝓪_𝓶𝓮𝓶𝓫𝓮𝓻𝓼(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭, 𝓸𝓹𝓬𝓸𝓭𝓮𝓼) \
  }

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(...) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊, 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘(__VA_ARGS__))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭, 𝓸𝓹𝓬𝓸𝓭𝓮𝓼, 𝓭𝓮𝓯𝓲𝓷𝓮_𝓮𝔁𝓽𝓻𝓪_𝓶𝓮𝓶𝓫𝓮𝓻𝓼) \
  template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻> \
  class 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐_𝒅𝒂𝒕𝒂𝒃𝒂𝒔𝒆< \
      𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, \
      𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓷𝓪𝓶𝓮, \
      typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::template 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮>, \
      typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::template 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮>, \
      void> \
      final { \
   public: \
    static constexpr 𝐬𝐢𝐳𝐞 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔴𝔦𝔡𝔱𝔥 = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮; \
    static constexpr auto 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢 = 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢𝔰[𝓲𝓭]; \
    static constexpr auto 𝔬𝔭𝔠𝔬𝔡𝔢𝔰 = \
        𝑎𝑑𝑗𝑢𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::𝔵86_𝔪𝔬𝔡𝔢, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮>(std::array{𝖞𝖆𝖈𝖊_𝖚𝖓𝖇𝖗𝖆𝖈𝖊 𝓸𝓹𝓬𝓸𝓭𝓮𝓼}); \
    𝓭𝓮𝓯𝓲𝓷𝓮_𝓮𝔁𝓽𝓻𝓪_𝓶𝓮𝓶𝓫𝓮𝓻𝓼(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭, 𝓸𝓹𝓬𝓸𝓭𝓮𝓼) \
  }

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(...) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊( \
      𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊, 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘(__VA_ARGS__))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊( \
    𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭, 𝓸𝓹𝓬𝓸𝓭𝓮𝓼, 𝓭𝓮𝓯𝓲𝓷𝓮_𝓮𝔁𝓽𝓻𝓪_𝓶𝓮𝓶𝓫𝓮𝓻𝓼) \
  template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻> \
  class 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐_𝒅𝒂𝒕𝒂𝒃𝒂𝒔𝒆< \
      𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, \
      𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓷𝓪𝓶𝓮, \
      typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::template 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮>, \
      typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::template 𝒅𝒂𝒕𝒂_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<16>, \
      void> \
      final { \
   public: \
    static constexpr 𝐬𝐢𝐳𝐞 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔴𝔦𝔡𝔱𝔥 = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮; \
    static constexpr auto 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢 = 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢𝔰[𝓲𝓭]; \
    static constexpr auto 𝔬𝔭𝔠𝔬𝔡𝔢𝔰 = \
        𝑎𝑑𝑗𝑢𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::𝔵86_𝔪𝔬𝔡𝔢, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮>(std::array{𝖞𝖆𝖈𝖊_𝖚𝖓𝖇𝖗𝖆𝖈𝖊 𝓸𝓹𝓬𝓸𝓭𝓮𝓼}); \
    𝓭𝓮𝓯𝓲𝓷𝓮_𝓮𝔁𝓽𝓻𝓪_𝓶𝓮𝓶𝓫𝓮𝓻𝓼(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭, 𝓸𝓹𝓬𝓸𝓭𝓮𝓼) \
  }

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(...) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊( \
      𝖉𝖊𝖋𝖎𝖓𝖊_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊, 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘(__VA_ARGS__))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊( \
    𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭, 𝓸𝓹𝓬𝓸𝓭𝓮𝓼, 𝓭𝓮𝓯𝓲𝓷𝓮_𝓮𝔁𝓽𝓻𝓪_𝓶𝓮𝓶𝓫𝓮𝓻𝓼) \
  template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻> \
  class 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐_𝒅𝒂𝒕𝒂𝒃𝒂𝒔𝒆< \
      𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, \
      𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓷𝓪𝓶𝓮, \
      typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::template 𝒅𝒂𝒕𝒂_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<16>, \
      typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::template 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮>, \
      void> \
      final { \
   public: \
    static constexpr 𝐬𝐢𝐳𝐞 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔴𝔦𝔡𝔱𝔥 = 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮; \
    static constexpr auto 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢 = 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢𝔰[𝓲𝓭]; \
    static constexpr auto 𝔬𝔭𝔠𝔬𝔡𝔢𝔰 = \
        𝑎𝑑𝑗𝑢𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::𝔵86_𝔪𝔬𝔡𝔢, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮>(std::array{𝖞𝖆𝖈𝖊_𝖚𝖓𝖇𝖗𝖆𝖈𝖊 𝓸𝓹𝓬𝓸𝓭𝓮𝓼}); \
    𝓭𝓮𝓯𝓲𝓷𝓮_𝓮𝔁𝓽𝓻𝓪_𝓶𝓮𝓶𝓫𝓮𝓻𝓼(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭, 𝓸𝓹𝓬𝓸𝓭𝓮𝓼) \
  }

// This enum should be autogenerated.
enum class 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 : 𝐢𝐧𝐭₁₆ {
  𝔞𝔡𝔡,
  𝔠𝔪𝔭𝔰,
  𝔣𝔴𝔞𝔦𝔱,
  𝔦𝔫𝔰,
  𝔩𝔬𝔡𝔰,
  𝔪𝔬𝔳,
  𝔪𝔬𝔳𝔰,
  𝔯𝔢𝔭𝔫𝔷,
  𝔯𝔢𝔭𝔷,
  𝔯𝔢𝔭𝔫𝔷_𝔠𝔪𝔭𝔰,
  𝔯𝔢𝔭𝔷_𝔠𝔪𝔭𝔰,
  𝔯𝔢𝔭_𝔦𝔫𝔰,
  𝔯𝔢𝔭𝔫𝔷_𝔦𝔫𝔰,
  𝔯𝔢𝔭_𝔩𝔬𝔡𝔰,
  𝔯𝔢𝔭𝔫𝔷_𝔩𝔬𝔡𝔰,
  𝔯𝔢𝔭_𝔪𝔬𝔳𝔰,
  𝔯𝔢𝔭𝔫𝔷_𝔪𝔬𝔳𝔰,
  𝔯𝔢𝔭_𝔬𝔲𝔱𝔰,
  𝔯𝔢𝔭𝔫𝔷_𝔬𝔲𝔱𝔰,
  𝔯𝔢𝔭𝔫𝔷_𝔰𝔠𝔞𝔰,
  𝔯𝔢𝔭𝔷_𝔰𝔠𝔞𝔰,
  𝔯𝔢𝔭_𝔰𝔱𝔬𝔰,
  𝔯𝔢𝔭𝔫𝔷_𝔰𝔱𝔬𝔰,
  𝔬𝔲𝔱𝔰,
  𝔰𝔠𝔞𝔰,
  𝔰𝔱𝔬𝔰,
  𝔰𝔲𝔟
};

// This table should be autogenerated.
inline constexpr 𝒔𝒕𝒓𝒊𝒏𝒈_𝒕𝒂𝒃𝒍𝒆 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔫𝔞𝔪𝔢𝔰{
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖑𝖎𝖘𝖙(𝖞𝖆𝖈𝖊_𝖚𝟴, add, cmps, fwait, ins, lods, mov, movs, outs, repnz, repz, scas, stos, sub)};

// These instances should be autogenerated, too.
𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔞𝔡𝔡, 0, (𝐮𝐢𝐧𝐭₈(0x00)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔞𝔡𝔡, 0, (𝐮𝐢𝐧𝐭₈(0x00)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔞𝔡𝔡, 0, (𝐮𝐢𝐧𝐭₈(0x02)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔠𝔪𝔭𝔰, 1, (𝐮𝐢𝐧𝐭₈(0xa6)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖓𝖔𝖔𝖕𝖊𝖗𝖆𝖓𝖉_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔣𝔴𝔞𝔦𝔱, 2, (𝐮𝐢𝐧𝐭₈(0xf2)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔦𝔫𝔰, 3, (𝐮𝐢𝐧𝐭₈(0x6c)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔩𝔬𝔡𝔰, 4, (𝐮𝐢𝐧𝐭₈(0xac)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔪𝔬𝔳, 5, (𝐮𝐢𝐧𝐭₈(0x88)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔪𝔬𝔳, 5, (𝐮𝐢𝐧𝐭₈(0x88)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔪𝔬𝔳, 5, (𝐮𝐢𝐧𝐭₈(0x8a)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔪𝔬𝔳𝔰, 6, (𝐮𝐢𝐧𝐭₈(0xa4)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔬𝔲𝔱𝔰, 7, (𝐮𝐢𝐧𝐭₈(0x6e)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔯𝔢𝔭_𝔦𝔫𝔰, 3, (𝐮𝐢𝐧𝐭₈(0xf3), 𝐮𝐢𝐧𝐭₈(0x6c)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔯𝔢𝔭_𝔩𝔬𝔡𝔰, 4, (𝐮𝐢𝐧𝐭₈(0xf3), 𝐮𝐢𝐧𝐭₈(0xac)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔯𝔢𝔭_𝔪𝔬𝔳𝔰, 6, (𝐮𝐢𝐧𝐭₈(0xf3), 𝐮𝐢𝐧𝐭₈(0xa4)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔯𝔢𝔭_𝔰𝔱𝔬𝔰, 11, (𝐮𝐢𝐧𝐭₈(0xf3), 𝐮𝐢𝐧𝐭₈(0xaa)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔯𝔢𝔭_𝔬𝔲𝔱𝔰, 7, (𝐮𝐢𝐧𝐭₈(0xf3), 𝐮𝐢𝐧𝐭₈(0x6e)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖓𝖔𝖔𝖕𝖊𝖗𝖆𝖓𝖉_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔯𝔢𝔭𝔫𝔷, 8, (𝐮𝐢𝐧𝐭₈(0xf2)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔯𝔢𝔭𝔫𝔷_𝔠𝔪𝔭𝔰, 1, (𝐮𝐢𝐧𝐭₈(0xf2), 𝐮𝐢𝐧𝐭₈(0xa6)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔯𝔢𝔭𝔫𝔷_𝔦𝔫𝔰, 3, (𝐮𝐢𝐧𝐭₈(0xf2), 𝐮𝐢𝐧𝐭₈(0x6c)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔯𝔢𝔭𝔫𝔷_𝔩𝔬𝔡𝔰, 4, (𝐮𝐢𝐧𝐭₈(0xf2), 𝐮𝐢𝐧𝐭₈(0xac)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔯𝔢𝔭𝔫𝔷_𝔪𝔬𝔳𝔰, 6, (𝐮𝐢𝐧𝐭₈(0xf2), 𝐮𝐢𝐧𝐭₈(0xa4)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔯𝔢𝔭𝔫𝔷_𝔬𝔲𝔱𝔰, 7, (𝐮𝐢𝐧𝐭₈(0xf2), 𝐮𝐢𝐧𝐭₈(0x6e)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔯𝔢𝔭𝔫𝔷_𝔰𝔠𝔞𝔰, 10, (𝐮𝐢𝐧𝐭₈(0xf2), 𝐮𝐢𝐧𝐭₈(0xae)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔯𝔢𝔭𝔫𝔷_𝔰𝔱𝔬𝔰, 11, (𝐮𝐢𝐧𝐭₈(0xf2), 𝐮𝐢𝐧𝐭₈(0xaa)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖓𝖔𝖔𝖕𝖊𝖗𝖆𝖓𝖉_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔯𝔢𝔭𝔷, 9, (𝐮𝐢𝐧𝐭₈(0xf3)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔯𝔢𝔭𝔷_𝔠𝔪𝔭𝔰, 2, (𝐮𝐢𝐧𝐭₈(0xf3), 𝐮𝐢𝐧𝐭₈(0xa6)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔯𝔢𝔭𝔷_𝔰𝔠𝔞𝔰, 10, (𝐮𝐢𝐧𝐭₈(0xf3), 𝐮𝐢𝐧𝐭₈(0xae)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔰𝔠𝔞𝔰, 10, (𝐮𝐢𝐧𝐭₈(0xae)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔰𝔱𝔬𝔰, 11, (𝐮𝐢𝐧𝐭₈(0xaa)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔰𝔲𝔟, 12, (𝐮𝐢𝐧𝐭₈(0x28)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔰𝔲𝔟, 12, (𝐮𝐢𝐧𝐭₈(0x28)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝔰𝔲𝔟, 12, (𝐮𝐢𝐧𝐭₈(0x2a)), 𝖞𝖆𝖈𝖊_𝖉𝖎𝖘𝖈𝖆𝖗𝖉);

#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖓𝖔𝖔𝖕𝖊𝖗𝖆𝖓𝖉_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖙𝖊𝖒𝖕𝖑𝖆𝖙𝖊
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖍𝖊𝖈𝖐(𝓬𝓱𝓮𝓬𝓴_𝓷𝓪𝓶𝓮, 𝓯𝓲𝓮𝓵𝓭_𝓷𝓪𝓶𝓮) \
  template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸, typename = void> \
  inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_##𝓬𝓱𝓮𝓬𝓴_𝓷𝓪𝓶𝓮 = false; \
  template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮, typename... 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼> \
  inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_##𝓬𝓱𝓮𝓬𝓴_𝓷𝓪𝓶𝓮< \
      𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐_𝒅𝒂𝒕𝒂𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮, 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼...>, \
      std::enable_if_t<𝐛𝐨𝐨𝐥( \
          sizeof 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐_𝒅𝒂𝒕𝒂𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮, 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼...>::𝓯𝓲𝓮𝓵𝓭_𝓷𝓪𝓶𝓮)>> = true;
𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖍𝖊𝖈𝖐,
    (𝔢𝔵𝔦𝔰𝔱𝔰, 𝔬𝔭𝔠𝔬𝔡𝔢𝔰),
    (𝔥𝔞𝔰_𝔠𝔬𝔪𝔭𝔯𝔢𝔰𝔰𝔢𝔡_𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱, 𝔡𝔦𝔰𝔭8_𝔰𝔠𝔞𝔩𝔢),
    (𝔥𝔞𝔰_𝔦𝔪𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡, 𝔦𝔪𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡),
    (𝔥𝔞𝔰_𝔦𝔪𝔪2_𝔬𝔭𝔢𝔯𝔞𝔫𝔡, 𝔦𝔪𝔪2_𝔬𝔭𝔢𝔯𝔞𝔫𝔡),
    (𝔥𝔞𝔰_𝔦𝔰4_𝔬𝔭𝔢𝔯𝔞𝔫𝔡, 𝔦𝔰4_𝔬𝔭𝔢𝔯𝔞𝔫𝔡),
    (𝔥𝔞𝔰_𝔪𝔞𝔰𝔨_𝔬𝔭𝔢𝔯𝔞𝔫𝔡, 𝔪𝔞𝔰𝔨_𝔬𝔭𝔢𝔯𝔞𝔫𝔡),
    (𝔥𝔞𝔰_𝔪𝔢𝔪𝔬𝔯𝔶_𝔯𝔢𝔣𝔢𝔯𝔢𝔫𝔠𝔢, 𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔴𝔦𝔡𝔱𝔥),
    (𝔥𝔞𝔰_𝔯𝔢𝔤_𝔬𝔭𝔢𝔯𝔞𝔫𝔡, 𝔯𝔢𝔤_𝔬𝔭𝔢𝔯𝔞𝔫𝔡),
    (𝔥𝔞𝔰_𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡, 𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡),
    (𝔥𝔞𝔰_𝔳𝔢𝔵_𝔬𝔭𝔢𝔯𝔞𝔫𝔡, 𝔳𝔢𝔵_𝔬𝔭𝔢𝔯𝔞𝔫𝔡));
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖍𝖊𝖈𝖐

template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸, typename = void>
inline constexpr 𝐬𝐢𝐳𝐞 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔡𝔦𝔰𝔭8_𝔰𝔠𝔞𝔩𝔢 = 1;
template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮, typename... 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔡𝔦𝔰𝔭8_𝔰𝔠𝔞𝔩𝔢<
    𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐_𝒅𝒂𝒕𝒂𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮, 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼...>,
    std::enable_if_t<𝐛𝐨𝐨𝐥(sizeof 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐_𝒅𝒂𝒕𝒂𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮, 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼...>::𝔡𝔦𝔰𝔭8_𝔰𝔠𝔞𝔩𝔢)>> =
    𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐_𝒅𝒂𝒕𝒂𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮, 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼...>::𝔡𝔦𝔰𝔭8_𝔰𝔠𝔞𝔩𝔢;

template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓬𝓸𝓭𝓮_𝓵𝓮𝓷𝓰𝓽𝓱>
𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑐𝑜𝑢𝑛𝑡_𝑙𝑒𝑔𝑎𝑐𝑦_𝑝𝑟𝑒𝑓𝑖𝑥𝑒𝑠(const std::array<𝐮𝐢𝐧𝐭₈, 𝓸𝓹𝓬𝓸𝓭𝓮_𝓵𝓮𝓷𝓰𝓽𝓱>& full_opcode) noexcept -> 𝐬𝐢𝐳𝐞 {
  auto is_legacy_prefix = [](𝐮𝐢𝐧𝐭₈ byte) {
    return (byte == 0x26 /*𝔰𝔢𝔤𝔢𝔰*/) or (byte == 0x2e /*𝔰𝔢𝔤𝔠𝔰*/) or (byte == 0x36 /*𝔰𝔢𝔤𝔰𝔰*/) or (byte == 0x3e /*𝔰𝔢𝔤𝔡𝔰*/) or
           (byte == 0x64 /*𝔰𝔢𝔤𝔣𝔰*/) or (byte == 0x65 /*𝔰𝔢𝔤𝔤𝔰*/) or (byte == 0x66 /*operand-size prefix*/) or
           (byte == 0x67 /*address-size prefix*/) or (byte == 0xf0 /*𝔩𝔬𝔠𝔨*/) or (byte == 0xf2 /*𝔯𝔢𝔭𝔫𝔷*/) or (byte == 0xf3 /*𝔯𝔢𝔭𝔷*/);
  };
#ifdef __cpp_lib_constexpr_algorithms
  auto end = std::find_if_not(std::begin(full_opcode), std::end(full_opcode), is_legacy_prefix);
#else
  auto end = std::begin(full_opcode);
  while (end < std::end(full_opcode) and is_legacy_prefix(*end)) {
    ++end;
  }
#endif
  return 𝐬𝐢𝐳𝐞(end - std::begin(full_opcode));
}

template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>
inline constexpr auto 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰 =
    𝑡𝑜_𝑎𝑟𝑟𝑎𝑦<𝑐𝑜𝑢𝑛𝑡_𝑙𝑒𝑔𝑎𝑐𝑦_𝑝𝑟𝑒𝑓𝑖𝑥𝑒𝑠(𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰)>(std::begin(𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰));

// Note: 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔯𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵 == 0 if there are no REX prefix.
template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>
inline constexpr 𝐮𝐢𝐧𝐭₈ 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔯𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵 =
    (𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰.size() <=  // Handle 𝔦𝔫𝔠/𝔡𝔢𝔠 in legacy and compatibility modes.
     𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() + 1)
        ? 0
        : ((0x40 <= 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰.size()]) and
           (𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰.size()] <= 0x4f));

template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔳𝔢𝔵_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫 =
    𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰.size() <= 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
                                            1  // Handle 𝔩𝔢𝔰 in legacy and compatibility modes.
        ? 0
        : 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size()] == 0xc4;

template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>
inline constexpr typename std::enable_if_t<𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔳𝔢𝔵_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>, std::array<𝐮𝐢𝐧𝐭₈, 3>>
    𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵 = {
        𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size()],
        𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() + 1],
        𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() + 2]};

template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔢𝔳𝔢𝔵_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫 =
    𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰.size() <= 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
                                            1  // Handle 𝔟𝔬𝔲𝔫𝔡 in legacy and compatibility modes.
        ? 0
        : 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size()] == 0x62;

template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>
inline constexpr typename std::enable_if_t<𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔢𝔳𝔢𝔵_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>, std::array<𝐮𝐢𝐧𝐭₈, 3>>
    𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔢𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵 = {
        𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size()],
        𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() + 1],
        𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() + 2],
        𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() + 3]};

template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔵𝔬𝔭_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫 =
    𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰.size() <= 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() + 2  // Handle 𝔭𝔬𝔭 𝓂ℯ𝓂ℴ𝓇𝓎.
        ? 0
        : 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size()] == 0x8f;

template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>
inline constexpr typename std::enable_if_t<𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔳𝔢𝔵_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>, std::array<𝐮𝐢𝐧𝐭₈, 3>>
    𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵 = {
        𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size()],
        𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() + 1],
        𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() + 2]};

template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>
inline constexpr typename std::enable_if_t<𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔳𝔢𝔵_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>, std::array<𝐮𝐢𝐧𝐭₈, 3>>
    𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵 = {
        𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size()],
        𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() + 1],
        𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() + 2]};

template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>
inline constexpr auto 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰 = 𝑡𝑜_𝑎𝑟𝑟𝑎𝑦<
    𝑐𝑜𝑢𝑛𝑡_𝑙𝑒𝑔𝑎𝑐𝑦_𝑝𝑟𝑒𝑓𝑖𝑥𝑒𝑠(𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰) + 𝐛𝐨𝐨𝐥(𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔯𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) +
    3 * 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔳𝔢𝔵_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸> + 4 * 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔢𝔳𝔢𝔵_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸> +
    3 * 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔵𝔬𝔭_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>>(std::begin(𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰));

template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>
inline constexpr auto 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯 = 𝑡𝑜_𝑎𝑟𝑟𝑎𝑦<
    𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰.size() ==
            𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size()  // Legacy opcodes as separate instruction.
        ? 0
        : 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔳𝔢𝔵_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸> or 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔢𝔳𝔢𝔵_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸> or
                  𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔵𝔬𝔭_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>
              ? 1
          : 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size()] != 0x0f ? 1
          : (𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() + 1] != 0x38 and
             𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰[𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() + 1] != 0x3a)
              ? 2
              : 3>(std::begin(𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰) + 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size());

// Note that check mistakenly marks 2nd byte of 𝔵87 instruction not as 𝔪𝔬𝔡𝔯𝔪 but as 𝔦𝔪𝔪 — but that interpretation doesn't really
// contradict documentation.
template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔬𝔭𝔠𝔬𝔡𝔢_𝔦𝔫_𝔪𝔬𝔡𝔯𝔪 =
    𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰.size() == 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
                                            𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size()
        ? false
        : 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>;

template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>
inline constexpr 𝐛𝐨𝐨𝐥 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔬𝔭𝔠𝔬𝔡𝔢_𝔦𝔫_𝔦𝔪𝔪 =
    𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰.size() == 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
                                            𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size()
        ? false
        : 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>;

// Note: the following code is deliberately written in a way which makes it impossible to take 𝔬𝔭𝔠𝔬𝔡𝔢_𝔢𝔵𝔱𝔢𝔫𝔰𝔦𝔬𝔫 from instruction
// which doesn't have or take more than one byte from instruction which does have it without a compile-time error.
template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>
inline constexpr typename std::enable_if_t<
    𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔬𝔭𝔠𝔬𝔡𝔢_𝔦𝔫_𝔪𝔬𝔡𝔯𝔪<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸> or 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔬𝔭𝔠𝔬𝔡𝔢_𝔦𝔫_𝔦𝔪𝔪<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
    std::array<𝐮𝐢𝐧𝐭₈, 1>>
    𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔢𝔵𝔱𝔢𝔫𝔰𝔦𝔬𝔫 = 𝑡𝑜_𝑎𝑟𝑟𝑎𝑦<
        𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰.size() -
        (𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() + 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size())>(
        std::begin(𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰) +
        (𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() + 𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size()));

// See 𝔢𝔪𝔦𝔱_𝔪𝔬𝔡𝔯𝔪_𝔰𝔦𝔟_𝔡𝔦𝔰𝔭 for explanation. This array shouldn't be in the template class or function, since most compilers wouldn't
// be able to merge these into one array.
alignas(sizeof(𝐮𝐢𝐧𝐭₆₄) * 4) inline constexpr std::array<𝐮𝐢𝐧𝐭₈, 32> 𝔯𝔪_𝔰𝔢𝔩𝔢𝔠𝔱𝔬𝔯_𝔱𝔬_𝔯𝔪₈₀₈₆ = {
    0, 0, 0, 0, 2, 3, 0, 1, 0, 0, 0, 0, 4, 5, 4, 5, 0, 0, 0, 0, 6, 6, 7, 7, 0, 0, 0, 0, 6, 6, 6, 6};

// Herlper array to convert segment-as-prefix number to segment-as-reg number.
alignas(sizeof(𝐮𝐢𝐧𝐭₆₄) * 4) inline constexpr std::array<𝐮𝐢𝐧𝐭₈, 32> 𝔰𝔢𝔤𝔪𝔢𝔫𝔱_𝔱𝔬_𝔯𝔢𝔤₈₀₃₈₆ = {
    7, 7, 7, 7, 4, 5, 0, 7, 7, 7, 7, 7, 7, 7, 1, 7, 7, 7, 7, 7, 7, 7, 2, 7, 7, 7, 7, 7, 7, 7, 3, 7};

}  // namespace 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿

// Note: 16-bit and 32-bit modes are very similar (although instructions are encoded differently but all sizes and modes are
// present). But 64-bit mode is different: it adds 64-bit registers and also many additional registers (𝔯8 to 𝔯15, 𝔵𝔪𝔪9 to 𝔵𝔪𝔪32 and
// so on), but it also makes many instructions unavailable and doesn't support 16-bit addresses.
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖇𝖆𝖘𝖎𝖈_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗(𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼) \
  template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼> \
  class 𝒃𝒂𝒔𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼> { \
   public: \
    static constexpr auto 𝔵86_𝔪𝔬𝔡𝔢 = 𝔁86_𝓶𝓸𝓭𝓮; \
    static constexpr auto 𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰 = 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼; \
    static_assert( \
        (𝔵86_𝔪𝔬𝔡𝔢 >= 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32) or \
        (𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰 == 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬::𝔲𝔫𝔰𝔲𝔭𝔭𝔬𝔯𝔱𝔢𝔡)); \
\
    using 𝐧𝐨𝐫𝐞𝐱 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐧𝐨𝐫𝐞𝐱; \
    static constexpr 𝐧𝐨𝐫𝐞𝐱 𝔫𝔬𝔯𝔢𝔵{}; \
\
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊( \
        𝖎𝖒𝖕𝖔𝖗𝖙_𝖌𝖕_𝖗𝖊𝖌, \
        (𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓), \
        (𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓), \
        (𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝒄𝒐𝒖𝒏𝒕𝒆𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓), \
        (𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝒅𝒂𝒕𝒂_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓), \
        (𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓), \
        (𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝒔𝒕𝒂𝒄𝒌_𝒑𝒐𝒊𝒏𝒕𝒆𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓), \
        (𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝒃𝒂𝒔𝒆_𝒑𝒐𝒊𝒏𝒕𝒆𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓), \
        (𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓), \
        (𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓)); \
\
    template <𝐬𝐢𝐳𝐞 𝓼𝓲𝔃𝓮 = address_size(𝔵86_𝔪𝔬𝔡𝔢)> \
    using 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 = \
        typename 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓_𝒊𝒎𝒑𝒐𝒓𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔵86_𝔪𝔬𝔡𝔢>::𝐭𝐲𝐩𝐞; \
    /* C++17 compatibility, use 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 with C++20. */ \
    using 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒃𝒂𝒔𝒆_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<>; \
\
    template <𝐬𝐢𝐳𝐞 𝓼𝓲𝔃𝓮 = address_size(𝔵86_𝔪𝔬𝔡𝔢)> \
    using 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 = \
        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔵86_𝔪𝔬𝔡𝔢>; \
    /* C++17 compatibility, use 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 with C++20. */ \
    using 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒊𝒏𝒅𝒆𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<>; \
\
    template <𝐬𝐢𝐳𝐞 𝓼𝓲𝔃𝓮 = 128> \
    using 𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 = \
        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔵86_𝔪𝔬𝔡𝔢, 𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰>; \
    /* C++17 compatibility, use 𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 with C++20. */ \
    using 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<>; \
\
    template <𝐬𝐢𝐳𝐞 𝓼𝓲𝔃𝓮 = 256> \
    using 𝒚𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 = \
        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔵86_𝔪𝔬𝔡𝔢, 𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰>; \
    /* C++17 compatibility, use 𝒚𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 with C++20. */ \
    using 𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝒚𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<>; \
\
    template <𝐬𝐢𝐳𝐞 𝓼𝓲𝔃𝓮 = 512> \
    using 𝒛𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 = \
        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝒙𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓼𝓲𝔃𝓮, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔵86_𝔪𝔬𝔡𝔢, 𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰>; \
    /* C++17 compatibility, use 𝒛𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓 with C++20. */ \
    using 𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝒛𝒎𝒎_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<>; \
\
    using 𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝒔𝒆𝒈𝒎𝒆𝒏𝒕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔵86_𝔪𝔬𝔡𝔢>; \
    using 𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝒙𝟖𝟕_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>>; \
    using 𝐦𝐦𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝒎𝒎𝒙_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>>; \
    using 𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝒎𝒂𝒔𝒌_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>>; \
    using 𝐧𝐨𝐳𝐞𝐫𝐨_𝐦𝐚𝐬𝐤_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝒏𝒐𝒛𝒆𝒓𝒐_𝒎𝒂𝒔𝒌_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>>; \
\
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖉𝖉𝖗, 𝒈𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, 𝒙𝒍𝒂𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔); \
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖉𝖉𝖗_𝖛, 𝒈𝒂𝒕𝒉𝒆𝒓_𝒙𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, 𝒈𝒂𝒕𝒉𝒆𝒓_𝒚𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔, 𝒈𝒂𝒕𝒉𝒆𝒓_𝒛𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔); \
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖉𝖉𝖗_𝖜, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔₃₂, 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₃₂, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₃₂, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₃₂, 𝒙𝒍𝒂𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₃₂); \
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖉𝖉𝖗_𝖛𝖜, 𝒈𝒂𝒕𝒉𝒆𝒓_𝒙𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₃₂, 𝒈𝒂𝒕𝒉𝒆𝒓_𝒚𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₃₂, 𝒈𝒂𝒕𝒉𝒆𝒓_𝒛𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₃₂); \
\
    template <𝐬𝐢𝐳𝐞 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝐬𝐢𝐳𝐞 𝓲𝓷𝓭𝓮𝔁_𝓼𝓲𝔃𝓮, 𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> \
    using 𝒗𝒆𝒄𝒕𝒐𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝒗𝒆𝒄𝒕𝒐𝒓_𝒂𝒅𝒅𝒓𝒆𝒔𝒔< \
        𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, \
        𝓲𝓷𝓭𝓮𝔁_𝓼𝓲𝔃𝓮, \
        𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, \
        ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, \
        𝔵86_𝔪𝔬𝔡𝔢, \
        𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰>; \
\
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖘𝖎𝖒𝖕𝖑𝖊_𝖙𝖞𝖕𝖊, 𝐧𝐨_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝐬𝐜𝐚𝐥𝐞); \
\
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌, 𝔞𝔩, 𝔠𝔩, 𝔡𝔩, 𝔟𝔩, 𝔞𝔥, 𝔠𝔥, 𝔡𝔥, 𝔟𝔥); \
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌, 𝔞𝔵, 𝔠𝔵, 𝔡𝔵, 𝔟𝔵, 𝔰𝔭, 𝔟𝔭, 𝔰𝔦, 𝔡𝔦); \
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌, 𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔰𝔭, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦, 𝔢𝔦𝔷); \
\
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖘𝖎𝖒𝖕𝖑𝖊_𝖛𝖆𝖗, 𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯, 𝔵1, 𝔵2, 𝔵4, 𝔵8); \
\
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌_𝖛, 𝔵𝔪𝔪0, 𝔵𝔪𝔪1, 𝔵𝔪𝔪2, 𝔵𝔪𝔪3, 𝔵𝔪𝔪4, 𝔵𝔪𝔪5, 𝔵𝔪𝔪6, 𝔵𝔪𝔪7); \
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌_𝖛, 𝔶𝔪𝔪0, 𝔶𝔪𝔪1, 𝔶𝔪𝔪2, 𝔶𝔪𝔪3, 𝔶𝔪𝔪4, 𝔶𝔪𝔪5, 𝔶𝔪𝔪6, 𝔶𝔪𝔪7); \
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌_𝖛, 𝔷𝔪𝔪0, 𝔷𝔪𝔪1, 𝔷𝔪𝔪2, 𝔷𝔪𝔪3, 𝔷𝔪𝔪4, 𝔷𝔪𝔪5, 𝔷𝔪𝔪6, 𝔷𝔪𝔪7); \
\
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖒𝖔𝖉𝖊_𝖆𝖌𝖓𝖔𝖘𝖙𝖎𝖈_𝖗𝖊𝖌, 𝔰𝔱0, 𝔰𝔱1, 𝔰𝔱2, 𝔰𝔱3, 𝔰𝔱4, 𝔰𝔱5, 𝔰𝔱6, 𝔰𝔱7); \
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖒𝖔𝖉𝖊_𝖆𝖌𝖓𝖔𝖘𝖙𝖎𝖈_𝖗𝖊𝖌, 𝔪𝔪0, 𝔪𝔪1, 𝔪𝔪2, 𝔪𝔪3, 𝔪𝔪4, 𝔪𝔪5, 𝔪𝔪6, 𝔪𝔪7); \
\
    static constexpr auto 𝔞𝔠𝔠𝔲𝔪𝔲𝔩𝔞𝔱𝔬𝔯 = 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫{0}; \
    static constexpr auto 𝔠𝔬𝔲𝔫𝔱𝔢𝔯_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯 = 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫{1}; \
    static constexpr auto 𝔡𝔞𝔱𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯 = 𝐝𝐚𝐭𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫{2}; \
    static constexpr auto 𝔟𝔞𝔰𝔢_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯 = 𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫{3}; \
    static constexpr auto 𝔰𝔱𝔞𝔠𝔨_𝔭𝔬𝔦𝔫𝔱𝔢𝔯 = 𝐬𝐭𝐚𝐜𝐤_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫{4}; \
    static constexpr auto 𝔟𝔞𝔰𝔢_𝔭𝔬𝔦𝔫𝔱𝔢𝔯 = 𝐛𝐚𝐬𝐞_𝐩𝐨𝐢𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫{5}; \
    static constexpr auto 𝔰𝔬𝔲𝔯𝔠𝔢_𝔦𝔫𝔡𝔢𝔵 = 𝐬𝐨𝐮𝐫𝐜𝐞_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫{6}; \
    static constexpr auto 𝔡𝔢𝔰𝔱𝔦𝔫𝔞𝔱𝔦𝔬𝔫_𝔦𝔫𝔡𝔢𝔵 = 𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫{7}; \
\
    𝖎𝖒𝖕𝖔𝖗𝖙_𝖘𝖎𝖟𝖊_𝖘𝖕𝖊𝖈𝖎𝖋𝖎𝖈_𝖊𝖓𝖙𝖎𝖙𝖎𝖊𝖘 \
\
    𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖛𝖝𝟱𝟭𝟮_𝖘𝖎𝖟𝖊_𝖘𝖕𝖊𝖈𝖎𝖋𝖎𝖈_𝖊𝖓𝖙𝖎𝖙𝖎𝖊𝖘 \
\
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌_𝖈𝖍𝖊𝖈𝖐𝖊𝖗, 𝔤𝔭, 𝔰𝔢𝔤𝔪𝔢𝔫𝔱, 𝔵87, 𝔪𝔪𝔵, 𝔵𝔪𝔪, 𝔶𝔪𝔪, 𝔷𝔪𝔪, 𝔪𝔞𝔰𝔨); \
    𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖘𝖎𝖒𝖕𝖑𝖊_𝖈𝖍𝖊𝖈𝖐𝖊𝖗, 𝔞𝔡𝔡𝔯𝔢𝔰𝔰, 𝔰𝔠𝔞𝔩𝔢, 𝔤𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰, 𝔳𝔢𝔠𝔱𝔬𝔯_𝔞𝔡𝔡𝔯𝔢𝔰𝔰); \
    /* That list of instructions must be autogenerated. \
     * Note: we are not making these functions templates because if we would do that automatic conversions would be disabled and \
     * it would be impossible to write something like mov(𝔞𝔵, {.base = 𝔞𝔵}); */ \
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(add, 𝔞𝔡𝔡) \
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(add, 𝔞𝔡𝔡) \
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(add, 𝔞𝔡𝔡) \
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(cmps, 𝔠𝔪𝔭𝔰) \
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖓𝖔𝖔𝖕𝖊𝖗𝖆𝖓𝖉_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(fwait, 𝔣𝔴𝔞𝔦𝔱) \
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(ins, 𝔦𝔫𝔰) \
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(lods, 𝔩𝔬𝔡𝔰) \
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(mov, 𝔪𝔬𝔳) \
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(mov, 𝔪𝔬𝔳) \
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(mov, 𝔪𝔬𝔳) \
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(movs, 𝔪𝔬𝔳𝔰) \
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(outs, 𝔬𝔲𝔱𝔰) \
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(scas, 𝔰𝔠𝔞𝔰) \
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(stos, 𝔰𝔱𝔬𝔰) \
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(sub, 𝔰𝔲𝔟) \
    𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(sub, 𝔰𝔲𝔟) \
    /* List of string instruction is Ok to keep not-autognerated: last string instruction was added with 80186 back in 1982, \
     * it happens rarely enough that automatic handling is not really needed. */ \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 auto rep() { return repz(); } \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 auto repe() { return repz(); } \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 auto repne() { return repnz(); } \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 auto repnz() noexcept { \
      class 𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫 final { \
       public: \
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(cmps, 𝔯𝔢𝔭𝔫𝔷_𝔠𝔪𝔭𝔰) \
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(ins, 𝔯𝔢𝔭𝔫𝔷_𝔦𝔫𝔰) \
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(lods, 𝔯𝔢𝔭𝔫𝔷_𝔩𝔬𝔡𝔰) \
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(movs, 𝔯𝔢𝔭𝔫𝔷_𝔪𝔬𝔳𝔰) \
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(outs, 𝔯𝔢𝔭𝔫𝔷_𝔬𝔲𝔱𝔰) \
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(scas, 𝔯𝔢𝔭𝔫𝔷_𝔰𝔠𝔞𝔰) \
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(stos, 𝔯𝔢𝔭𝔫𝔷_𝔰𝔱𝔬𝔰) \
        𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫(const 𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫&) = delete; \
        𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫(𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫&&) = delete; \
        auto operator=(const 𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫&) -> 𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫& = delete; \
        auto operator=(𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫&&) -> 𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫& = delete; \
        𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 𝖞𝖆𝖈𝖊_𝖈𝖔𝖓𝖘𝖙𝖊𝖝𝖕𝖗_𝖋𝖔𝖗_𝖉𝖊𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗 ~𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫() { \
          if (𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋) { \
            get_assembler().template 𝑒𝑚𝑖𝑡_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝔯𝔢𝔭𝔫𝔷>(); \
          } \
        } \
\
       private: \
        𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) get_assembler() { \
          auto* assembler = 𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋; \
          𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖓𝖔𝖙_𝖊𝖖𝖚𝖆𝖑(::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, assembler, nullptr); \
          𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋 = nullptr; \
          return *static_cast<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻*>(assembler); \
        } \
        𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫( \
            𝒃𝒂𝒔𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼>* assembler) noexcept \
            : 𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋(assembler) {} \
        𝒃𝒂𝒔𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝔁86_𝓶𝓸𝓭𝓮, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>>* 𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋; \
        friend 𝒃𝒂𝒔𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓; \
      }; \
      return 𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫(this); \
    } \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 auto repz() noexcept { \
      class 𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫 final { \
       public: \
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(cmps, 𝔯𝔢𝔭𝔷_𝔠𝔪𝔭𝔰) \
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(ins, 𝔯𝔢𝔭_𝔦𝔫𝔰) \
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(lods, 𝔯𝔢𝔭_𝔩𝔬𝔡𝔰) \
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(movs, 𝔯𝔢𝔭_𝔪𝔬𝔳𝔰) \
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(outs, 𝔯𝔢𝔭_𝔬𝔲𝔱𝔰) \
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(scas, 𝔯𝔢𝔭𝔷_𝔰𝔠𝔞𝔰) \
        𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(stos, 𝔯𝔢𝔭_𝔰𝔱𝔬𝔰) \
        𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫(const 𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫&) = delete; \
        𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫(𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫&&) = delete; \
        auto operator=(const 𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫&) -> 𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫& = delete; \
        auto operator=(𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫&&) -> 𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫& = delete; \
        𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 𝖞𝖆𝖈𝖊_𝖈𝖔𝖓𝖘𝖙𝖊𝖝𝖕𝖗_𝖋𝖔𝖗_𝖉𝖊𝖘𝖙𝖗𝖚𝖈𝖙𝖔𝖗 ~𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫() { \
          if (𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋) { \
            get_assembler().template 𝑒𝑚𝑖𝑡_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝔯𝔢𝔭𝔷>(); \
          } \
        } \
\
       private: \
        𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) get_assembler() { \
          auto* assembler = 𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋; \
          𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖓𝖔𝖙_𝖊𝖖𝖚𝖆𝖑(::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, assembler, nullptr); \
          𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋 = nullptr; \
          return *static_cast<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻*>(assembler); \
        } \
        𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫( \
            𝒃𝒂𝒔𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝔁86_𝓶𝓸𝓭𝓮, 𝓪𝓿𝔁512_𝓮𝔁𝓽𝓻𝓪_𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻𝓼>* assembler) noexcept \
            : 𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋(assembler) {} \
        𝒃𝒂𝒔𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝔁86_𝓶𝓸𝓭𝓮, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>>* 𝖺𝗌𝗌𝖾𝗆𝖻𝗅𝖾𝗋; \
        friend 𝒃𝒂𝒔𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓; \
      }; \
      return 𝐩𝐫𝐞𝐟𝐢𝐱_𝐡𝐨𝐥𝐝𝐞𝐫(*this); \
    } \
\
   private: \
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) get_assembler() { return *static_cast<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻*>(this); } \
  }

#define 𝖎𝖒𝖕𝖔𝖗𝖙_𝖌𝖕_𝖗𝖊𝖌(𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮, 𝓽𝓮𝓶𝓹𝓵𝓪𝓽𝓮_𝓷𝓪𝓶𝓮) \
  template <𝐬𝐢𝐳𝐞 𝓼𝓲𝔃𝓮 = operand_size(𝔵86_𝔪𝔬𝔡𝔢)> /* NOLINTNEXTLINE(bugprone-macro-parentheses) */ \
  using 𝓽𝓮𝓶𝓹𝓵𝓪𝓽𝓮_𝓷𝓪𝓶𝓮 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝓽𝓮𝓶𝓹𝓵𝓪𝓽𝓮_𝓷𝓪𝓶𝓮<𝓼𝓲𝔃𝓮, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔵86_𝔪𝔬𝔡𝔢>; \
  /* C++17 compatibility, use 𝓽𝓮𝓶𝓹𝓵𝓪𝓽𝓮_𝓷𝓪𝓶𝓮 with C++20. */ \
  using 𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 = 𝓽𝓮𝓶𝓹𝓵𝓪𝓽𝓮_𝓷𝓪𝓶𝓮<> /* NOLINT(bugprone-macro-parentheses) */
#define 𝖎𝖒𝖕𝖔𝖗𝖙_𝖘𝖎𝖟𝖊_𝖘𝖕𝖊𝖈𝖎𝖋𝖎𝖈_𝖊𝖓𝖙𝖎𝖙𝖎𝖊𝖘 \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖉𝖉𝖗_𝖜, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔₁₆, 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₁₆, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₁₆, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₁₆, 𝒙𝒍𝒂𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₁₆); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌, 𝔫𝔬_𝔰𝔢𝔤𝔪𝔢𝔫𝔱, 𝔦𝔷, 𝔢𝔰, 𝔠𝔰, 𝔰𝔰, 𝔡𝔰, 𝔣𝔰, 𝔤𝔰);
#define 𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖛𝖝𝟱𝟭𝟮_𝖘𝖎𝖟𝖊_𝖘𝖕𝖊𝖈𝖎𝖋𝖎𝖈_𝖊𝖓𝖙𝖎𝖙𝖎𝖊𝖘 𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖒𝖔𝖉𝖊_𝖆𝖌𝖓𝖔𝖘𝖙𝖎𝖈_𝖗𝖊𝖌, 𝔨0, 𝔨1, 𝔨2, 𝔨3, 𝔨4, 𝔨5, 𝔨6, 𝔨7);
#define 𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖉𝖉𝖗(𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮) \
  template <𝐬𝐢𝐳𝐞 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> \
  using 𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 = /* NOLINTNEXTLINE(bugprone-macro-parentheses) */ \
      𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔵86_𝔪𝔬𝔡𝔢>
#define 𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖉𝖉𝖗_𝖛(𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮) \
  template <𝐬𝐢𝐳𝐞 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> \
  using 𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 = \
      𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿:: /* NOLINTNEXTLINE(bugprone-macro-parentheses) */ \
          𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔵86_𝔪𝔬𝔡𝔢, 𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰>
#define 𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖉𝖉𝖗_𝖜(𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮) \
  template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> /* NOLINTNEXTLINE(bugprone-macro-parentheses) */ \
  using 𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔵86_𝔪𝔬𝔡𝔢>
#define 𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖉𝖉𝖗_𝖛𝖜(𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮) \
  template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> /* NOLINTNEXTLINE(bugprone-macro-parentheses) */ \
  using 𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 = \
      𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔵86_𝔪𝔬𝔡𝔢, 𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰>
#define 𝖎𝖒𝖕𝖔𝖗𝖙_𝖘𝖎𝖒𝖕𝖑𝖊_𝖙𝖞𝖕𝖊(𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮) using 𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 /* NOLINT(bugprone-macro-parentheses) */
#define 𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌(𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮) \
  static constexpr auto 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮<::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔵86_𝔪𝔬𝔡𝔢>
#define 𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌_𝖛(𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮) \
  static constexpr auto 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮 = \
      𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮<::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔵86_𝔪𝔬𝔡𝔢, 𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰>
#define 𝖎𝖒𝖕𝖔𝖗𝖙_𝖒𝖔𝖉𝖊_𝖆𝖌𝖓𝖔𝖘𝖙𝖎𝖈_𝖗𝖊𝖌(𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮) \
  static constexpr auto 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮<::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>>
#define 𝖎𝖒𝖕𝖔𝖗𝖙_𝖘𝖎𝖒𝖕𝖑𝖊_𝖛𝖆𝖗(𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮) static constexpr auto 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻_𝓷𝓪𝓶𝓮
#define 𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌_𝖈𝖍𝖊𝖈𝖐𝖊𝖗(𝓮𝓷𝓽𝓲𝓽𝔂_𝓷𝓪𝓶𝓮) \
  template <typename 𝓣> \
  static constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_##𝓮𝓷𝓽𝓲𝓽𝔂_𝓷𝓪𝓶𝓮##_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔰_##𝓮𝓷𝓽𝓲𝓽𝔂_𝓷𝓪𝓶𝓮##_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯<𝓣>
#define 𝖎𝖒𝖕𝖔𝖗𝖙_𝖘𝖎𝖒𝖕𝖑𝖊_𝖈𝖍𝖊𝖈𝖐𝖊𝖗(𝓮𝓷𝓽𝓲𝓽𝔂_𝓷𝓪𝓶𝓮) \
  template <typename 𝓣> \
  static constexpr 𝐛𝐨𝐨𝐥 𝔦𝔰_##𝓮𝓷𝓽𝓲𝓽𝔂_𝓷𝓪𝓶𝓮 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔰_##𝓮𝓷𝓽𝓲𝓽𝔂_𝓷𝓪𝓶𝓮<𝓣>

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(...) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓, 𝖘𝖚𝖕𝖕𝖔𝖗𝖙𝖊𝖉_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘(__VA_ARGS__))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝓷𝓪𝓶𝓮( \
      𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> arg1, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> arg2) { \
    return get_assembler().template 𝑒𝑚𝑖𝑡_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓲𝓭>(arg1, arg2); \
  }

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖓𝖔𝖔𝖕𝖊𝖗𝖆𝖓𝖉_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(𝓷𝓪𝓶𝓮, 𝓲𝓭) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝓷𝓪𝓶𝓮() { \
    return get_assembler().template 𝑒𝑚𝑖𝑡_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓲𝓭>(); \
  }

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(...) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓, 𝖘𝖚𝖕𝖕𝖔𝖗𝖙𝖊𝖉_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖎𝖟𝖊𝖘(__VA_ARGS__))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝓷𝓪𝓶𝓮(𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> arg1, 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> arg2) { \
    return get_assembler().template 𝑒𝑚𝑖𝑡_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓲𝓭>(arg1, arg2); \
  }

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(...) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓, 𝖘𝖚𝖕𝖕𝖔𝖗𝖙𝖊𝖉_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘(__VA_ARGS__))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝓷𝓪𝓶𝓮(𝒈𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> arg1, 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> arg2) { \
    return get_assembler().template 𝑒𝑚𝑖𝑡_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓲𝓭>(arg1, arg2); \
  } \
  𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓_ₓ𝖎𝖕(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭)
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓_ₓ𝖎𝖕(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭)

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(...) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓, 𝖘𝖚𝖕𝖕𝖔𝖗𝖙𝖊𝖉_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘(__VA_ARGS__))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝓷𝓪𝓶𝓮(𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> arg1, 𝒈𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> arg2) { \
    return get_assembler().template 𝑒𝑚𝑖𝑡_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓲𝓭>(arg1, arg2); \
  } \
  𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓_ₓ𝖎𝖕(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭)
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓_ₓ𝖎𝖕(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭)

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(...) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓, 𝖘𝖚𝖕𝖕𝖔𝖗𝖙𝖊𝖉_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘(__VA_ARGS__))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝓷𝓪𝓶𝓮( \
      𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> arg1, 𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> arg2) { \
    return get_assembler().template 𝑒𝑚𝑖𝑡_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓲𝓭>(arg1, arg2); \
  }

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(...) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓, 𝖘𝖚𝖕𝖕𝖔𝖗𝖙𝖊𝖉_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘(__VA_ARGS__))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝓷𝓪𝓶𝓮( \
      𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> arg1, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> arg2) { \
    return get_assembler().template 𝑒𝑚𝑖𝑡_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓲𝓭>(arg1, arg2); \
  }

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(...) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓, 𝖘𝖚𝖕𝖕𝖔𝖗𝖙𝖊𝖉_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘(__VA_ARGS__))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝓷𝓪𝓶𝓮(𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> arg1, 𝒅𝒂𝒕𝒂_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<16> arg2) { \
    return get_assembler().template 𝑒𝑚𝑖𝑡_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓲𝓭>(arg1, arg2); \
  }

#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓(...) \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖆𝖜(𝖉𝖊𝖋𝖎𝖓𝖊_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓, 𝖘𝖚𝖕𝖕𝖔𝖗𝖙𝖊𝖉_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘(__VA_ARGS__))
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝓷𝓪𝓶𝓮(𝒅𝒂𝒕𝒂_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<16> arg1, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> arg2) { \
    return get_assembler().template 𝑒𝑚𝑖𝑡_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓲𝓭>(arg1, arg2); \
  }

#define 𝖘𝖚𝖕𝖕𝖔𝖗𝖙𝖊𝖉_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖎𝖟𝖊𝖘(...) 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆(__VA_ARGS__)
#define 𝖘𝖚𝖕𝖕𝖔𝖗𝖙𝖊𝖉_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘(...) \
  𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆_₁₆(__VA_ARGS__), 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆_₃₂(__VA_ARGS__)

𝖉𝖊𝖋𝖎𝖓𝖊_𝖇𝖆𝖘𝖎𝖈_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗(𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16, 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬::𝔲𝔫𝔰𝔲𝔭𝔭𝔬𝔯𝔱𝔢𝔡);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖇𝖆𝖘𝖎𝖈_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗(𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32, 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬::𝔲𝔫𝔰𝔲𝔭𝔭𝔬𝔯𝔱𝔢𝔡);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖇𝖆𝖘𝖎𝖈_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗(𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16, 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬::𝔲𝔫𝔰𝔲𝔭𝔭𝔬𝔯𝔱𝔢𝔡);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖇𝖆𝖘𝖎𝖈_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗(𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32, 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬::𝔲𝔫𝔰𝔲𝔭𝔭𝔬𝔯𝔱𝔢𝔡);

#undef 𝖎𝖒𝖕𝖔𝖗𝖙_𝖘𝖎𝖟𝖊_𝖘𝖕𝖊𝖈𝖎𝖋𝖎𝖈_𝖊𝖓𝖙𝖎𝖙𝖎𝖊𝖘
#define 𝖎𝖒𝖕𝖔𝖗𝖙_𝖘𝖎𝖟𝖊_𝖘𝖕𝖊𝖈𝖎𝖋𝖎𝖈_𝖊𝖓𝖙𝖎𝖙𝖎𝖊𝖘 \
  template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> \
  using 𝒆𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝒆𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔵86_𝔪𝔬𝔡𝔢>; \
  template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> \
  using 𝒓𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝒓𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔵86_𝔪𝔬𝔡𝔢>; \
  template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> \
  using ₓ𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₃₂ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝒆𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔵86_𝔪𝔬𝔡𝔢>; \
  template <𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> \
  using ₓ𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₆₄ = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝒓𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔵86_𝔪𝔬𝔡𝔢>; \
  template <𝐬𝐢𝐳𝐞 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝐬𝐢𝐳𝐞 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> \
  using ₓ𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::ₓ𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, ::𝘆𝗮𝗰𝗲::𝘀𝗮𝗻𝗶𝘁𝘆_𝗰𝗵𝗲𝗰𝗸𝘀::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>, 𝔵86_𝔪𝔬𝔡𝔢>; \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖉𝖉𝖗_𝖜, 𝒂𝒅𝒅𝒓𝒆𝒔𝒔₆₄, 𝒂𝒃𝒔𝒐𝒍𝒖𝒕𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₆₄, 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₆₄, 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₆₄, 𝒙𝒍𝒂𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₆₄); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖉𝖉𝖗_𝖛𝖜, 𝒈𝒂𝒕𝒉𝒆𝒓_𝒙𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₆₄, 𝒈𝒂𝒕𝒉𝒆𝒓_𝒚𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₆₄, 𝒈𝒂𝒕𝒉𝒆𝒓_𝒛𝒎𝒎_𝒂𝒅𝒅𝒓𝒆𝒔𝒔₆₄); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌, 𝔰𝔭𝔩, 𝔟𝔭𝔩, 𝔰𝔦𝔩, 𝔡𝔦𝔩, 𝔯8𝔟, 𝔯9𝔟, 𝔯10𝔟, 𝔯11𝔟, 𝔯12𝔟, 𝔯13𝔟, 𝔯14𝔟, 𝔯15𝔟); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌, 𝔯8𝔴, 𝔯9𝔴, 𝔯10𝔴, 𝔯11𝔴, 𝔯12𝔴, 𝔯13𝔴, 𝔯14𝔴, 𝔯15𝔴); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌, 𝔯8𝔡, 𝔯9𝔡, 𝔯10𝔡, 𝔯11𝔡, 𝔯12𝔡, 𝔯13𝔡, 𝔯14𝔡, 𝔯15𝔡); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌, 𝔯𝔞𝔵, 𝔯𝔠𝔵, 𝔯𝔡𝔵, 𝔯𝔟𝔵, 𝔯𝔰𝔭, 𝔯𝔟𝔭, 𝔯𝔰𝔦, 𝔯𝔡𝔦, 𝔯8, 𝔯9, 𝔯10, 𝔯11, 𝔯12, 𝔯13, 𝔯14, 𝔯15, 𝔯𝔦𝔷); \
\
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌_𝖛, 𝔵𝔪𝔪8, 𝔵𝔪𝔪9, 𝔵𝔪𝔪10, 𝔵𝔪𝔪11, 𝔵𝔪𝔪12, 𝔵𝔪𝔪13, 𝔵𝔪𝔪14, 𝔵𝔪𝔪15); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌_𝖛, 𝔶𝔪𝔪8, 𝔶𝔪𝔪9, 𝔶𝔪𝔪10, 𝔶𝔪𝔪11, 𝔶𝔪𝔪12, 𝔶𝔪𝔪13, 𝔶𝔪𝔪14, 𝔶𝔪𝔪15); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌_𝖛, 𝔷𝔪𝔪8, 𝔷𝔪𝔪9, 𝔷𝔪𝔪10, 𝔷𝔪𝔪11, 𝔷𝔪𝔪12, 𝔷𝔪𝔪13, 𝔷𝔪𝔪14, 𝔷𝔪𝔪15); \
\
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌, 𝔫𝔬_𝔰𝔢𝔤𝔪𝔢𝔫𝔱, 𝔣𝔰, 𝔤𝔰); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖘𝖎𝖒𝖕𝖑𝖊_𝖛𝖆𝖗, 𝔢𝔦𝔭, 𝔯𝔦𝔭); \
\
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖘𝖎𝖒𝖕𝖑𝖊_𝖈𝖍𝖊𝖈𝖐𝖊𝖗, 𝔢𝔦𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰, 𝔯𝔦𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰, ₓ𝔦𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰);

#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓_ₓ𝖎𝖕
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓_ₓ𝖎𝖕(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝓷𝓪𝓶𝓮(𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> arg1, ₓ𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> arg2) { \
    return get_assembler().template 𝑒𝑚𝑖𝑡_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓲𝓭>(arg1, arg2); \
  }

#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓_ₓ𝖎𝖕
#define 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓_ₓ𝖎𝖕(𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓷𝓪𝓶𝓮, 𝓲𝓭) \
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝓷𝓪𝓶𝓮(ₓ𝒊𝒑_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓪𝓭𝓭𝓻𝓮𝓼𝓼_𝓼𝓲𝔃𝓮, 𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> arg1, 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<𝓸𝓹𝓮𝓻𝓪𝓷𝓭_𝓼𝓲𝔃𝓮> arg2) { \
    return get_assembler().template 𝑒𝑚𝑖𝑡_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓲𝓭>(arg1, arg2); \
  }

#undef 𝖘𝖚𝖕𝖕𝖔𝖗𝖙𝖊𝖉_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖎𝖟𝖊𝖘
#define 𝖘𝖚𝖕𝖕𝖔𝖗𝖙𝖊𝖉_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖎𝖟𝖊𝖘(...) 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖎𝖟𝖊𝖘(__VA_ARGS__)
#undef 𝖘𝖚𝖕𝖕𝖔𝖗𝖙𝖊𝖉_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘
#define 𝖘𝖚𝖕𝖕𝖔𝖗𝖙𝖊𝖉_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘(...) \
  𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆_₃₂(__VA_ARGS__), 𝖞𝖆𝖈𝖊_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘_ₓ₈₆_₆₄(__VA_ARGS__)

𝖉𝖊𝖋𝖎𝖓𝖊_𝖇𝖆𝖘𝖎𝖈_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗(𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32, 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬::𝔲𝔫𝔰𝔲𝔭𝔭𝔬𝔯𝔱𝔢𝔡);
#undef 𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖛𝖝𝟱𝟭𝟮_𝖘𝖎𝖟𝖊_𝖘𝖕𝖊𝖈𝖎𝖋𝖎𝖈_𝖊𝖓𝖙𝖎𝖙𝖎𝖊𝖘
#define 𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖛𝖝𝟱𝟭𝟮_𝖘𝖎𝖟𝖊_𝖘𝖕𝖊𝖈𝖎𝖋𝖎𝖈_𝖊𝖓𝖙𝖎𝖙𝖎𝖊𝖘 \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖒𝖔𝖉𝖊_𝖆𝖌𝖓𝖔𝖘𝖙𝖎𝖈_𝖗𝖊𝖌, 𝔨0, 𝔨1, 𝔨2, 𝔨3, 𝔨4, 𝔨5, 𝔨6, 𝔨7); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌_𝖛, 𝔵𝔪𝔪16, 𝔵𝔪𝔪17, 𝔵𝔪𝔪18, 𝔵𝔪𝔪19, 𝔵𝔪𝔪20, 𝔵𝔪𝔪21, 𝔵𝔪𝔪22, 𝔵𝔪𝔪23); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌_𝖛, 𝔵𝔪𝔪24, 𝔵𝔪𝔪25, 𝔵𝔪𝔪26, 𝔵𝔪𝔪27, 𝔵𝔪𝔪28, 𝔵𝔪𝔪29, 𝔵𝔪𝔪30, 𝔵𝔪𝔪31); \
\
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌_𝖛, 𝔶𝔪𝔪16, 𝔶𝔪𝔪17, 𝔶𝔪𝔪18, 𝔶𝔪𝔪19, 𝔶𝔪𝔪20, 𝔶𝔪𝔪21, 𝔶𝔪𝔪22, 𝔶𝔪𝔪23); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌_𝖛, 𝔶𝔪𝔪24, 𝔶𝔪𝔪25, 𝔶𝔪𝔪26, 𝔶𝔪𝔪27, 𝔶𝔪𝔪28, 𝔶𝔪𝔪29, 𝔶𝔪𝔪30, 𝔶𝔪𝔪31); \
\
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌_𝖛, 𝔷𝔪𝔪16, 𝔷𝔪𝔪17, 𝔷𝔪𝔪18, 𝔷𝔪𝔪19, 𝔷𝔪𝔪20, 𝔷𝔪𝔪21, 𝔷𝔪𝔪22, 𝔷𝔪𝔪23); \
  𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖘𝖊𝖖𝖚𝖊𝖓𝖈𝖊(𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌_𝖛, 𝔷𝔪𝔪24, 𝔷𝔪𝔪25, 𝔷𝔪𝔪26, 𝔷𝔪𝔪27, 𝔷𝔪𝔪28, 𝔷𝔪𝔪29, 𝔷𝔪𝔪30, 𝔷𝔪𝔪31);
𝖉𝖊𝖋𝖎𝖓𝖊_𝖇𝖆𝖘𝖎𝖈_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗(𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32, 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐯𝐱𝟓𝟏𝟐_𝐞𝐱𝐭𝐫𝐚_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫𝐬::𝔰𝔲𝔭𝔭𝔬𝔯𝔱𝔢𝔡);

#undef 𝖘𝖚𝖕𝖕𝖔𝖗𝖙𝖊𝖉_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖆𝖓𝖉_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖎𝖟𝖊𝖘
#undef 𝖘𝖚𝖕𝖕𝖔𝖗𝖙𝖊𝖉_𝖝𝟴𝟲_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖎𝖟𝖊𝖘
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖕𝖔𝖗𝖙_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓_ₓ𝖎𝖕
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖒_𝖙𝖔_𝖗𝖊𝖌_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓_ₓ𝖎𝖕
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖒𝖊𝖒𝖔𝖗𝖞_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖊𝖌_𝖙𝖔_𝖗𝖒_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖓𝖔𝖔𝖕𝖊𝖗𝖆𝖓𝖉_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖋𝖚𝖓𝖈𝖙𝖎𝖔𝖓
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖌𝖕_𝖗𝖊𝖌𝖎𝖘𝖙𝖊𝖗_𝖒𝖊𝖒𝖔𝖗𝖞_𝖘𝖙𝖗𝖎𝖓𝖌_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓
#undef 𝖎𝖒𝖕𝖔𝖗𝖙_𝖘𝖎𝖒𝖕𝖑𝖊_𝖈𝖍𝖊𝖈𝖐𝖊𝖗
#undef 𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌_𝖈𝖍𝖊𝖈𝖐𝖊𝖗
#undef 𝖎𝖒𝖕𝖔𝖗𝖙_𝖘𝖎𝖒𝖕𝖑𝖊_𝖛𝖆𝖗
#undef 𝖎𝖒𝖕𝖔𝖗𝖙_𝖒𝖔𝖉𝖊_𝖆𝖌𝖓𝖔𝖘𝖙𝖎𝖈_𝖗𝖊𝖌
#undef 𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌_𝖛
#undef 𝖎𝖒𝖕𝖔𝖗𝖙_𝖗𝖊𝖌
#undef 𝖎𝖒𝖕𝖔𝖗𝖙_𝖘𝖎𝖒𝖕𝖑𝖊_𝖙𝖞𝖕𝖊
#undef 𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖉𝖉𝖗_𝖛𝖜
#undef 𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖉𝖉𝖗_𝖜
#undef 𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖉𝖉𝖗_𝖛
#undef 𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖉𝖉𝖗
#undef 𝖎𝖒𝖕𝖔𝖗𝖙_𝖆𝖛𝖝𝟱𝟭𝟮_𝖘𝖎𝖟𝖊_𝖘𝖕𝖊𝖈𝖎𝖋𝖎𝖈_𝖊𝖓𝖙𝖎𝖙𝖎𝖊𝖘
#undef 𝖎𝖒𝖕𝖔𝖗𝖙_𝖘𝖎𝖟𝖊_𝖘𝖕𝖊𝖈𝖎𝖋𝖎𝖈_𝖊𝖓𝖙𝖎𝖙𝖎𝖊𝖘
#undef 𝖎𝖒𝖕𝖔𝖗𝖙_𝖌𝖕_𝖗𝖊𝖌
#undef 𝖉𝖊𝖋𝖎𝖓𝖊_𝖇𝖆𝖘𝖎𝖈_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗

}  // namespace 𝘆𝗮𝗰𝗲::𝘅𝟴𝟲

#endif  // 𝔜𝔄ℭ𝔈_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_𝔛86_𝔅𝔄𝔖ℑℭ_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_ℌ
