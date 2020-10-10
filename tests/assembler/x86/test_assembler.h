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

#ifndef 𝔜𝔄ℭ𝔈_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_𝔛86_𝔗𝔈𝔖𝔗_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_ℌ
#define 𝔜𝔄ℭ𝔈_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_𝔛86_𝔗𝔈𝔖𝔗_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_ℌ

#include "yace/foundation.h"

#include "yace/assembler/x86/byte_emit_assembler.h"

namespace 𝘆𝗮𝗰𝗲::𝘅𝟴𝟲 {

// Assembler made for testing.  Supplied “emit₈” function compares expected output to what assembler it generating.
// Since we are knowing all the data in advance it's much simpler than 𝒄𝒐𝒏𝒔𝒕𝒆𝒙𝒑𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓 which helps with compilation speed.
// Note: it's 𝗻𝗼𝘁 a descendant of 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓, instead it's function “expected_from” returns such assembler.
// This allows us to separate std::array container from “iterator” object.  Note that this doesn't work if we are dealing with
// constexprs.
template <
    typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻,
    auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼 = &::𝘆𝗮𝗰𝗲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔡𝔢𝔣𝔞𝔲𝔩𝔱,
    𝐜𝐨𝐧𝐬𝐭𝐞𝐱𝐩𝐫_𝐟𝐫𝐢𝐞𝐧𝐝𝐥𝐲 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂 = 𝔠𝔬𝔫𝔰𝔱𝔢𝔵𝔭𝔯_𝔲𝔫𝔣𝔯𝔦𝔢𝔫𝔡𝔩𝔶,
    𝐬𝐢𝐳𝐞 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 = std::numeric_limits<𝐬𝐢𝐳𝐞>::max()>
class 𝒃𝒂𝒔𝒊𝒄_𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓 {
  using ⒮ = 𝒃𝒂𝒔𝒊𝒄_𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>;
  // Since we are only doing checks it doesn't make much sense to work with them disabled.
  static_assert(𝓸𝓹𝓽𝓲𝓸𝓷𝓼->sanity_checks);

 public:
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto expected_from() noexcept {
    // Note: even with C++20 we couldn't automatically determine if we are used in constexpr context or not and then select the
    // appropriate type. We can check whether the proper version was used, though.
#ifdef __cpp_lib_is_constant_evaluated
    𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖊𝖖𝖚𝖆𝖑(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, std::is_constant_evaluated(), 𝐛𝐨𝐨𝐥(𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂));
#endif
    return typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::⒭{𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌};
  }

 protected:
  template <typename... 𝓽𝔂𝓹𝓮>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒃𝒂𝒔𝒊𝒄_𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓(𝓽𝔂𝓹𝓮... arg) : 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌{𝐮𝐢𝐧𝐭₈(arg)...} {
    // Note: we want to be able to pass negative values in some cases (which is convenient), but don't want to lose precision.
    𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖊𝖖𝖚𝖆𝖑_𝖋𝖔𝖑𝖉(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝐮𝐢𝐧𝐭₈(arg), std::make_unsigned_t<decltype(arg)>(arg));
  }
  class 𝐛𝐚𝐬𝐢𝐜_𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫 : public 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::⒭, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼> {
    using ⒮ = 𝐛𝐚𝐬𝐢𝐜_𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫;

   public:
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr operator 𝐛𝐨𝐨𝐥() noexcept { return 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌.size() == 𝗉𝗈𝗌𝗂𝗍𝗂𝗈𝗇; }

   protected:
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝐛𝐚𝐬𝐢𝐜_𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫(
        const std::array<𝐮𝐢𝐧𝐭₈, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>& expected_contents) noexcept
        : 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌{expected_contents}, 𝗉𝗈𝗌𝗂𝗍𝗂𝗈𝗇{0} {}
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr 𝐛𝐚𝐬𝐢𝐜_𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫(
        const std::array<𝐮𝐢𝐧𝐭₈, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>& expected_contents,
        𝐬𝐢𝐳𝐞 position)
        : 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌{expected_contents}, 𝗉𝗈𝗌𝗂𝗍𝗂𝗈𝗇{position} {}

