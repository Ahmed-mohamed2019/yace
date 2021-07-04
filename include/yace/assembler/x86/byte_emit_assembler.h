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

#ifndef 𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_𝔛86_𝔅𝔜𝔗𝔈_𝔈𝔐ℑ𝔗_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_ℌ
#define 𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_𝔛86_𝔅𝔜𝔗𝔈_𝔈𝔐ℑ𝔗_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_ℌ

#include <array>
#include <tuple>
#include <type_traits>

#include "yace/foundation.h"

#include "yace/assembler/x86/basic_assembler.h"
#include "yace/assembler/x86/options.h"

namespace 𝘆𝗮𝗰𝗲::𝘅𝟴𝟲 {

// Assembler capable of emitting bytes.  Note: it delegates work of actually allocating
// memory, etc to descendant assembler.
template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>
class 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓 : public 𝒃𝒂𝒔𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼> {
  using ⒫ = 𝒃𝒂𝒔𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>;
  using ⒮ = 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>;
  friend ⒫;

 protected:
#define 𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖈𝖍𝖊𝖈𝖐𝖘(...) (𝖞𝖆𝖈𝖊_𝖉𝖊𝖋𝖎𝖓𝖊_𝖈𝖔𝖗𝖊(𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖈𝖍𝖊𝖈𝖐, (or), 𝖞𝖆𝖈𝖊_𝖕𝖆𝖗𝖊𝖓𝖘((), (), __VA_ARGS__)))
#define 𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖈𝖍𝖊𝖈𝖐(𝓷𝓪𝓶𝓮) 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮 == 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧::𝓷𝓪𝓶𝓮
  template <𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮, typename... 𝓣>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝑒𝑚𝑖𝑡_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛(const 𝓣... 𝓽) {
    // Some old 8086 instructions are “one of kind”: very quirky and follow special rules.
    // It's easier to detect and handle them specially than to try to process then with all others.
    // Here's the list:
    //   𝔠𝔪𝔭𝔰/𝔪𝔬𝔳𝔰 — two explicit memory arguments, but they have to always be “xxx PTR [xSI]” and “xxx PTR [xDI]”.
    //   𝔦𝔫𝔰 - two operands, one always “xxx PTR [xDI]”, one always “DX” (even in 32-bit/64-bit mode).
    //   𝔩𝔬𝔡𝔰 — one explicit memory argument, but it has to always be “xxx PTR [xSI]”.
    //   𝔬𝔲𝔱𝔰 - two operands, one always “DX” (even in 32-bit/64-bit mode), one always “xxx PTR [xDI]”.
    //   𝔰𝔠𝔞𝔰/𝔰𝔱𝔬𝔰 — one explicit memory argument, but it has to always be “xxx PTR [xDI]”.

    if constexpr (
        𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖈𝖍𝖊𝖈𝖐𝖘(𝔠𝔪𝔭𝔰, 𝔦𝔫𝔰, 𝔩𝔬𝔡𝔰, 𝔪𝔬𝔳𝔰, 𝔬𝔲𝔱𝔰, 𝔰𝔠𝔞𝔰, 𝔰𝔱𝔬𝔰) or
        𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖈𝖍𝖊𝖈𝖐𝖘(𝔯𝔢𝔭𝔷_𝔠𝔪𝔭𝔰, 𝔯𝔢𝔭_𝔦𝔫𝔰, 𝔯𝔢𝔭_𝔩𝔬𝔡𝔰, 𝔯𝔢𝔭_𝔪𝔬𝔳𝔰, 𝔯𝔢𝔭_𝔬𝔲𝔱𝔰, 𝔯𝔢𝔭𝔷_𝔰𝔠𝔞𝔰, 𝔯𝔢𝔭_𝔰𝔱𝔬𝔰) or
        𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖈𝖍𝖊𝖈𝖐𝖘(𝔯𝔢𝔭𝔫𝔷_𝔠𝔪𝔭𝔰, 𝔯𝔢𝔭𝔫𝔷_𝔦𝔫𝔰, 𝔯𝔢𝔭𝔫𝔷_𝔩𝔬𝔡𝔰, 𝔯𝔢𝔭𝔫𝔷_𝔪𝔬𝔳𝔰, 𝔯𝔢𝔭𝔫𝔷_𝔬𝔲𝔱𝔰, 𝔯𝔢𝔭𝔫𝔷_𝔰𝔠𝔞𝔰, 𝔯𝔢𝔭𝔫𝔷_𝔰𝔱𝔬𝔰)) {
      return 𝑒𝑚𝑖𝑡_𝑠𝑡𝑟𝑖𝑛𝑔_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮>(𝓽...);
    } else {
      // Detailed information about instruction.
      using 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨 = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐<𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓, 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮, 𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<𝓣>...>;
      std::tuple arguments{𝓽...};

      if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔳𝔢𝔵_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫<𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨>) {
        return 𝑒𝑚𝑖𝑡_𝑣𝑒𝑥_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨>(arguments);
      } else if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔢𝔳𝔢𝔵_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫<𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨>) {
        return 𝑒𝑚𝑖𝑡_𝑒𝑣𝑒𝑥_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨>(arguments);
      } else if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔵𝔬𝔭_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫<𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨>) {
        // Note: 𝔳𝔢𝔵-encoding and 𝔵𝔬𝔭-encoding are very similar, only 𝔳𝔢𝔵-encoding have short 2-byte form.
        // 𝑒𝑚𝑖𝑡_𝑣𝑒𝑥_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛 handles short case  or calls 𝑒𝑚𝑖𝑡_𝑙𝑜𝑛𝑔𝑣𝑒𝑥_𝑜𝑟_𝑥𝑜𝑝_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛 to handle 3-byte form.
        return 𝑒𝑚𝑖𝑡_𝑙𝑜𝑛𝑔𝑣𝑒𝑥_𝑜𝑟_𝑥𝑜𝑝_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨>(arguments);
      } else {
        return 𝑒𝑚𝑖𝑡_𝑙𝑒𝑔𝑎𝑐𝑦_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨>(arguments);
      }
    }
  }

  // Default implementations.  Used if 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻 doesn't offer any optimized versions.
  // Note: emit₁₆/emit₃₂/emit₆₄ must emit value as little-endian integer.
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) emit₁₆(𝐮𝐢𝐧𝐭₁₆ value) {
    // NOLINTNEXTLINE(hicpp-signed-bitwise)
    return get_assembler().emit₈(𝐮𝐢𝐧𝐭₈(value)).emit₈(𝐮𝐢𝐧𝐭₈(value >> 𝐮𝐢𝐧𝐭₁₆(8)));
  }
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) emit₃₂(𝐮𝐢𝐧𝐭₃₂ value) {
    // NOLINTNEXTLINE(hicpp-signed-bitwise)
    return get_assembler().emit₁₆(𝐮𝐢𝐧𝐭₁₆(value)).emit₁₆(𝐮𝐢𝐧𝐭₁₆(value >> 𝐮𝐢𝐧𝐭₃₂(16)));
  }
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) emit₆₄(𝐮𝐢𝐧𝐭₆₄ value) {
    // NOLINTNEXTLINE(hicpp-signed-bitwise)
    return get_assembler().emit₃₂(𝐮𝐢𝐧𝐭₃₂(value)).emit₃₂(𝐮𝐢𝐧𝐭₃₂(value >> 𝐮𝐢𝐧𝐭₆₄(32)));
  }

  // Instructions are emitted ass three arrays: prefixes+opcode with 𝑒𝑚𝑖𝑡ₒₚ, then ModRM+SIB+disp with 𝑒𝑚𝑖𝑡ₘᵣₘ and, finally, 𝑒𝑚𝑖𝑡ᵢₘₘ.
  // Only 𝑒𝑚𝑖𝑡ₒₚ are used for all instructions, 𝑒𝑚𝑖𝑡ₘᵣₘ and 𝑒𝑚𝑖𝑡ᵢₘₘ may be skipped.
  template <𝐬𝐢𝐳𝐞 𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝑒𝑚𝑖𝑡ₒₚ(const std::array<𝐮𝐢𝐧𝐭₈, 𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱>& array) {
    return get_assembler().𝑒𝑚𝑖𝑡ₐᵣᵣ(array);
  }
  template <𝐬𝐢𝐳𝐞 𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝑒𝑚𝑖𝑡ₘᵣₘ(const std::array<𝐮𝐢𝐧𝐭₈, 𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱>& array) {
    return get_assembler().𝑒𝑚𝑖𝑡ₐᵣᵣ(array);
  }
  template <𝐬𝐢𝐳𝐞 𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝑒𝑚𝑖𝑡ᵢₘₘ(const std::array<𝐮𝐢𝐧𝐭₈, 𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱>& array) {
    return get_assembler().𝑒𝑚𝑖𝑡ₐᵣᵣ(array);
  }

  // See 𝒃𝒂𝒔𝒊𝒄_𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓 for explanation about why we emit instructions are sequences of up-to-8-bytes.
  template <𝐬𝐢𝐳𝐞 𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝑒𝑚𝑖𝑡ₐᵣᵣ(const std::array<𝐮𝐢𝐧𝐭₈, 𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱>& array) {
    static_assert(𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 > 0);
    static_assert(𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 <= 8);
    if constexpr (𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 == 1) {
      return get_assembler().emit₈(array[0]);
    } else if constexpr (𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 == 2) {
      // NOLINTNEXTLINE(hicpp-signed-bitwise)
      return get_assembler().emit₁₆(𝐮𝐢𝐧𝐭₁₆((𝐮𝐢𝐧𝐭₁₆(array[1]) << 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor 𝐮𝐢𝐧𝐭₁₆(array[0])));
    } else if constexpr (𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 == 3) {
      // NOLINTNEXTLINE(hicpp-signed-bitwise)
      return get_assembler().emit₈(array[0]).emit₁₆(𝐮𝐢𝐧𝐭₁₆((𝐮𝐢𝐧𝐭₁₆(array[2]) << 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor 𝐮𝐢𝐧𝐭₁₆(array[1])));
    } else if constexpr (𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 == 4) {
      return get_assembler().emit₃₂(
          // NOLINTNEXTLINE(hicpp-signed-bitwise)
          (𝐮𝐢𝐧𝐭₃₂(array[3]) << 3 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor (𝐮𝐢𝐧𝐭₃₂(array[2]) << 2 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor (𝐮𝐢𝐧𝐭₃₂(array[1]) << 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor
          𝐮𝐢𝐧𝐭₃₂(array[0]));
    } else if constexpr (𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 == 5) {
      return get_assembler().emit₈(array[0]).emit₃₂(
          // NOLINTNEXTLINE(hicpp-signed-bitwise)
          (𝐮𝐢𝐧𝐭₃₂(array[4]) << 3 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor (𝐮𝐢𝐧𝐭₃₂(array[3]) << 2 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor (𝐮𝐢𝐧𝐭₃₂(array[2]) << 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor
          𝐮𝐢𝐧𝐭₃₂(array[1]));
    } else if constexpr (𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 == 6) {
      return get_assembler()
          // NOLINTNEXTLINE(hicpp-signed-bitwise)
          .emit₁₆(𝐮𝐢𝐧𝐭₁₆((𝐮𝐢𝐧𝐭₁₆(array[1]) << 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor 𝐮𝐢𝐧𝐭₁₆(array[0])))
          .emit₃₂(
              // NOLINTNEXTLINE(hicpp-signed-bitwise)
              (𝐮𝐢𝐧𝐭₃₂(array[5]) << 3 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor (𝐮𝐢𝐧𝐭₃₂(array[4]) << 2 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor (𝐮𝐢𝐧𝐭₃₂(array[3]) << 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor
              𝐮𝐢𝐧𝐭₃₂(array[2]));
    } else if constexpr (𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 == 7) {
      return get_assembler()
          .emit₈(array[0])
          // NOLINTNEXTLINE(hicpp-signed-bitwise)
          .emit₁₆((𝐮𝐢𝐧𝐭₁₆(array[2]) << 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor 𝐮𝐢𝐧𝐭₁₆(array[1]))
          // NOLINTNEXTLINE(hicpp-signed-bitwise)
          .emit₃₂(
              (𝐮𝐢𝐧𝐭₃₂(array[6]) << 3 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor (𝐮𝐢𝐧𝐭₃₂(array[5]) << 2 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor (𝐮𝐢𝐧𝐭₃₂(array[4]) << 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor
              𝐮𝐢𝐧𝐭₃₂(array[3]));
    } else if constexpr (𝓪𝓻𝓻𝓪𝔂_𝓵𝓮𝓷𝓰𝓽𝓱 == 8) {
      return get_assembler().emit₆₄(
          // NOLINTNEXTLINE(hicpp-signed-bitwise)
          (𝐮𝐢𝐧𝐭₆₄(array[7]) << 7 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor (𝐮𝐢𝐧𝐭₆₄(array[6]) << 6 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor (𝐮𝐢𝐧𝐭₆₄(array[5]) << 5 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor
          // NOLINTNEXTLINE(hicpp-signed-bitwise)
          (𝐮𝐢𝐧𝐭₆₄(array[4]) << 4 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor (𝐮𝐢𝐧𝐭₆₄(array[3]) << 3 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor (𝐮𝐢𝐧𝐭₆₄(array[2]) << 2 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor
          // NOLINTNEXTLINE(hicpp-signed-bitwise)
          (𝐮𝐢𝐧𝐭₆₄(array[1]) << 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢) bitor 𝐮𝐢𝐧𝐭₆₄(array[0]));
    }
  }

 private:
  template <𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮, typename 𝓣₁, typename 𝓣₂>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝑒𝑚𝑖𝑡_𝑠𝑡𝑟𝑖𝑛𝑔_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛(const 𝓣₁ /*destination*/, const 𝓣₂ source) {
    // Detailed information about instruction.
    using 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨 =
        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏_𝒊𝒏𝒇𝒐<𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓, 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮, 𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<𝓣₁>, 𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<𝓣₂>>;

    if constexpr (
        𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖈𝖍𝖊𝖈𝖐𝖘(𝔠𝔪𝔭𝔰, 𝔩𝔬𝔡𝔰, 𝔪𝔬𝔳𝔰, 𝔬𝔲𝔱𝔰) or
        𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖈𝖍𝖊𝖈𝖐𝖘(𝔯𝔢𝔭𝔷_𝔠𝔪𝔭𝔰, 𝔯𝔢𝔭_𝔩𝔬𝔡𝔰, 𝔯𝔢𝔭_𝔪𝔬𝔳𝔰, 𝔯𝔢𝔭_𝔬𝔲𝔱𝔰, 𝔯𝔢𝔭𝔫𝔷_𝔠𝔪𝔭𝔰, 𝔯𝔢𝔭𝔫𝔷_𝔩𝔬𝔡𝔰, 𝔯𝔢𝔭𝔫𝔷_𝔪𝔬𝔳𝔰, 𝔯𝔢𝔭𝔫𝔷_𝔬𝔲𝔱𝔰)) {
      if (source.segment == ⒮::𝔫𝔬_𝔰𝔢𝔤𝔪𝔢𝔫𝔱)
        𝖞𝖆𝖈𝖊_𝖑𝖎𝖐𝖊𝖑𝖞 { return get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨::𝔬𝔭𝔠𝔬𝔡𝔢𝔰); }
      // Note: it's not clear how we should handle rep_before_segment when there are more than one prefix: it's only important for
      // 8086 CPU and more than one prefix could only happen on 80386+… put segment override prefix directly before opcode for now.
      if constexpr (𝔯𝔢𝔭_𝔟𝔢𝔣𝔬𝔯𝔢_𝔰𝔢𝔤𝔪𝔢𝔫𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>) {
        return get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
            𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨>,
            std::array{𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(source.segment))},
            𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨>));
      } else {
        return get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(std::array{𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(source.segment))}, 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨::𝔬𝔭𝔠𝔬𝔡𝔢𝔰));
      }
    } else if constexpr (
        𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖈𝖍𝖊𝖈𝖐𝖘(𝔦𝔫𝔰, 𝔰𝔠𝔞𝔰, 𝔰𝔱𝔬𝔰) or
        𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖈𝖍𝖊𝖈𝖐𝖘(𝔯𝔢𝔭_𝔦𝔫𝔰, 𝔯𝔢𝔭𝔷_𝔰𝔠𝔞𝔰, 𝔯𝔢𝔭_𝔰𝔱𝔬𝔰, 𝔯𝔢𝔭𝔫𝔷_𝔦𝔫𝔰, 𝔯𝔢𝔭𝔫𝔷_𝔰𝔠𝔞𝔰, 𝔯𝔢𝔭𝔫𝔷_𝔰𝔱𝔬𝔰)) {
      return get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐢𝐧𝐟𝐨::𝔬𝔭𝔠𝔬𝔡𝔢𝔰);
    } else {
      static_assert(𝒇𝒂𝒍𝒔𝒆<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓷𝓪𝓶𝓮>);
    }
  }
#undef 𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖈𝖍𝖊𝖈𝖐
#undef 𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓_𝖈𝖍𝖊𝖈𝖐𝖘

  template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸, typename... 𝓣>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝑒𝑚𝑖𝑡_𝑙𝑒𝑔𝑎𝑐𝑦_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛(const std::tuple<𝓣...>& arguments) {
    𝐮𝐢𝐧𝐭₈ segment = 0;
    if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
      auto rm = std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments);
      if constexpr (⒮::template 𝔦𝔰_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>>) {
        segment = 𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(rm.segment));
      }
    }
    // All the complications related to rex don't exist in 16-bit/32-bit mode.
    if constexpr (⒮::𝔵86_𝔪𝔬𝔡𝔢 <= 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32) {
      if (not segment)
        𝖞𝖆𝖈𝖊_𝖑𝖎𝖐𝖊𝖑𝖞 {
          return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
              get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
                  𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
                  𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>)),
              arguments);
        }
      return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
          get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
              std::array{𝐮𝐢𝐧𝐭₈(segment)},
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>)),
          arguments);
    } else if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔯𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
      // Handle case where rex is mandatory separately to help compiler optimizer.
      if (not segment)
        𝖞𝖆𝖈𝖊_𝖑𝖎𝖐𝖊𝖑𝖞 {
          return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
              get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
                  𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
                  std::array{𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑟𝑒𝑥<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(arguments)},
                  𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>)),
              arguments);
        }
      return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
          get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
              std::array{𝐮𝐢𝐧𝐭₈(segment)},
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
              std::array{𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑟𝑒𝑥<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(arguments)},
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>)),
          arguments);
    } else {
      𝐮𝐢𝐧𝐭₈ rex = 𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑟𝑒𝑥<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(arguments);
      if (not rex)
        𝖞𝖆𝖈𝖊_𝖑𝖎𝖐𝖊𝖑𝖞 {
          if (not segment)
            𝖞𝖆𝖈𝖊_𝖑𝖎𝖐𝖊𝖑𝖞 {
              return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
                  get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
                      𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
                      𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>)),
                  arguments);
            }
          return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
              get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
                  std::array{𝐮𝐢𝐧𝐭₈(segment)},
                  𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
                  𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>)),
              arguments);
        }
      if (not segment)
        𝖞𝖆𝖈𝖊_𝖑𝖎𝖐𝖊𝖑𝖞 {
          return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
              get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
                  𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
                  std::array{rex},
                  𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>)),
              arguments);
        }
      return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
          get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
              std::array{𝐮𝐢𝐧𝐭₈(segment)},
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
              std::array{rex},
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>)),
          arguments);
    }
  }

  template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸, typename... 𝓣>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝑒𝑚𝑖𝑡_𝑣𝑒𝑥_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛(const std::tuple<𝓣...>& arguments) {
    static_assert(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔳𝔢𝔵_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>);
    static_assert(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() == 3);
    static_assert(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0] == 0xc4);
    static_assert(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() == 1);
    if constexpr (
        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔬𝔭𝔠𝔬𝔡𝔢_𝔦𝔫_𝔪𝔬𝔡𝔯𝔪<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸> or
        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔬𝔭𝔠𝔬𝔡𝔢_𝔦𝔫_𝔦𝔪𝔪<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
      static_assert(
          𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔢𝔵𝔱𝔢𝔫𝔰𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() ==
          𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰.size());
    } else {
      static_assert(
          𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() ==
          𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰.size());
    }
    𝐮𝐢𝐧𝐭₈ segment = 0;
    if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
      auto rm = std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments);
      if constexpr (⒮::template 𝔦𝔰_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>>) {
        segment = 𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(rm.segment));
      }
    }
    𝐮𝐢𝐧𝐭₈ reg_operand_byte = 0;
    if constexpr (
        ⒮::𝔵86_𝔪𝔬𝔡𝔢 >= 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32 and 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔢𝔤_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
      auto reg = std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔯𝔢𝔤_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments);
      // If we have 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, or 𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 then we must ensure that we are not dealing with ₓ𝔪𝔪15-ₓ𝔪𝔪31.
      if constexpr (
          𝐛𝐨𝐨𝐥(⒮::𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰) and (std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(reg)>, typename ⒮::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
                                               std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(reg)>, typename ⒮::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
                                               std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(reg)>, typename ⒮::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>)) {
        𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖇𝖊𝖑𝖔𝖜_𝖔𝖗_𝖊𝖖𝖚𝖆𝖑(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, reg, ⒮::𝔵𝔪𝔪15);
      }
      reg_operand_byte = (𝐢𝐧𝐭₈(reg) bitand 0b0000'1000) << 4;  // NOLINT(hicpp-signed-bitwise)
    }
    𝐮𝐢𝐧𝐭₈ vex_operand_byte = 0;
    if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔳𝔢𝔵_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
      if constexpr (⒮::𝔵86_𝔪𝔬𝔡𝔢 <= 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32) {
        vex_operand_byte = 𝐢𝐧𝐭₈(std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔳𝔢𝔵_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments)) << 3;  // NOLINT(hicpp-signed-bitwise)
      } else {
        auto vex = std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔳𝔢𝔵_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments);
        // If we have 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, or 𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 then we must ensure that we are not dealing with ₓ𝔪𝔪15-ₓ𝔪𝔪31.
        if constexpr (
            𝐛𝐨𝐨𝐥(⒮::𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰) and (std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(vex)>, typename ⒮::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
                                                 std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(vex)>, typename ⒮::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
                                                 std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(vex)>, typename ⒮::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>)) {
          𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖇𝖊𝖑𝖔𝖜_𝖔𝖗_𝖊𝖖𝖚𝖆𝖑(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, vex, ⒮::𝔵𝔪𝔪15);
        }
        vex_operand_byte = 𝐢𝐧𝐭₈(vex) << 3;  // NOLINT(hicpp-signed-bitwise)
      }
    }
    // Only certain subset of 𝔳𝔢𝔵-encoded instructions could have short form.
    if constexpr (
        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[1] == 0b111'00001 and
        (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[2] bitand 0b1'0000'0'00) == 0) {
      // In 16-bit or 32-bit mode we don't have X̅ or B̅ bits, so could emit opcode without checking.
      if constexpr (⒮::𝔵86_𝔪𝔬𝔡𝔢 <= 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32) {
        if (not segment)
          𝖞𝖆𝖈𝖊_𝖑𝖎𝖐𝖊𝖑𝖞 {
            return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
                get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
                    𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
                    std::array{
                        𝐮𝐢𝐧𝐭₈(0xc5),
                        𝐮𝐢𝐧𝐭₈(
                            ((𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[1] bitand 0b1'0000'0'00) bitor
                             (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[2] bitand 0b0'11'11111)) xor
                            vex_operand_byte),
                        𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0])})),
                arguments);
          }
        return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
            get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
                std::array{𝐮𝐢𝐧𝐭₈(segment)},
                𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
                std::array{
                    𝐮𝐢𝐧𝐭₈(0xc5),
                    𝐮𝐢𝐧𝐭₈(
                        ((𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[1] bitand 0b1'0000'0'00) bitor
                         (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[2] bitand 0b0'11'11111)) xor
                        vex_operand_byte),
                    𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0])})),
            arguments);
      } else if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
        auto rm = std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments);
        if constexpr (⒮::template 𝔦𝔰_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>>) {
          // Note: currently there are no instructions that combine opcode map 1 and vector address.
          static_assert(⒮::template 𝔦𝔰_𝔤𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>>);
          // If registers for base and/or index are both low - we can use short form.
          if (𝐢𝐧𝐭₈(rm.base) <= 7 and 𝐢𝐧𝐭₈(rm.index) <= 7) {
            if (not segment)
              𝖞𝖆𝖈𝖊_𝖑𝖎𝖐𝖊𝖑𝖞 {
                return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
                    get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
                        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
                        std::array{
                            𝐮𝐢𝐧𝐭₈(0xc5),
                            𝐮𝐢𝐧𝐭₈(
                                ((𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[1] bitand 0b1'0000'0'00) bitor
                                 (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[2] bitand 0b0'11'11111)) xor
                                (reg_operand_byte bitor vex_operand_byte)),
                            𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0])})),
                    arguments);
              }
            return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
                get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
                    std::array{𝐮𝐢𝐧𝐭₈(segment)},
                    𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
                    std::array{
                        𝐮𝐢𝐧𝐭₈(0xc5),
                        𝐮𝐢𝐧𝐭₈(
                            ((𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[1] bitand 0b1'0000'0'00) bitor
                             (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[2] bitand 0b0'11'11111)) xor
                            (reg_operand_byte bitor vex_operand_byte)),
                        𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0])})),
                arguments);
          }
        } else {
          // There are no 𝔳𝔢𝔵-encoded instructions with these operands so it's not clean how should we treat them.
          static_assert(
              not std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::template 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<8>> and
              not std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> and
              not std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> and
              not std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐦𝐦𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>);
          // If rm is low registers then we can use short form.
          if (𝐢𝐧𝐭₈(rm) <= 7) {
            if (not segment)
              𝖞𝖆𝖈𝖊_𝖑𝖎𝖐𝖊𝖑𝖞 {
                return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
                    get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
                        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
                        std::array{
                            𝐮𝐢𝐧𝐭₈(0xc5),
                            𝐮𝐢𝐧𝐭₈(
                                ((𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[1] bitand 0b1'0000'0'00) bitor
                                 (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[2] bitand 0b0'11'11111)) xor
                                (reg_operand_byte bitor vex_operand_byte)),
                            𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0])})),
                    arguments);
              }
            return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
                get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
                    std::array{𝐮𝐢𝐧𝐭₈(segment)},
                    𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
                    std::array{
                        𝐮𝐢𝐧𝐭₈(0xc5),
                        𝐮𝐢𝐧𝐭₈(
                            ((𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[1] bitand 0b1'0000'0'00) bitor
                             (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[2] bitand 0b0'11'11111)) xor
                            (reg_operand_byte bitor vex_operand_byte)),
                        𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0])})),
                arguments);
          }
        }
      } else {
        if (not segment)
          𝖞𝖆𝖈𝖊_𝖑𝖎𝖐𝖊𝖑𝖞 {
            return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
                get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
                    𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
                    std::array{
                        𝐮𝐢𝐧𝐭₈(0xc5),
                        𝐮𝐢𝐧𝐭₈(
                            ((𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[1] bitand 0b1'0000'0'00) bitor
                             (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[2] bitand 0b0'11'11111)) xor
                            (reg_operand_byte bitor vex_operand_byte)),
                        𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0])})),
                arguments);
          }
        return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
            get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
                std::array{𝐮𝐢𝐧𝐭₈(segment)},
                𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
                std::array{
                    𝐮𝐢𝐧𝐭₈(0xc5),
                    𝐮𝐢𝐧𝐭₈(
                        ((𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[1] bitand 0b1'0000'0'00) bitor
                         (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[2] bitand 0b0'11'11111)) xor
                        (reg_operand_byte bitor vex_operand_byte)),
                    𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0])})),
            arguments);
      }
      // Note: technically 𝔳𝔢𝔵-encoded instructions and 𝔵𝔬𝔭-encoded instructions are not the same, but the only difference is
      // possibility of “short encoding” of certain 𝔳𝔢𝔵-encoded instructions. This is handled above.
      return 𝑒𝑚𝑖𝑡_𝑙𝑜𝑛𝑔𝑣𝑒𝑥_𝑜𝑟_𝑥𝑜𝑝_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(arguments);
    }
  }

  template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸, typename... 𝓣>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝑒𝑚𝑖𝑡_𝑒𝑣𝑒𝑥_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛(const std::tuple<𝓣...>& arguments) {
    static_assert(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔢𝔳𝔢𝔵_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>);
    static_assert(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔢𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() == 4);
    static_assert(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔢𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0] == 0x62);
    static_assert(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() == 1);
    if constexpr (
        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔬𝔭𝔠𝔬𝔡𝔢_𝔦𝔫_𝔪𝔬𝔡𝔯𝔪<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸> or
        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔬𝔭𝔠𝔬𝔡𝔢_𝔦𝔫_𝔦𝔪𝔪<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
      static_assert(
          𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔢𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔢𝔵𝔱𝔢𝔫𝔰𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() ==
          𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰.size());
    } else {
      static_assert(
          𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔢𝔳𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() ==
          𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰.size());
    }
    // We need to handle R̅′R̅, X̅, B̅, or V̅′ bits only in 64-bit mode.
    if constexpr (⒮::𝔵86_𝔪𝔬𝔡𝔢 <= 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32) {
      𝐮𝐢𝐧𝐭₈ segment = 0;
      𝐮𝐢𝐧𝐭₈ vex_operand_byte = 0;
      𝐮𝐢𝐧𝐭₈ mask_operand_byte = 0;
      if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
        auto rm = std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments);
        if constexpr (⒮::template 𝔦𝔰_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>>) {
          segment = 𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(rm.segment));
        }
      }
      if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔳𝔢𝔵_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
        vex_operand_byte = 𝐢𝐧𝐭₈(std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔳𝔢𝔵_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments)) << 3;  // NOLINT(hicpp-signed-bitwise)
      }
      if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔪𝔞𝔰𝔨_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
        mask_operand_byte = 𝐢𝐧𝐭₈(std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔪𝔞𝔰𝔨_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments));
      }
      if (not segment)
        𝖞𝖆𝖈𝖊_𝖑𝖎𝖐𝖊𝖑𝖞 {
          return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
              get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
                  𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
                  std::array{
                      𝐮𝐢𝐧𝐭₈(0x62),
                      𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[1]),
                      𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[2] xor vex_operand_byte),
                      𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[2] xor mask_operand_byte),
                      𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0])})),
              arguments);
        }
      return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
          get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
              std::array{𝐮𝐢𝐧𝐭₈(segment)},
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
              std::array{
                  𝐮𝐢𝐧𝐭₈(0x62),
                  𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[1]),
                  𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[2] xor vex_operand_byte),
                  𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[3] xor mask_operand_byte),
                  𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0])})),
          arguments);
    } else {
      𝐮𝐢𝐧𝐭₈ segment = 0;
      𝐮𝐢𝐧𝐭₈ reg_operand_byte = 0;
      𝐮𝐢𝐧𝐭₈ vex_operand_byte = 0;
      𝐮𝐢𝐧𝐭₈ mask_operand_byte = 0;
      if (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔢𝔤_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
        auto reg = std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔯𝔢𝔤_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments);
        if constexpr (
            𝐛𝐨𝐨𝐥(⒮::𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰) and (std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(reg)>, typename ⒮::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
                                                 std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(reg)>, typename ⒮::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
                                                 std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(reg)>, typename ⒮::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>)) {
          // NOLINTNEXTLINE(hicpp-signed-bitwise)
          reg_operand_byte = ((𝐢𝐧𝐭₈(reg) bitand 0b000'01000) << 4) bitor (𝐢𝐧𝐭₈(reg) bitand 0b000'10000);
        } else {
          reg_operand_byte = (𝐢𝐧𝐭₈(reg) bitand 0b0000'1000) << 4;  // NOLINT(hicpp-signed-bitwise)
        }
      }
      if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔳𝔢𝔵_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
        auto vex = std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔳𝔢𝔵_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments);
        vex_operand_byte = 𝐢𝐧𝐭₈(vex) << 3;  // NOLINT(hicpp-signed-bitwise)
        if constexpr (
            𝐛𝐨𝐨𝐥(⒮::𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰) and (std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(vex)>, typename ⒮::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
                                                 std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(vex)>, typename ⒮::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
                                                 std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(vex)>, typename ⒮::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>)) {
          if (𝐢𝐧𝐭₈(vex_operand_byte) < 0) {
            mask_operand_byte = 0b0'00'0'1'000;
          }
          vex_operand_byte and_eq 0b0'1111'1'11;  // NOLINT(hicpp-signed-bitwise)
        }
      }
      if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
        auto rm = std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments);
        if constexpr (⒮::template 𝔦𝔰_𝔤𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>>) {
          segment = 𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(rm.segment));
          if (𝐢𝐧𝐭₈(rm.base) >= 8) {
            reg_operand_byte or_eq 0b0'0'1'00000;  // NOLINT(hicpp-signed-bitwise)
          }
          if (𝐢𝐧𝐭₈(rm.index) >= 8) {
            reg_operand_byte or_eq 0b0'1'0'00000;  // NOLINT(hicpp-signed-bitwise)
          }
        } else if constexpr (⒮::template 𝔦𝔰_𝔳𝔢𝔠𝔱𝔬𝔯_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>>) {
          segment = 𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(rm.segment));
          if (𝐢𝐧𝐭₈(rm.base) >= 8) {
            reg_operand_byte or_eq 0b0'0'1'00000;  // NOLINT(hicpp-signed-bitwise)
          }
          static_assert(
              std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm.index)>, typename ⒮::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
              std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm.index)>, typename ⒮::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
              std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm.index)>, typename ⒮::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>);
          // The same V̅′ bit is used to extend VSIB's Index and 𝔳𝔢𝔵-encoded operand.
          // We have no idea how to combine these because currently there are no instructions which have both.
          static_assert(not 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔳𝔢𝔵_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>);
          reg_operand_byte or_eq (𝐢𝐧𝐭₈(rm.index) bitand 0b000'01000) << 3;  // NOLINT(hicpp-signed-bitwise)
          if constexpr (𝐛𝐨𝐨𝐥(⒮::𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰)) {
            if (𝐢𝐧𝐭₈(rm.index) bitand 0b000'10000) {  // NOLINT(hicpp-signed-bitwise)
              mask_operand_byte = 0b0'00'0'1'000;
            }
          }
        } else {
          // There are no 𝔳𝔢𝔵-encoded instructions with these operands so it's not clean how should we treat them.
          static_assert(
              not std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::template 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<8>> and
              not std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> and
              not std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> and
              not std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐦𝐦𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>);
          // If we have 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, or 𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 then we encode two bits in reg_operand_byte - in X̅B̅ bits.
          if constexpr ((std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
                         std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
                         std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>)) {
            reg_operand_byte or_eq (𝐢𝐧𝐭₈(rm) bitand 0b000'11000) << 2;  // NOLINT(hicpp-signed-bitwise)
          } else if (𝐢𝐧𝐭₈(rm) >= 8) {
            reg_operand_byte or_eq 0b0'0'1'00000;  // NOLINT(hicpp-signed-bitwise)
          }
        }
      }
      if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔪𝔞𝔰𝔨_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
        mask_operand_byte or_eq 𝐢𝐧𝐭₈(std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔪𝔞𝔰𝔨_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments));  // NOLINT(hicpp-signed-bitwise)
      }
      if (not segment)
        𝖞𝖆𝖈𝖊_𝖑𝖎𝖐𝖊𝖑𝖞 {
          return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
              get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
                  𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
                  std::array{
                      𝐮𝐢𝐧𝐭₈(0x62),
                      𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[1] xor reg_operand_byte),
                      𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[2] xor vex_operand_byte),
                      𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[3] xor mask_operand_byte),
                      𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0])})),
              arguments);
        }
      return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
          get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
              std::array{𝐮𝐢𝐧𝐭₈(segment)},
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
              std::array{
                  𝐮𝐢𝐧𝐭₈(0x62),
                  𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[1] xor reg_operand_byte),
                  𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[2] xor vex_operand_byte),
                  𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[3] xor mask_operand_byte),
                  𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0])})),
          arguments);
    }
  }

  template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸, typename... 𝓣>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝑒𝑚𝑖𝑡_𝑙𝑜𝑛𝑔𝑣𝑒𝑥_𝑜𝑟_𝑥𝑜𝑝_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛(const std::tuple<𝓣...>& arguments) {
    // Note: technically 𝔳𝔢𝔵-encoded instructions and 𝔵𝔬𝔭-encoded instructions are not the same, but the only difference is
    // possibility of “short encoding” of certain 𝔳𝔢𝔵-encoded instructions. 𝑒𝑚𝑖𝑡_𝑣𝑒𝑥_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛 handles that case and then
    // calls 𝑒𝑚𝑖𝑡_𝑙𝑜𝑛𝑔𝑣𝑒𝑥_𝑜𝑟_𝑥𝑜𝑝_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛 to handle common case.
    static_assert(
        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔳𝔢𝔵_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸> or
        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔦𝔰_𝔵𝔬𝔭_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>);
    static_assert(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() == 3);
    static_assert(
        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0] == 0xc4 or  // 𝔳𝔢𝔵
        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0] == 0x8f);   // 𝔵𝔬𝔭
    static_assert(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() == 1);
    if constexpr (
        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔬𝔭𝔠𝔬𝔡𝔢_𝔦𝔫_𝔪𝔬𝔡𝔯𝔪<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸> or
        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔬𝔭𝔠𝔬𝔡𝔢_𝔦𝔫_𝔦𝔪𝔪<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
      static_assert(
          𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔢𝔵𝔱𝔢𝔫𝔰𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() ==
          𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰.size());
    } else {
      static_assert(
          𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() +
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>.size() ==
          𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔬𝔭𝔠𝔬𝔡𝔢𝔰.size());
    }
    // We need to handle R̅, X̅, or B̅ bits only in 64-bit mode.
    if constexpr (⒮::𝔵86_𝔪𝔬𝔡𝔢 <= 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32) {
      𝐮𝐢𝐧𝐭₈ segment = 0;
      𝐮𝐢𝐧𝐭₈ vex_operand_byte = 0;
      if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
        auto rm = std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments);
        if constexpr (⒮::template 𝔦𝔰_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>>) {
          segment = 𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(rm.segment));
        }
      }
      if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔳𝔢𝔵_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
        vex_operand_byte = 𝐢𝐧𝐭₈(std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔳𝔢𝔵_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments)) << 3;  // NOLINT(hicpp-signed-bitwise)
      }
      if (not segment)
        𝖞𝖆𝖈𝖊_𝖑𝖎𝖐𝖊𝖑𝖞 {
          return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
              get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
                  𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
                  std::array{
                      𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0]),
                      𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[1]),
                      𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[2] xor vex_operand_byte),
                      𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0])})),
              arguments);
        }
      return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
          get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
              std::array{𝐮𝐢𝐧𝐭₈(segment)},
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
              std::array{
                  𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0]),
                  𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[1]),
                  𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[2] xor vex_operand_byte),
                  𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0])})),
          arguments);
    } else if constexpr (⒮::𝔵86_𝔪𝔬𝔡𝔢 >= 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32) {
      𝐮𝐢𝐧𝐭₈ segment = 0;
      𝐮𝐢𝐧𝐭₈ reg_operand_byte = 0;
      𝐮𝐢𝐧𝐭₈ vex_operand_byte = 0;
      if (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔢𝔤_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
        auto reg = std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔯𝔢𝔤_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments);
        // If we have 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, or 𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 then we must ensure that we are not dealing with ₓ𝔪𝔪15-ₓ𝔪𝔪31.
        if constexpr (
            𝐛𝐨𝐨𝐥(⒮::𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰) and (std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(reg)>, typename ⒮::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
                                                 std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(reg)>, typename ⒮::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
                                                 std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(reg)>, typename ⒮::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>)) {
          𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖇𝖊𝖑𝖔𝖜_𝖔𝖗_𝖊𝖖𝖚𝖆𝖑(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, reg, ⒮::𝔵𝔪𝔪15);
        }
        reg_operand_byte = (𝐢𝐧𝐭₈(reg) bitand 0b0000'1000) << 4;  // NOLINT(hicpp-signed-bitwise)
      }
      if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔳𝔢𝔵_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
        auto vex = std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔳𝔢𝔵_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments);
        vex_operand_byte = 𝐢𝐧𝐭₈(vex) << 3;  // NOLINT(hicpp-signed-bitwise)
        // If we have 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, or 𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 then we must ensure that we are not dealing with ₓ𝔪𝔪15-ₓ𝔪𝔪31.
        if constexpr (
            𝐛𝐨𝐨𝐥(⒮::𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰) and (std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(vex)>, typename ⒮::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
                                                 std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(vex)>, typename ⒮::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
                                                 std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(vex)>, typename ⒮::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>)) {
          𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖇𝖊𝖑𝖔𝖜_𝖔𝖗_𝖊𝖖𝖚𝖆𝖑(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, vex, ⒮::𝔵𝔪𝔪15);
        }
      }
      if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
        auto rm = std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments);
        if constexpr (⒮::template 𝔦𝔰_𝔤𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>>) {
          segment = 𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(rm.segment));
          if (𝐢𝐧𝐭₈(rm.base) >= 8) {
            reg_operand_byte or_eq 0b0'0'1'00000;  // NOLINT(hicpp-signed-bitwise)
          }
          if (𝐢𝐧𝐭₈(rm.index) >= 8) {
            reg_operand_byte or_eq 0b0'1'0'00000;  // NOLINT(hicpp-signed-bitwise)
          }
        } else if constexpr (⒮::template 𝔦𝔰_𝔳𝔢𝔠𝔱𝔬𝔯_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>>) {
          segment = 𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(rm.segment));
          if (𝐢𝐧𝐭₈(rm.base) >= 8) {
            reg_operand_byte or_eq 0b0'0'1'00000;  // NOLINT(hicpp-signed-bitwise)
          }
          static_assert(
              std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm.index)>, typename ⒮::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
              std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm.index)>, typename ⒮::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
              std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm.index)>, typename ⒮::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>);
          if constexpr (𝐛𝐨𝐨𝐥(⒮::𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰)) {
            𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖇𝖊𝖑𝖔𝖜_𝖔𝖗_𝖊𝖖𝖚𝖆𝖑(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, rm.index, ⒮::𝔵𝔪𝔪15);
          }
          reg_operand_byte or_eq (𝐢𝐧𝐭₈(rm.index) bitand 0b0000'1000) << 3;  // NOLINT(hicpp-signed-bitwise)
        } else {
          // There are no 𝔳𝔢𝔵-encoded instructions with these operands so it's not clean how should we treat them.
          static_assert(
              not std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::template 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<8>> and
              not std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> and
              not std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> and
              not std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐦𝐦𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>);
          // If we have 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, or 𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 then we must ensure that we are not dealing with ₓ𝔪𝔪15-ₓ𝔪𝔪31.
          if constexpr ((std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
                         std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
                         std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>)) {
            if constexpr (𝐛𝐨𝐨𝐥(⒮::𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰)) {
              𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖇𝖊𝖑𝖔𝖜_𝖔𝖗_𝖊𝖖𝖚𝖆𝖑(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, rm, ⒮::𝔵𝔪𝔪15);
            }
            reg_operand_byte or_eq (𝐢𝐧𝐭₈(rm) bitand 0b0000'1000) << 2;  // NOLINT(hicpp-signed-bitwise)
          } else if (𝐢𝐧𝐭₈(rm) >= 8) {
            reg_operand_byte or_eq 0b0'0'1'00000;  // NOLINT(hicpp-signed-bitwise)
          }
        }
      }
      if (not segment)
        𝖞𝖆𝖈𝖊_𝖑𝖎𝖐𝖊𝖑𝖞 {
          return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
              get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
                  𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
                  std::array{
                      𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0]),
                      𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[1] xor reg_operand_byte),
                      𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[2] xor vex_operand_byte),
                      𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0])})),
              arguments);
        }
      return 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(
          get_assembler().𝑒𝑚𝑖𝑡ₒₚ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
              std::array{𝐮𝐢𝐧𝐭₈(segment)},
              𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔩𝔢𝔤𝔞𝔠𝔶_𝔭𝔯𝔢𝔣𝔦𝔵𝔢𝔰<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>,
              std::array{
                  𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0]),
                  𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[1] xor reg_operand_byte),
                  𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔳𝔢𝔵_𝔬𝔯_𝔵𝔬𝔭_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[2] xor vex_operand_byte),
                  𝐮𝐢𝐧𝐭₈(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔭𝔯𝔬𝔭𝔢𝔯<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[0])})),
          arguments);
    }
  }

  template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸, typename... 𝓣>
  [[nodiscard]] 𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑟𝑒𝑥(const std::tuple<𝓣...>& arguments) const -> 𝐮𝐢𝐧𝐭₈ {
    static_assert(⒮::𝔵86_𝔪𝔬𝔡𝔢 >= 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32);
    𝐮𝐢𝐧𝐭₈ rex = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔯𝔢𝔵_𝔭𝔯𝔢𝔣𝔦𝔵<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>;
    if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔢𝔤_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
      auto reg = std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔯𝔢𝔤_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments);
      if constexpr (std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(reg)>, typename ⒮::template 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<8>>) {
        // Note: we neex ℜ𝔈𝔛 not only for 𝔯8𝔟 to 𝔯15𝔟, but also for 𝔰𝔭𝔩/𝔟𝔭𝔩/𝔰𝔦𝔩/𝔡𝔦𝔩.
        if (𝐢𝐧𝐭₈(reg) >= 4) {
          // But only 3th bit is encoded in ℜ𝔈𝔛.ℜ.
          rex or_eq 𝐮𝐢𝐧𝐭₈(𝐮𝐢𝐧𝐭₈(0b01'000'000) bitor ((𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(reg)) bitand 0b1000) >> 1));  // NOLINT(hicpp-signed-bitwise)
        }
      } else if constexpr (
          not std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(reg)>, typename ⒮::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> and
          not std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(reg)>, typename ⒮::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> and
          not std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(reg)>, typename ⒮::𝐦𝐦𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>) {
        // Most register types encode additional bit in ℜ𝔈𝔛.ℜ, but not 𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, or 𝐦𝐦𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫.
        if (𝐢𝐧𝐭₈(reg) >= 8) {
          rex or_eq 𝐮𝐢𝐧𝐭₈(𝐮𝐢𝐧𝐭₈(0b01'000'000) bitor ((𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(reg)) bitand 0b1000) >> 1));  // NOLINT(hicpp-signed-bitwise)
        }
        // If we have 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, or 𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 then we must ensure that we are not dealing with ₓ𝔪𝔪15-ₓ𝔪𝔪31.
        if constexpr (
            ⒮::𝔵86_𝔪𝔬𝔡𝔢 >= 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32 and 𝐛𝐨𝐨𝐥(⒮::𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰) and
            (std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(reg)>, typename ⒮::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
             std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(reg)>, typename ⒮::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
             std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(reg)>, typename ⒮::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>)) {
          𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖇𝖊𝖑𝖔𝖜_𝖔𝖗_𝖊𝖖𝖚𝖆𝖑(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, reg, ⒮::𝔵𝔪𝔪15);
        }
      }
    }
    if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
      auto rm = std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments);
      if constexpr (⒮::template 𝔦𝔰_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>>) {
        static_assert(⒮::template 𝔦𝔰_𝔤𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>>);
        // Additional bit of base register number is encoded as ℜ𝔈𝔛.𝔅.  Note: 𝐢𝐧𝐭₈(𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯) == -1 which is less than 8.
        if (𝐢𝐧𝐭₈(rm.base) >= 8) {
          rex or_eq 𝐮𝐢𝐧𝐭₈(𝐮𝐢𝐧𝐭₈(0b01'000'000) bitor ((𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(rm.base)) bitand 0b1000) >> 3));  // NOLINT(hicpp-signed-bitwise)
        }
        // Additional bit of index register number is encoded as ℜ𝔈𝔛.𝔛.  Note: 𝐢𝐧𝐭₈(𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯) == -1 which is less than 8.
        if (𝐢𝐧𝐭₈(rm.index) >= 8) {
          rex or_eq 𝐮𝐢𝐧𝐭₈(𝐮𝐢𝐧𝐭₈(0b01'000'000) bitor ((𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(rm.index)) bitand 0b1000) >> 2));  // NOLINT(hicpp-signed-bitwise)
        }
      } else if constexpr (std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::template 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<8>>) {
        // Note: we neex ℜ𝔈𝔛 not only for 𝔯8𝔟 to 𝔯15𝔟, but also for 𝔰𝔭𝔩/𝔟𝔭𝔩/𝔰𝔦𝔩/𝔡𝔦𝔩.
        if (𝐢𝐧𝐭₈(rm) >= 4) {
          // But only 3th bit is encoded in ℜ𝔈𝔛.𝔅.
          rex or_eq 𝐮𝐢𝐧𝐭₈(𝐮𝐢𝐧𝐭₈(0b01'000'000) bitor ((𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(rm)) bitand 0b1000) >> 3));  // NOLINT(hicpp-signed-bitwise)
        }
      } else if constexpr (
          not std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> and
          not std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> and
          not std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐦𝐦𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>) {
        // Most register types encode additional bit in ℜ𝔈𝔛.𝔅, but not 𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝐱𝟖𝟕_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, or 𝐦𝐦𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫.
        if (𝐢𝐧𝐭₈(rm) >= 8) {
          rex or_eq 𝐮𝐢𝐧𝐭₈(𝐮𝐢𝐧𝐭₈(0b01'000'000) bitor ((𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(rm)) bitand 0b1000) >> 3));  // NOLINT(hicpp-signed-bitwise)
        }
        // If we have 𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, 𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫, or 𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 then we must ensure that we are not dealing with ₓ𝔪𝔪15-ₓ𝔪𝔪31.
        if constexpr (
            ⒮::𝔵86_𝔪𝔬𝔡𝔢 >= 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32 and 𝐛𝐨𝐨𝐥(⒮::𝔞𝔳𝔵512_𝔢𝔵𝔱𝔯𝔞_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯𝔰) and
            (std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐱𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
             std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐲𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> or
             std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::𝐳𝐦𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>)) {
          𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖇𝖊𝖑𝖔𝖜_𝖔𝖗_𝖊𝖖𝖚𝖆𝖑(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, rm, ⒮::𝔵𝔪𝔪15);
        }
      }
    }
    // If sanity_checks are enabled then we must verify that ℜ𝔈𝔛 is not combined with 𝔞𝔥/𝔠𝔥/𝔡𝔥/𝔟𝔥.
    if constexpr (𝓸𝓹𝓽𝓲𝓸𝓷𝓼->sanity_checks) {
      // If ℜ𝔈𝔛 is not present then there are nothing to check.
      if (not rex) {
        return rex;
      }
      if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
        auto reg = std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments);
        if constexpr (std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(reg)>, typename ⒮::template 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<8>>) {
          𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖆𝖇𝖔𝖛𝖊_𝖔𝖗_𝖊𝖖𝖚𝖆𝖑(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝐢𝐧𝐭₈(reg), 0);
        }
      }
      if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
        auto rm = std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments);
        if constexpr (std::is_same_v<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(rm)>, typename ⒮::template 𝒈𝒑_𝒓𝒆𝒈𝒊𝒔𝒕𝒆𝒓<8>>) {
          𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖆𝖇𝖔𝖛𝖊_𝖔𝖗_𝖊𝖖𝖚𝖆𝖑(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝐢𝐧𝐭₈(rm), 0);
        }
      }
    }
    return rex;
  }

  template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸, typename 𝓮𝓶𝓲𝓽𝓽𝓮𝓻, typename... 𝓣>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝑒𝑚𝑖𝑡_𝑝𝑜𝑠𝑡_𝑜𝑝𝑐𝑜𝑑𝑒_𝑖𝑛𝑠𝑡𝑟𝑢𝑐𝑡𝑖𝑜𝑛_𝑝𝑎𝑟𝑡(
      𝓮𝓶𝓲𝓽𝓽𝓮𝓻&& emitter,
      const std::tuple<𝓣...>& arguments) {
    if constexpr (
        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔢𝔤_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸> or
        𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
      if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔦𝔪𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
        static_assert(not 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔬𝔭𝔠𝔬𝔡𝔢_𝔦𝔫_𝔦𝔪𝔪<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>);
        if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔦𝔪𝔪2_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
          return 𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑠𝑖𝑏_𝑑𝑖𝑠𝑝<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(emitter, arguments)
              .𝑒𝑚𝑖𝑡ᵢₘₘ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
                  {𝑖𝑚𝑚𝑒𝑑𝑖𝑎𝑡𝑒_𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔦𝔪𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments)),
                   𝑖𝑚𝑚𝑒𝑑𝑖𝑎𝑡𝑒_𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔦𝔪𝔪2_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>)}));
        } else {
          return 𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑠𝑖𝑏_𝑑𝑖𝑠𝑝<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(emitter, arguments)
              .𝑒𝑚𝑖𝑡ᵢₘₘ(𝑖𝑚𝑚𝑒𝑑𝑖𝑎𝑡𝑒_𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔦𝔪𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments)));
        }
      } else if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔬𝔭𝔠𝔬𝔡𝔢_𝔦𝔫_𝔦𝔪𝔪<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
        return 𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑠𝑖𝑏_𝑑𝑖𝑠𝑝<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(emitter, arguments)
            .𝑒𝑚𝑖𝑡ᵢₘₘ(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔢𝔵𝔱𝔢𝔫𝔰𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>);
      } else {
        return 𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑠𝑖𝑏_𝑑𝑖𝑠𝑝<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(emitter, arguments);
      }
    } else if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔦𝔪𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
      static_assert(not 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔬𝔭𝔠𝔬𝔡𝔢_𝔦𝔫_𝔦𝔪𝔪<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>);
      if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔦𝔪𝔪2_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
        return emitter.𝑒𝑚𝑖𝑡ᵢₘₘ(𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(
            {𝑖𝑚𝑚𝑒𝑑𝑖𝑎𝑡𝑒_𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔦𝔪𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments)),
             𝑖𝑚𝑚𝑒𝑑𝑖𝑎𝑡𝑒_𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔦𝔪𝔪2_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>)}));
      }
      return emitter.𝑒𝑚𝑖𝑡ᵢₘₘ(𝑖𝑚𝑚𝑒𝑑𝑖𝑎𝑡𝑒_𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔦𝔪𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments)));
    } else if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔬𝔭𝔠𝔬𝔡𝔢_𝔦𝔫_𝔦𝔪𝔪<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
      return emitter.𝑒𝑚𝑖𝑡ᵢₘₘ(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔢𝔵𝔱𝔢𝔫𝔰𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>);
    } else {
      return emitter;
    }
  }

  template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸, typename 𝓮𝓶𝓲𝓽𝓽𝓮𝓻, typename... 𝓣>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑠𝑖𝑏_𝑑𝑖𝑠𝑝(𝓮𝓶𝓲𝓽𝓽𝓮𝓻&& emitter, const std::tuple<𝓣...>& arguments) {
    𝐮𝐢𝐧𝐭₈ reg = 𝑔𝑒𝑡_𝑚𝑜𝑑𝑟𝑚_𝑟𝑒𝑔<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(arguments);
    if constexpr (not 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔪𝔢𝔪𝔬𝔯𝔶_𝔯𝔢𝔣𝔢𝔯𝔢𝔫𝔠𝔢<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
      return 𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑟𝑒𝑔𝑖𝑠𝑡𝑒𝑟𝑠(emitter, reg, std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments));
    } else {
      // Memory reference always goes into 𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡 (except with string instructions and xlat).
      const auto& address = std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔯𝔪_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments);
      if constexpr (𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔴𝔦𝔡𝔱𝔥 == 16) {
        return 𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑚𝑒𝑚𝑜𝑟𝑦_16𝑏𝑖𝑡_𝑎𝑑𝑑𝑟𝑒𝑠𝑠<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(emitter, reg, address);
      } else if constexpr (𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔴𝔦𝔡𝔱𝔥 == 32 or 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔞𝔡𝔡𝔯𝔢𝔰𝔰_𝔴𝔦𝔡𝔱𝔥 == 64) {
        static_assert(std::is_same_v<decltype(address.disp), const 𝐢𝐧𝐭₃₂>);
        if constexpr (⒮::𝔵86_𝔪𝔬𝔡𝔢 == 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32) {
          if constexpr (⒮::template 𝔦𝔰_ₓ𝔦𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(address)>>) {
            return 𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑚𝑒𝑚𝑜𝑟𝑦_ₓ𝑖𝑝_𝑎𝑑𝑑𝑟𝑒𝑠𝑠(emitter, reg, address);
          }
        }
        if constexpr (⒮::template 𝔦𝔰_𝔳𝔢𝔠𝔱𝔬𝔯_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(address)>>) {
          return 𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑚𝑒𝑚𝑜𝑟𝑦_𝑣𝑒𝑐𝑡𝑜𝑟_𝑎𝑑𝑑𝑟𝑒𝑠𝑠<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(emitter, reg, address);
        } else if constexpr (⒮::template 𝔦𝔰_𝔤𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(address)>>) {
          return 𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑚𝑒𝑚𝑜𝑟𝑦_32𝑏𝑖𝑡_𝑜𝑟_64𝑏𝑖𝑡_𝑎𝑑𝑑𝑟𝑒𝑠𝑠<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(emitter, reg, address);
        }
      } else {
        static_assert(𝒇𝒂𝒍𝒔𝒆<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>);
      }
    }
  }

  template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸, typename... 𝓣>
  [[nodiscard]] 𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑔𝑒𝑡_𝑚𝑜𝑑𝑟𝑚_𝑟𝑒𝑔(const std::tuple<𝓣...>& arguments) -> 𝐮𝐢𝐧𝐭₈ {
    if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔯𝔢𝔤_𝔬𝔭𝔢𝔯𝔞𝔫𝔡<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
      auto reg = 𝐢𝐧𝐭₈(std::get<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸::𝔯𝔢𝔤_𝔬𝔭𝔢𝔯𝔞𝔫𝔡>(arguments));
      if constexpr (std::is_same_v<decltype(reg), typename ⒮::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>) {
        // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-constant-array-index,hicpp-signed-bitwise)
        return 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔰𝔢𝔤𝔪𝔢𝔫𝔱_𝔱𝔬_𝔯𝔢𝔤₈₀₃₈₆[reg bitand 0xb000'11111];  // Segment register must be remapped for use as
                                                                               // reg.
      } else if constexpr (⒮::𝔵86_𝔪𝔬𝔡𝔢 == 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32) {
        // We can only have more than 8 registers in 𝔵86_64 mode.
        return 𝐮𝐢𝐧𝐭₈(reg bitand 0b00'000'111);  // NOLINT(hicpp-signed-bitwise)
      } else {
        return 𝐮𝐢𝐧𝐭₈(reg);  // legacy or compatibility mode, not segment - just return register number.
      }
    } else {
      // If we don't have 𝔯𝔢𝔤 operand then reg encodes an opcode extension.
      return 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔬𝔭𝔠𝔬𝔡𝔢_𝔢𝔵𝔱𝔢𝔫𝔰𝔦𝔬𝔫<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>[1];
    }
  }

  template <typename 𝓮𝓶𝓲𝓽𝓽𝓮𝓻, typename 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑟𝑒𝑔𝑖𝑠𝑡𝑒𝑟𝑠(𝓮𝓶𝓲𝓽𝓽𝓮𝓻&& emitter, 𝐮𝐢𝐧𝐭₈ reg, 𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻 rm_register) {
    static_assert(not std::is_same_v<𝓻𝓮𝓰𝓲𝓼𝓽𝓮𝓻, typename ⒮::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>);
    𝐮𝐢𝐧𝐭₈ rm = 𝐢𝐧𝐭₈(rm_register);
    // If there are no memory access then rm just carries low 3 bits of register number.
    if constexpr (⒮::𝔵86_𝔪𝔬𝔡𝔢 == 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32) {
      // We can only have more than 8 registers in 𝔵86_64 mode.
      rm &= 0b00'000'111;  // NOLINT(hicpp-signed-bitwise)
    }
    return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{𝐮𝐢𝐧𝐭₈(0b11'000'000 bitor (reg << 3) bitor (rm << 0))});  // NOLINT(hicpp-signed-bitwise)
  }

  template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸, typename 𝓮𝓶𝓲𝓽𝓽𝓮𝓻, typename 𝓪𝓭𝓭𝓻𝓮𝓼𝓼>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑚𝑒𝑚𝑜𝑟𝑦_16𝑏𝑖𝑡_𝑎𝑑𝑑𝑟𝑒𝑠𝑠(𝓮𝓶𝓲𝓽𝓽𝓮𝓻&& emitter, 𝐮𝐢𝐧𝐭₈ reg, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼 address) {
    // If we have 16-bit address then only two registers could be used as base: 3 (𝔟𝔵) and 4 (𝔟𝔭). And two registers could be
    // used as index: 6 (𝔰𝔦) and 7(𝔡𝔦). Also both base and index can be absent. We can use the following value to easily
    // distinguish all 9 cases:
    // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-constant-array-index)
    𝐮𝐢𝐧𝐭₈ rm = 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔯𝔪_𝔰𝔢𝔩𝔢𝔠𝔱𝔬𝔯_𝔱𝔬_𝔯𝔪₈₀₈₆  // NOLINTNEXTLINE(hicpp-signed-bitwise)
        [(𝐢𝐧𝐭₈(address.base) bitand 0b0000'1010) bitor (𝐢𝐧𝐭₈(address.index) bitand 0b0001'0101)];
    static_assert(std::is_same_v<decltype(address.disp), const 𝐢𝐧𝐭₁₆>);
    // rm is 6 both when only 𝔟𝔭 is used without index and when absolute addressing is used. Hadle absolute addressing first.
    if ((rm == 0b0000'0110) and (𝐢𝐧𝐭₈(address.base) != 0b0000'0101))
      𝖞𝖆𝖈𝖊_𝖚𝖓𝖑𝖎𝖐𝖊𝖑𝖞 {
        return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(  // NOLINTNEXTLINE(hicpp-signed-bitwise)
            std::array{𝐮𝐢𝐧𝐭₈(0b00'000'000 bitor (reg << 3) bitor rm), 𝐮𝐢𝐧𝐭₈(address.disp), 𝐮𝐢𝐧𝐭₈(address.disp >> 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢)});
      }
    // Note: if only 𝔟𝔭 is used then we shouldn't try to use form without displacement.
    if ((rm != 0b0000'0110) and address.disp == 0) {  // NOLINTNEXTLINE(hicpp-signed-bitwise)
      return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{𝐮𝐢𝐧𝐭₈((reg << 3) bitor rm)});
    }
    if (𝑣𝑎𝑙𝑖𝑑_𝑑𝑖𝑠𝑝8<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(address.disp)) {  // NOLINTNEXTLINE(hicpp-signed-bitwise)
      return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{𝐮𝐢𝐧𝐭₈(0b01'000'000 bitor (reg << 3) bitor rm), 𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(address.disp))});
    }
    return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(  // NOLINTNEXTLINE(hicpp-signed-bitwise)
        std::array{𝐮𝐢𝐧𝐭₈(0b10'000'000 bitor (reg << 3) bitor rm), 𝐮𝐢𝐧𝐭₈(address.disp), 𝐮𝐢𝐧𝐭₈(address.disp >> 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢)});
  }

  template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸, typename 𝓮𝓶𝓲𝓽𝓽𝓮𝓻, typename 𝓪𝓭𝓭𝓻𝓮𝓼𝓼>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto)
  𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑚𝑒𝑚𝑜𝑟𝑦_32𝑏𝑖𝑡_𝑜𝑟_64𝑏𝑖𝑡_𝑎𝑑𝑑𝑟𝑒𝑠𝑠(𝓮𝓶𝓲𝓽𝓽𝓮𝓻&& emitter, 𝐮𝐢𝐧𝐭₈ reg, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼 address) {
    static_assert(⒮::template 𝔦𝔰_𝔤𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(address)>>);
    static_assert(std::is_same_v<decltype(address.disp), const 𝐢𝐧𝐭₃₂>);
    if (address.base <= ⒮::𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯 and address.index == ⒮::𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯)
      𝖞𝖆𝖈𝖊_𝖚𝖓𝖑𝖎𝖐𝖊𝖑𝖞 { return 𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑚𝑒𝑚𝑜𝑟𝑦_32𝑏𝑖𝑡_𝑜𝑟_64𝑏𝑖𝑡_𝑎𝑏𝑠𝑜𝑙𝑢𝑡𝑒_𝑎𝑑𝑑𝑟𝑒𝑠𝑠(emitter, reg, address); }
    if (address.index == ⒮::𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯)
      𝖞𝖆𝖈𝖊_𝖑𝖎𝖐𝖊𝖑𝖞 { return 𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑚𝑒𝑚𝑜𝑟𝑦_32𝑏𝑖𝑡_𝑜𝑟_64𝑏𝑖𝑡_𝑏𝑎𝑠𝑒_𝑜𝑛𝑙𝑦_𝑎𝑑𝑑𝑟𝑒𝑠𝑠<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(emitter, reg, address); }
    return 𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑚𝑒𝑚𝑜𝑟𝑦_32𝑏𝑖𝑡_𝑜𝑟_64𝑏𝑖𝑡_𝑏𝑎𝑠𝑒_𝑎𝑛𝑑_𝑖𝑛𝑑𝑒𝑥_𝑎𝑑𝑑𝑟𝑒𝑠𝑠<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(emitter, reg, address);
  }

  template <typename 𝓮𝓶𝓲𝓽𝓽𝓮𝓻, typename 𝓪𝓭𝓭𝓻𝓮𝓼𝓼>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto)
  𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑚𝑒𝑚𝑜𝑟𝑦_32𝑏𝑖𝑡_𝑜𝑟_64𝑏𝑖𝑡_𝑎𝑏𝑠𝑜𝑙𝑢𝑡𝑒_𝑎𝑑𝑑𝑟𝑒𝑠𝑠(𝓮𝓶𝓲𝓽𝓽𝓮𝓻&& emitter, 𝐮𝐢𝐧𝐭₈ reg, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼 address) {
    static_assert(⒮::template 𝔦𝔰_𝔤𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(address)>>);
    static_assert(std::is_same_v<decltype(address.disp), const 𝐢𝐧𝐭₃₂>);
    if constexpr (⒮::𝔵86_𝔪𝔬𝔡𝔢 == 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32) {
      return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{
          𝐮𝐢𝐧𝐭₈(0b00'000'100 bitor (reg << 3)),  // NOLINT(hicpp-signed-bitwise)
          𝐮𝐢𝐧𝐭₈(0b00'100'101),
          𝐮𝐢𝐧𝐭₈(address.disp),
          𝐮𝐢𝐧𝐭₈(address.disp >> 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),        // NOLINT(hicpp-signed-bitwise)
          𝐮𝐢𝐧𝐭₈(address.disp >> 2 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),    // NOLINT(hicpp-signed-bitwise)
          𝐮𝐢𝐧𝐭₈(address.disp >> 3 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢)});  // NOLINT(hicpp-signed-bitwise)
    } else {
      return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{
          𝐮𝐢𝐧𝐭₈(0b00'000'101 bitor (reg << 3)),  // NOLINT(hicpp-signed-bitwise)
          𝐮𝐢𝐧𝐭₈(address.disp),
          𝐮𝐢𝐧𝐭₈(address.disp >> 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),        // NOLINT(hicpp-signed-bitwise)
          𝐮𝐢𝐧𝐭₈(address.disp >> 2 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),    // NOLINT(hicpp-signed-bitwise)
          𝐮𝐢𝐧𝐭₈(address.disp >> 3 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢)});  // NOLINT(hicpp-signed-bitwise)
    }
  }

  template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸, typename 𝓮𝓶𝓲𝓽𝓽𝓮𝓻, typename 𝓪𝓭𝓭𝓻𝓮𝓼𝓼>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto)
  𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑚𝑒𝑚𝑜𝑟𝑦_32𝑏𝑖𝑡_𝑜𝑟_64𝑏𝑖𝑡_𝑏𝑎𝑠𝑒_𝑜𝑛𝑙𝑦_𝑎𝑑𝑑𝑟𝑒𝑠𝑠(𝓮𝓶𝓲𝓽𝓽𝓮𝓻&& emitter, 𝐮𝐢𝐧𝐭₈ reg, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼 address) {
    static_assert(⒮::template 𝔦𝔰_𝔤𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(address)>>);
    static_assert(std::is_same_v<decltype(address.disp), const 𝐢𝐧𝐭₃₂>);
    𝐮𝐢𝐧𝐭₈ base = 𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(address.base));  // NOLINT(hicpp-use-auto,modernize-use-auto)
    if constexpr (⒮::𝔵86_𝔪𝔬𝔡𝔢 == 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32) {
      base &= 0b00'000'111;  // NOLINT(hicpp-signed-bitwise)
    }
    if (base != 0b101 and address.disp == 0) {
      if (base == 0b100)
        𝖞𝖆𝖈𝖊_𝖚𝖓𝖑𝖎𝖐𝖊𝖑𝖞 {
          // NOLINTNEXTLINE(hicpp-signed-bitwise)
          return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{𝐮𝐢𝐧𝐭₈(0b00'000'100 bitor (reg << 3)), 𝐮𝐢𝐧𝐭₈(0b00'100'100)});
        }
      // NOLINTNEXTLINE(hicpp-signed-bitwise)
      return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{𝐮𝐢𝐧𝐭₈(0b00'000'000 bitor (reg << 3) bitor base)});
    }
    if (𝑣𝑎𝑙𝑖𝑑_𝑑𝑖𝑠𝑝8<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(address.disp)) {
      if (base == 0b100)
        𝖞𝖆𝖈𝖊_𝖚𝖓𝖑𝖎𝖐𝖊𝖑𝖞 {
          // NOLINTNEXTLINE(hicpp-signed-bitwise)
          return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{𝐮𝐢𝐧𝐭₈(0b01'000'100 bitor (reg << 3)), 𝐮𝐢𝐧𝐭₈(0b00'100'100), 𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(address.disp))});
        }
      // NOLINTNEXTLINE(hicpp-signed-bitwise)
      return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{𝐮𝐢𝐧𝐭₈(0b01'000'000 bitor (reg << 3) bitor base), 𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(address.disp))});
    }
    if (base == 0b100)
      𝖞𝖆𝖈𝖊_𝖚𝖓𝖑𝖎𝖐𝖊𝖑𝖞 {
        // NOLINTNEXTLINE(hicpp-signed-bitwise)
        return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{
            𝐮𝐢𝐧𝐭₈(0b10'000'100 bitor (reg << 3)),
            𝐮𝐢𝐧𝐭₈(0b00'100'100),
            𝐮𝐢𝐧𝐭₈(address.disp),
            𝐮𝐢𝐧𝐭₈(address.disp >> 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),        // NOLINT(hicpp-signed-bitwise)
            𝐮𝐢𝐧𝐭₈(address.disp >> 2 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),    // NOLINT(hicpp-signed-bitwise)
            𝐮𝐢𝐧𝐭₈(address.disp >> 3 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢)});  // NOLINT(hicpp-signed-bitwise)
      }
    return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{
        𝐮𝐢𝐧𝐭₈(0b10'000'000 bitor (reg << 3) bitor base),  // NOLINT(hicpp-signed-bitwise)
        𝐮𝐢𝐧𝐭₈(address.disp),
        𝐮𝐢𝐧𝐭₈(address.disp >> 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),        // NOLINT(hicpp-signed-bitwise)
        𝐮𝐢𝐧𝐭₈(address.disp >> 2 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),    // NOLINT(hicpp-signed-bitwise)
        𝐮𝐢𝐧𝐭₈(address.disp >> 3 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢)});  // NOLINT(hicpp-signed-bitwise)
  }

  template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸, typename 𝓮𝓶𝓲𝓽𝓽𝓮𝓻, typename 𝓪𝓭𝓭𝓻𝓮𝓼𝓼>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto)
  𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑚𝑒𝑚𝑜𝑟𝑦_32𝑏𝑖𝑡_𝑜𝑟_64𝑏𝑖𝑡_𝑏𝑎𝑠𝑒_𝑎𝑛𝑑_𝑖𝑛𝑑𝑒𝑥_𝑎𝑑𝑑𝑟𝑒𝑠𝑠(𝓮𝓶𝓲𝓽𝓽𝓮𝓻&& emitter, 𝐮𝐢𝐧𝐭₈ reg, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼 address) {
    // Note: in legacy and compatibility modes this correctly handles the case when index is 𝔢𝔦𝔷/𝔯𝔦𝔷.
    𝐮𝐢𝐧𝐭₈ index = 𝐮𝐢𝐧𝐭₈(𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(address.index)) bitand 𝐮𝐢𝐧𝐭₈(0b00'000'111));  // NOLINT(hicpp-use-auto,modernize-use-auto)
    if (address.base <= ⒮::𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯)
      𝖞𝖆𝖈𝖊_𝖚𝖓𝖑𝖎𝖐𝖊𝖑𝖞 {
        return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{
            𝐮𝐢𝐧𝐭₈(0b00'000'100 bitor (reg << 3)),                                      // NOLINT(hicpp-signed-bitwise)
            𝐮𝐢𝐧𝐭₈(0b00'000'101 bitor (𝐮𝐢𝐧𝐭₈(address.scale) << 6) bitor (index << 3)),  // NOLINT(hicpp-signed-bitwise)
            𝐮𝐢𝐧𝐭₈(address.disp),
            𝐮𝐢𝐧𝐭₈(address.disp >> 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),        // NOLINT(hicpp-signed-bitwise)
            𝐮𝐢𝐧𝐭₈(address.disp >> 2 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),    // NOLINT(hicpp-signed-bitwise)
            𝐮𝐢𝐧𝐭₈(address.disp >> 3 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢)});  // NOLINT(hicpp-signed-bitwise)
      }
    𝐮𝐢𝐧𝐭₈ base = 𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(address.base));  // NOLINT(hicpp-use-auto,modernize-use-auto)
    if constexpr (⒮::𝔵86_𝔪𝔬𝔡𝔢 == 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32) {
      base &= 0b00'000'111;  // NOLINT(hicpp-signed-bitwise)
    }
    if (base != 0b101 and address.disp == 0) {
      return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{
          // NOLINTNEXTLINE(hicpp-signed-bitwise)
          𝐮𝐢𝐧𝐭₈(0b00'000'100 bitor (reg << 3)),
          // NOLINTNEXTLINE(hicpp-signed-bitwise)
          𝐮𝐢𝐧𝐭₈(0b00'000'000 bitor (𝐮𝐢𝐧𝐭₈(address.scale) << 6) bitor (index << 3) bitor (base << 0))});
    }
    if (𝑣𝑎𝑙𝑖𝑑_𝑑𝑖𝑠𝑝8<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(address.disp)) {
      return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{
          𝐮𝐢𝐧𝐭₈(0b01'000'100 bitor (reg << 3)),  // NOLINT(hicpp-signed-bitwise)
          𝐮𝐢𝐧𝐭₈(
              0b00'000'000 bitor (𝐮𝐢𝐧𝐭₈(address.scale) << 6) bitor (index << 3) bitor (base << 0)),  // NOLINT(hicpp-signed-bitwise)
          𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(address.disp))});
    }
    return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{
        𝐮𝐢𝐧𝐭₈(0b10'000'100 bitor (reg << 3)),                                                        // NOLINT(hicpp-signed-bitwise)
        𝐮𝐢𝐧𝐭₈(0b00'000'000 bitor (𝐮𝐢𝐧𝐭₈(address.scale) << 6) bitor (index << 3) bitor (base << 0)),  // NOLINT(hicpp-signed-bitwise)
        𝐮𝐢𝐧𝐭₈(address.disp),
        𝐮𝐢𝐧𝐭₈(address.disp >> 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),        // NOLINT(hicpp-signed-bitwise)
        𝐮𝐢𝐧𝐭₈(address.disp >> 2 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),    // NOLINT(hicpp-signed-bitwise)
        𝐮𝐢𝐧𝐭₈(address.disp >> 3 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢)});  // NOLINT(hicpp-signed-bitwise)
  }

  template <typename 𝓮𝓶𝓲𝓽𝓽𝓮𝓻, typename 𝓪𝓭𝓭𝓻𝓮𝓼𝓼>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑚𝑒𝑚𝑜𝑟𝑦_ₓ𝑖𝑝_𝑎𝑑𝑑𝑟𝑒𝑠𝑠(𝓮𝓶𝓲𝓽𝓽𝓮𝓻&& emitter, 𝐮𝐢𝐧𝐭₈ reg, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼 address) {
    static_assert(⒮::template 𝔦𝔰_ₓ𝔦𝔭_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(address)>>);
    static_assert(std::is_same_v<decltype(address.disp), 𝐢𝐧𝐭₃₂>);
    static_assert(⒮::𝔵86_𝔪𝔬𝔡𝔢 == 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32);
    return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{
        𝐮𝐢𝐧𝐭₈(0b00'000'101 bitor (reg << 3)),  // NOLINT(hicpp-signed-bitwise)
        𝐮𝐢𝐧𝐭₈(address.disp),
        𝐮𝐢𝐧𝐭₈(address.disp >> 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),        // NOLINT(hicpp-signed-bitwise)
        𝐮𝐢𝐧𝐭₈(address.disp >> 2 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),    // NOLINT(hicpp-signed-bitwise)
        𝐮𝐢𝐧𝐭₈(address.disp >> 3 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢)});  // NOLINT(hicpp-signed-bitwise)
  }

  template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸, typename 𝓮𝓶𝓲𝓽𝓽𝓮𝓻, typename 𝓪𝓭𝓭𝓻𝓮𝓼𝓼>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) 𝑒𝑚𝑖𝑡_𝑚𝑜𝑑𝑟𝑚_𝑚𝑒𝑚𝑜𝑟𝑦_𝑣𝑒𝑐𝑡𝑜𝑟_𝑎𝑑𝑑𝑟𝑒𝑠𝑠(𝓮𝓶𝓲𝓽𝓽𝓮𝓻&& emitter, 𝐮𝐢𝐧𝐭₈ reg, 𝓪𝓭𝓭𝓻𝓮𝓼𝓼 address) {
    static_assert(⒮::template 𝔦𝔰_𝔳𝔢𝔠𝔱𝔬𝔯_𝔞𝔡𝔡𝔯𝔢𝔰𝔰<𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(address)>>);
    static_assert(std::is_same_v<decltype(address.disp), 𝐢𝐧𝐭₃₂>);
    𝐮𝐢𝐧𝐭₈ index = 𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(address.index));  // NOLINT(hicpp-use-auto,modernize-use-auto)
    if constexpr (⒮::𝔵86_𝔪𝔬𝔡𝔢 == 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32) {
      // We can only have more than 8 registers in 𝔵86_64 mode.
      index &= 0b00'000'111;  // NOLINT(hicpp-signed-bitwise)
    }
    if (address.base == ⒮::𝔫𝔬_𝔯𝔢𝔤𝔦𝔰𝔱𝔢𝔯)
      𝖞𝖆𝖈𝖊_𝖚𝖓𝖑𝖎𝖐𝖊𝖑𝖞 {
        return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{
            𝐮𝐢𝐧𝐭₈(0b00'000'100 bitor (reg << 3)),                                      // NOLINT(hicpp-signed-bitwise)
            𝐮𝐢𝐧𝐭₈(0b00'000'101 bitor (𝐮𝐢𝐧𝐭₈(address.scale) << 6) bitor (index << 3)),  // NOLINT(hicpp-signed-bitwise)
            𝐮𝐢𝐧𝐭₈(address.disp),
            𝐮𝐢𝐧𝐭₈(address.disp >> 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),        // NOLINT(hicpp-signed-bitwise)
            𝐮𝐢𝐧𝐭₈(address.disp >> 2 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),    // NOLINT(hicpp-signed-bitwise)
            𝐮𝐢𝐧𝐭₈(address.disp >> 3 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢)});  // NOLINT(hicpp-signed-bitwise)
      }
    𝐮𝐢𝐧𝐭₈ base = 𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(address.base));  // NOLINT(hicpp-use-auto,modernize-use-auto)
    if constexpr (⒮::𝔵86_𝔪𝔬𝔡𝔢 == 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32) {
      // We can only have more than 8 registers in 𝔵86_64 mode.
      base &= 0b00'000'111;  // NOLINT(hicpp-signed-bitwise)
    }
    if (𝑣𝑎𝑙𝑖𝑑_𝑑𝑖𝑠𝑝8<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>(address.disp)) {
      return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{
          𝐮𝐢𝐧𝐭₈(0b10'000'100 bitor (reg << 3)),  // NOLINT(hicpp-signed-bitwise)
          𝐮𝐢𝐧𝐭₈(
              0b00'000'000 bitor (𝐮𝐢𝐧𝐭₈(address.scale) << 6) bitor (index << 3) bitor (base << 0)),  // NOLINT(hicpp-signed-bitwise)
          𝐮𝐢𝐧𝐭₈(𝐢𝐧𝐭₈(address.disp))});
    }
    return emitter.𝑒𝑚𝑖𝑡ₘᵣₘ(std::array{
        𝐮𝐢𝐧𝐭₈(0b10'000'100 bitor (reg << 3)),                                                        // NOLINT(hicpp-signed-bitwise)
        𝐮𝐢𝐧𝐭₈(0b00'000'000 bitor (𝐮𝐢𝐧𝐭₈(address.scale) << 6) bitor (index << 3) bitor (base << 0)),  // NOLINT(hicpp-signed-bitwise)
        𝐮𝐢𝐧𝐭₈(address.disp),
        𝐮𝐢𝐧𝐭₈(address.disp >> 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),        // NOLINT(hicpp-signed-bitwise)
        𝐮𝐢𝐧𝐭₈(address.disp >> 2 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),    // NOLINT(hicpp-signed-bitwise)
        𝐮𝐢𝐧𝐭₈(address.disp >> 3 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢)});  // NOLINT(hicpp-signed-bitwise)
  }

  template <typename 𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸, typename 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑣𝑎𝑙𝑖𝑑_𝑑𝑖𝑠𝑝8(𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮 disp) -> 𝐛𝐨𝐨𝐥 {
    if constexpr (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔥𝔞𝔰_𝔠𝔬𝔪𝔭𝔯𝔢𝔰𝔰𝔢𝔡_𝔡𝔦𝔰𝔭𝔩𝔞𝔠𝔢𝔪𝔢𝔫𝔱<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) {
      static_assert(
          𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔡𝔦𝔰𝔭8_𝔰𝔠𝔞𝔩𝔢<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸> > 1 and
          ((𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔡𝔦𝔰𝔭8_𝔰𝔠𝔞𝔩𝔢<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸> bitand
            (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔡𝔦𝔰𝔭8_𝔰𝔠𝔞𝔩𝔢<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸> - 1)) == 0));
      if (disp bitand (𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔡𝔦𝔰𝔭8_𝔰𝔠𝔞𝔩𝔢<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸> - 1))
        𝖞𝖆𝖈𝖊_𝖚𝖓𝖑𝖎𝖐𝖊𝖑𝖞 { return false; }
      return (
          ((std::numeric_limits<𝐢𝐧𝐭₈>::min() * 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔡𝔦𝔰𝔭8_𝔰𝔠𝔞𝔩𝔢<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>) <= disp) and
          (disp <= (std::numeric_limits<𝐢𝐧𝐭₈>::max() * 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔡𝔦𝔰𝔭8_𝔰𝔠𝔞𝔩𝔢<𝓲𝓷𝓼𝓽𝓻𝓾𝓬𝓽𝓲𝓸𝓷_𝓲𝓷𝓯𝓸>)));
    } else {
      return 𝐢𝐧𝐭₈(disp) == disp;
    }
  }

  template <typename 𝓲𝓶𝓶𝓮𝓭𝓲𝓪𝓽𝓮>
  [[nodiscard]] 𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 auto constexpr 𝑖𝑚𝑚𝑒𝑑𝑖𝑎𝑡𝑒_𝑡𝑜_𝑎𝑟𝑟𝑎𝑦(𝓲𝓶𝓶𝓮𝓭𝓲𝓪𝓽𝓮 immediate) -> std::array<𝐮𝐢𝐧𝐭₈, sizeof immediate> {
    static_assert(std::is_integral_v<𝓲𝓶𝓶𝓮𝓭𝓲𝓪𝓽𝓮>);
    std::make_unsigned<𝓲𝓶𝓶𝓮𝓭𝓲𝓪𝓽𝓮> unsigned_immediate = immediate;  // Make sure there are no surprses related to signedness.
    static_assert(sizeof immediate == sizeof unsigned_immediate);
    if constexpr (sizeof immediate == 1) {
      return std::array{𝐮𝐢𝐧𝐭₈(immediate)};
    } else if constexpr (sizeof immediate == 2) {
      return std::array{𝐮𝐢𝐧𝐭₈(immediate), 𝐮𝐢𝐧𝐭₈(immediate >> 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢)};
    } else if constexpr (sizeof immediate == 4) {
      return std::array{
          𝐮𝐢𝐧𝐭₈(immediate), 𝐮𝐢𝐧𝐭₈(immediate >> 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢), 𝐮𝐢𝐧𝐭₈(immediate >> 2 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢), 𝐮𝐢𝐧𝐭₈(immediate >> 3 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢)};
    } else if constexpr (sizeof immediate == 8) {
      return std::array{
          𝐮𝐢𝐧𝐭₈(immediate),
          𝐮𝐢𝐧𝐭₈(immediate >> 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),
          𝐮𝐢𝐧𝐭₈(immediate >> 2 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),
          𝐮𝐢𝐧𝐭₈(immediate >> 3 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),
          𝐮𝐢𝐧𝐭₈(immediate >> 4 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),
          𝐮𝐢𝐧𝐭₈(immediate >> 5 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),
          𝐮𝐢𝐧𝐭₈(immediate >> 6 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢),
          𝐮𝐢𝐧𝐭₈(immediate >> 7 * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢)};
    } else {
      std::array<𝐮𝐢𝐧𝐭₈, sizeof(𝓲𝓶𝓶𝓮𝓭𝓲𝓪𝓽𝓮)> result{};
      for (𝐬𝐢𝐳𝐞 counter = 0; counter < sizeof immediate; ++counter) {
        result[counter] = 𝐮𝐢𝐧𝐭₈(unsigned_immediate >> (counter * 𝔟𝔶𝔱𝔢𝔰𝔦𝔷𝔢));
      }
      return result;
    }
  }

  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 decltype(auto) constexpr get_assembler() noexcept { return *static_cast<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻*>(this); }
};

}  // namespace 𝘆𝗮𝗰𝗲::𝘅𝟴𝟲

#endif  // 𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_𝔛86_𝔅𝔜𝔗𝔈_𝔈𝔐ℑ𝔗_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_ℌ