   private:
    friend 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::⒭, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>;
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) emit₈(𝐮𝐢𝐧𝐭₈ value) {
      𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖇𝖊𝖑𝖔𝖜(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝗉𝗈𝗌𝗂𝗍𝗂𝗈𝗇, 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌.size());
      𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖊𝖖𝖚𝖆𝖑(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, value, 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌[𝗉𝗈𝗌𝗂𝗍𝗂𝗈𝗇]);
      return typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::⒭{𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌, 𝗉𝗈𝗌𝗂𝗍𝗂𝗈𝗇 + 1};
    }
    // Note: references are not allowed in costexpr expressions so we need to copy full array if we are dealing with constexpr.
#ifdef __clang__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wpadded"
#endif
    std::conditional_t<𝐛𝐨𝐨𝐥(𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂), const std::array<𝐮𝐢𝐧𝐭₈, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>, const std::array<𝐮𝐢𝐧𝐭₈, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>&>
        𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌;
    const 𝐬𝐢𝐳𝐞 𝗉𝗈𝗌𝗂𝗍𝗂𝗈𝗇;
#ifdef __clang__
#pragma clang diagnostic pop
#endif
  };

 private:
  const std::array<𝐮𝐢𝐧𝐭₈, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮> 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌;
};

// Similarly to std::span - version with size specified in runtime.
template <typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝐜𝐨𝐧𝐬𝐭𝐞𝐱𝐩𝐫_𝐟𝐫𝐢𝐞𝐧𝐝𝐥𝐲 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂>
class 𝒃𝒂𝒔𝒊𝒄_𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂, std::numeric_limits<𝐬𝐢𝐳𝐞>::max()> {
  using ⒮ = 𝒃𝒂𝒔𝒊𝒄_𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂, std::numeric_limits<𝐬𝐢𝐳𝐞>::max()>;
  // No support for constexpr version here.
  static_assert(𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂 == 𝔠𝔬𝔫𝔰𝔱𝔢𝔵𝔭𝔯_𝔲𝔫𝔣𝔯𝔦𝔢𝔫𝔡𝔩𝔶);
  // Since we are only doing checks it doesn't make much sense to work with them disabled.
  static_assert(𝓸𝓹𝓽𝓲𝓸𝓷𝓼->sanity_checks);

 public:
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto expected_from() noexcept {
#ifdef __cpp_lib_span
    return typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::⒭{𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌};
#else
    return typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::⒭{𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖻𝖾𝗀𝗂𝗇, 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖾𝗇𝖽};
#endif
  }

 protected:
  template <typename 𝓲𝓽𝓮𝓻𝓪𝓽𝓸𝓻>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr 𝒃𝒂𝒔𝒊𝒄_𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓(𝓲𝓽𝓮𝓻𝓪𝓽𝓸𝓻 first, 𝐬𝐢𝐳𝐞 count) noexcept
#ifdef __cpp_lib_span
      : 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌 {
    first, count
  }
#else
      : 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖻𝖾𝗀𝗂𝗇{first}, 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖾𝗇𝖽 {
    first + count
  }
#endif
  {}
  template <typename 𝓲𝓽𝓮𝓻𝓪𝓽𝓸𝓻, typename 𝓮𝓷𝓭>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr 𝒃𝒂𝒔𝒊𝒄_𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓(𝓲𝓽𝓮𝓻𝓪𝓽𝓸𝓻 first, 𝓮𝓷𝓭 last) noexcept
#ifdef __cpp_lib_span
      : 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌 {
    first, last
  }
#else
      : 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖻𝖾𝗀𝗂𝗇{first}, 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖾𝗇𝖽 {
    first + (last - first)
  }
#endif
  {}
  class 𝐛𝐚𝐬𝐢𝐜_𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫 : public 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::⒭, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼> {
    using ⒮ = 𝐛𝐚𝐬𝐢𝐜_𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫;

   public:
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr operator 𝐛𝐨𝐨𝐥() noexcept {
#ifdef __cpp_lib_span
      return 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌.empty();
#else
      return 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖻𝖾𝗀𝗂𝗇 == 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖾𝗇𝖽;
#endif
    }

   protected:
#ifdef __cpp_lib_span
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝐛𝐚𝐬𝐢𝐜_𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫(const std::span<const 𝐮𝐢𝐧𝐭₈> expected_contents) noexcept
        : 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌{expected_contents} {}
#else
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝐛𝐚𝐬𝐢𝐜_𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫(
        const 𝐮𝐢𝐧𝐭₈* const expected_contents_begin,
        const 𝐮𝐢𝐧𝐭₈* const expected_contents_end) noexcept
        : 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖻𝖾𝗀𝗂𝗇{expected_contents_begin}, 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖾𝗇𝖽{expected_contents_end} {}
#endif

   private:
    friend 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::⒭, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>;
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr decltype(auto) emit₈(𝐮𝐢𝐧𝐭₈ value) {
#ifdef __cpp_lib_span
      𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, not 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌.empty());
      𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖊𝖖𝖚𝖆𝖑(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, value, 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌[0]);
      return typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::⒭{𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌.subspan(1)};
#else
      𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖇𝖊𝖑𝖔𝖜(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖻𝖾𝗀𝗂𝗇, 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖾𝗇𝖽);
      // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
      𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖊𝖖𝖚𝖆𝖑(𝓸𝓹𝓽𝓲𝓸𝓷𝓼, value, 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖻𝖾𝗀𝗂𝗇[0]);
      // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
      return typename 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻::⒭{𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖻𝖾𝗀𝗂𝗇 + 1, 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖾𝗇𝖽};
#endif
    }
#ifdef __cpp_lib_span
    const std::span<const 𝐮𝐢𝐧𝐭₈> 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌;
#else
    const 𝐮𝐢𝐧𝐭₈* const 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖻𝖾𝗀𝗂𝗇;
    const 𝐮𝐢𝐧𝐭₈* const 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖾𝗇𝖽;
#endif
  };

 private:
#ifdef __cpp_lib_span
  const std::span<const 𝐮𝐢𝐧𝐭₈> 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌;
#else
  const 𝐮𝐢𝐧𝐭₈* const 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖻𝖾𝗀𝗂𝗇;
  const 𝐮𝐢𝐧𝐭₈* const 𝖾𝗑𝗉𝖾𝖼𝗍𝖾𝖽_𝖼𝗈𝗇𝗍𝖾𝗇𝗍𝗌_𝖾𝗇𝖽;
#endif
};

template <
    auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼 = &::𝘆𝗮𝗰𝗲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔡𝔢𝔣𝔞𝔲𝔩𝔱,
    𝐜𝐨𝐧𝐬𝐭𝐞𝐱𝐩𝐫_𝐟𝐫𝐢𝐞𝐧𝐝𝐥𝐲 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂 = 𝔠𝔬𝔫𝔰𝔱𝔢𝔵𝔭𝔯_𝔲𝔫𝔣𝔯𝔦𝔢𝔫𝔡𝔩𝔶,
    𝐬𝐢𝐳𝐞 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮 = std::numeric_limits<𝐬𝐢𝐳𝐞>::max()>
class 𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓 : public 𝒃𝒂𝒔𝒊𝒄_𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<
                           𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>,
                           𝓸𝓹𝓽𝓲𝓸𝓷𝓼,
                           𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂,
                           𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮> {
  using ⒫ = 𝒃𝒂𝒔𝒊𝒄_𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<
      𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>,
      𝓸𝓹𝓽𝓲𝓸𝓷𝓼,
      𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂,
      𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>;
  using ⒮ = 𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>;

 public:
  template <typename... 𝓽𝔂𝓹𝓮>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓(𝓽𝔂𝓹𝓮... arg) : ⒫{arg...} {}

 private:
  friend class 𝒃𝒂𝒔𝒊𝒄_𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<
      𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>,
      𝓸𝓹𝓽𝓲𝓸𝓷𝓼,
      𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂,
      𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>;
  friend class ⒫::𝐛𝐚𝐬𝐢𝐜_𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫;
  class 𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫 : public ⒫::𝐛𝐚𝐬𝐢𝐜_𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫 {
    using ⒮ = 𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫;

   private:
    friend class 𝒃𝒂𝒔𝒊𝒄_𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<
        𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>,
        𝓸𝓹𝓽𝓲𝓸𝓷𝓼,
        𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂,
        𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>;
    friend class ⒫::𝐛𝐚𝐬𝐢𝐜_𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫;
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫(const std::array<𝐮𝐢𝐧𝐭₈, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>& expected_contents) noexcept
        : ⒫::𝐛𝐚𝐬𝐢𝐜_𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫{expected_contents} {}
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr 𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫(
        const std::array<𝐮𝐢𝐧𝐭₈, 𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓼𝓲𝔃𝓮>& expected_contents,
        𝐬𝐢𝐳𝐞 position) noexcept
        : ⒫::𝐛𝐚𝐬𝐢𝐜_𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫{expected_contents, position} {}
  };
  using ⒭ = 𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫;
};

template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝐜𝐨𝐧𝐬𝐭𝐞𝐱𝐩𝐫_𝐟𝐫𝐢𝐞𝐧𝐝𝐥𝐲 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂>
class 𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂, std::numeric_limits<𝐬𝐢𝐳𝐞>::max()>
    : public 𝒃𝒂𝒔𝒊𝒄_𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<
          𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂, std::numeric_limits<𝐬𝐢𝐳𝐞>::max()>,
          𝓸𝓹𝓽𝓲𝓸𝓷𝓼,
          𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂,
          std::numeric_limits<𝐬𝐢𝐳𝐞>::max()> {
  using ⒫ = 𝒃𝒂𝒔𝒊𝒄_𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<
      𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂, std::numeric_limits<𝐬𝐢𝐳𝐞>::max()>,
      𝓸𝓹𝓽𝓲𝓸𝓷𝓼,
      𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂,
      std::numeric_limits<𝐬𝐢𝐳𝐞>::max()>;
  using ⒮ = 𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂, std::numeric_limits<𝐬𝐢𝐳𝐞>::max()>;

 public:
  template <typename 𝓲𝓽𝓮𝓻𝓪𝓽𝓸𝓻>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr 𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓(𝓲𝓽𝓮𝓻𝓪𝓽𝓸𝓻 first, 𝐬𝐢𝐳𝐞 count) noexcept : ⒫{first, count} {}
  template <typename 𝓲𝓽𝓮𝓻𝓪𝓽𝓸𝓻, typename 𝓮𝓷𝓭>
  𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr 𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓(𝓲𝓽𝓮𝓻𝓪𝓽𝓸𝓻 first, 𝓮𝓷𝓭 last) noexcept : ⒫{first, last} {}

 private:
  friend class 𝒃𝒂𝒔𝒊𝒄_𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<
      𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂, std::numeric_limits<𝐬𝐢𝐳𝐞>::max()>,
      𝓸𝓹𝓽𝓲𝓸𝓷𝓼,
      𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂,
      std::numeric_limits<𝐬𝐢𝐳𝐞>::max()>;
  friend class ⒫::𝐛𝐚𝐬𝐢𝐜_𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫;
  class 𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫 : public ⒫::𝐛𝐚𝐬𝐢𝐜_𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫 {
    using ⒮ = 𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫;

   private:
    friend class 𝒃𝒂𝒔𝒊𝒄_𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<
        𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂, std::numeric_limits<𝐬𝐢𝐳𝐞>::max()>,
        𝓸𝓹𝓽𝓲𝓸𝓷𝓼,
        𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂,
        std::numeric_limits<𝐬𝐢𝐳𝐞>::max()>;
    friend class ⒫::𝐛𝐚𝐬𝐢𝐜_𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫;
#ifdef __cpp_lib_span
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫(const std::span<const 𝐮𝐢𝐧𝐭₈> expected_contents) noexcept
        : ⒫::𝐛𝐚𝐬𝐢𝐜_𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫{expected_contents} {}
#else
    𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 explicit constexpr 𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫(
        const 𝐮𝐢𝐧𝐭₈* const expected_contents_begin,
        const 𝐮𝐢𝐧𝐭₈* const expected_contents_end) noexcept
        : ⒫::𝐛𝐚𝐬𝐢𝐜_𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫{expected_contents_begin, expected_contents_end} {}
#endif
  };
  using ⒭ = 𝐭𝐞𝐬𝐭_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝐯𝐞𝐫𝐢𝐟𝐢𝐞𝐫;
};

template <
    auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼 =&::𝘆𝗮𝗰𝗲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔞𝔰𝔰𝔢𝔯𝔱,
    𝐜𝐨𝐧𝐬𝐭𝐞𝐱𝐩𝐫_𝐟𝐫𝐢𝐞𝐧𝐝𝐥𝐲 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂 = 𝔠𝔬𝔫𝔰𝔱𝔢𝔵𝔭𝔯_𝔣𝔯𝔦𝔢𝔫𝔡𝔩𝔶,
    typename... 𝓽𝔂𝓹𝓮>
𝖞𝖆𝖈𝖊_𝖋𝖔𝖗𝖈𝖊𝖎𝖓𝖑𝖎𝖓𝖊 constexpr auto 𝑡𝑒𝑠𝑡_𝑎𝑠𝑠𝑒𝑚𝑏𝑙𝑒𝑟(𝓽𝔂𝓹𝓮... arg) {
  return 𝒕𝒆𝒔𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, 𝓬𝓸𝓷𝓼𝓽𝓮𝔁𝓹𝓻_𝓯𝓻𝓲𝓮𝓷𝓭𝓵𝔂, sizeof...(𝓽𝔂𝓹𝓮)>(arg...);
}

}  // namespace 𝘆𝗮𝗰𝗲::𝘅𝟴𝟲

#endif  // 𝔜𝔄ℭ𝔈_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_𝔛86_𝔗𝔈𝔖𝔗_𝔄𝔖𝔖𝔈𝔐𝔅𝔏𝔈ℜ_ℌ
