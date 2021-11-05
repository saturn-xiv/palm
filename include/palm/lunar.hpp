#pragma once

#include "palm/env.hpp"

namespace palm {
namespace lunar {
const int LUNAR[] = {0x04bd8, 0x04ae0, 0x0a570, 0x054d5, 0x0d260, 0x0d950,
                     0x16554, 0x056a0, 0x09ad0, 0x055d2,  // 1900-1909
                     0x04ae0, 0x0a5b6, 0x0a4d0, 0x0d250, 0x1d255, 0x0b540,
                     0x0d6a0, 0x0ada2, 0x095b0, 0x14977,  // 1910-1919
                     0x04970, 0x0a4b0, 0x0b4b5, 0x06a50, 0x06d40, 0x1ab54,
                     0x02b60, 0x09570, 0x052f2, 0x04970,  // 1920-1929
                     0x06566, 0x0d4a0, 0x0ea50, 0x06e95, 0x05ad0, 0x02b60,
                     0x186e3, 0x092e0, 0x1c8d7, 0x0c950,  // 1930-1939
                     0x0d4a0, 0x1d8a6, 0x0b550, 0x056a0, 0x1a5b4, 0x025d0,
                     0x092d0, 0x0d2b2, 0x0a950, 0x0b557,  // 1940-1949
                     0x06ca0, 0x0b550, 0x15355, 0x04da0, 0x0a5b0, 0x14573,
                     0x052b0, 0x0a9a8, 0x0e950, 0x06aa0,  // 1950-1959
                     0x0aea6, 0x0ab50, 0x04b60, 0x0aae4, 0x0a570, 0x05260,
                     0x0f263, 0x0d950, 0x05b57, 0x056a0,  // 1960-1969
                     0x096d0, 0x04dd5, 0x04ad0, 0x0a4d0, 0x0d4d4, 0x0d250,
                     0x0d558, 0x0b540, 0x0b6a0, 0x195a6,  // 1970-1979
                     0x095b0, 0x049b0, 0x0a974, 0x0a4b0, 0x0b27a, 0x06a50,
                     0x06d40, 0x0af46, 0x0ab60, 0x09570,  // 1980-1989
                     0x04af5, 0x04970, 0x064b0, 0x074a3, 0x0ea50, 0x06b58,
                     0x055c0, 0x0ab60, 0x096d5, 0x092e0,  // 1990-1999
                     0x0c960, 0x0d954, 0x0d4a0, 0x0da50, 0x07552, 0x056a0,
                     0x0abb7, 0x025d0, 0x092d0, 0x0cab5,  // 2000-2009
                     0x0a950, 0x0b4a0, 0x0baa4, 0x0ad50, 0x055d9, 0x04ba0,
                     0x0a5b0, 0x15176, 0x052b0, 0x0a930,  // 2010-2019
                     0x07954, 0x06aa0, 0x0ad50, 0x05b52, 0x04b60, 0x0a6e6,
                     0x0a4e0, 0x0d260, 0x0ea65, 0x0d530,  // 2020-2029
                     0x05aa0, 0x076a3, 0x096d0, 0x04afb, 0x04ad0, 0x0a4d0,
                     0x1d0b6, 0x0d250, 0x0d520, 0x0dd45,  // 2030-2039
                     0x0b5a0, 0x056d0, 0x055b2, 0x049b0, 0x0a577, 0x0a4b0,
                     0x0aa50, 0x1b255, 0x06d20, 0x0ada0,  // 2040-2049
                     /**Add By JJonline@JJonline.Cn**/
                     0x14b63, 0x09370, 0x049f8, 0x04970, 0x064b0, 0x168a6,
                     0x0ea50, 0x06b20, 0x1a6c4, 0x0aae0,  // 2050-2059
                     0x0a2e0, 0x0d2e3, 0x0c960, 0x0d557, 0x0d4a0, 0x0da50,
                     0x05d55, 0x056a0, 0x0a6d0, 0x055d4,  // 2060-2069
                     0x052d0, 0x0a9b8, 0x0a950, 0x0b4a0, 0x0b6a6, 0x0ad50,
                     0x055a0, 0x0aba4, 0x0a5b0, 0x052b0,  // 2070-2079
                     0x0b273, 0x06930, 0x07337, 0x06aa0, 0x0ad50, 0x14b55,
                     0x04b60, 0x0a570, 0x054e4, 0x0d160,  // 2080-2089
                     0x0e968, 0x0d520, 0x0daa0, 0x16aa6, 0x056d0, 0x04ae0,
                     0x0a9d4, 0x0a2d0, 0x0d150, 0x0f252,  // 2090-2099
                     0x0d520};

const int SOLAR_MONTH[] = {31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31};

const char* TIAN_GAN[] = {"\u7532", "\u4e59", "\u4e19", "\u4e01", "\u620a",
                          "\u5df1", "\u5e9a", "\u8f9b", "\u58ec", "\u7678"};

const char* DI_ZHI[] = {"\u5b50", "\u4e11", "\u5bc5", "\u536f",
                        "\u8fb0", "\u5df3", "\u5348", "\u672a",
                        "\u7533", "\u9149", "\u620c", "\u4ea5"};

const char* ANIMALS[] = {"\u9f20", "\u725b", "\u864e", "\u5154",
                         "\u9f99", "\u86c7", "\u9a6c", "\u7f8a",
                         "\u7334", "\u9e21", "\u72d7", "\u732a"};
const char* SOLAR_TERMS[] = {
    "\u5c0f\u5bd2", "\u5927\u5bd2", "\u7acb\u6625", "\u96e8\u6c34",
    "\u60ca\u86f0", "\u6625\u5206", "\u6e05\u660e", "\u8c37\u96e8",
    "\u7acb\u590f", "\u5c0f\u6ee1", "\u8292\u79cd", "\u590f\u81f3",
    "\u5c0f\u6691", "\u5927\u6691", "\u7acb\u79cb", "\u5904\u6691",
    "\u767d\u9732", "\u79cb\u5206", "\u5bd2\u9732", "\u971c\u964d",
    "\u7acb\u51ac", "\u5c0f\u96ea", "\u5927\u96ea", "\u51ac\u81f3"};

const char* TERM_INFO[] = {
    "9778397bd097c36b0b6fc9274c91aa", "97b6b97bd19801ec9210c965cc920e",
    "97bcf97c3598082c95f8c965cc920f", "97bd0b06bdb0722c965ce1cfcc920f",
    "b027097bd097c36b0b6fc9274c91aa", "97b6b97bd19801ec9210c965cc920e",
    "97bcf97c359801ec95f8c965cc920f", "97bd0b06bdb0722c965ce1cfcc920f",
    "b027097bd097c36b0b6fc9274c91aa", "97b6b97bd19801ec9210c965cc920e",
    "97bcf97c359801ec95f8c965cc920f", "97bd0b06bdb0722c965ce1cfcc920f",
    "b027097bd097c36b0b6fc9274c91aa", "9778397bd19801ec9210c965cc920e",
    "97b6b97bd19801ec95f8c965cc920f", "97bd09801d98082c95f8e1cfcc920f",
    "97bd097bd097c36b0b6fc9210c8dc2", "9778397bd197c36c9210c9274c91aa",
    "97b6b97bd19801ec95f8c965cc920e", "97bd09801d98082c95f8e1cfcc920f",
    "97bd097bd097c36b0b6fc9210c8dc2", "9778397bd097c36c9210c9274c91aa",
    "97b6b97bd19801ec95f8c965cc920e", "97bcf97c3598082c95f8e1cfcc920f",
    "97bd097bd097c36b0b6fc9210c8dc2", "9778397bd097c36c9210c9274c91aa",
    "97b6b97bd19801ec9210c965cc920e", "97bcf97c3598082c95f8c965cc920f",
    "97bd097bd097c35b0b6fc920fb0722", "9778397bd097c36b0b6fc9274c91aa",
    "97b6b97bd19801ec9210c965cc920e", "97bcf97c3598082c95f8c965cc920f",
    "97bd097bd097c35b0b6fc920fb0722", "9778397bd097c36b0b6fc9274c91aa",
    "97b6b97bd19801ec9210c965cc920e", "97bcf97c359801ec95f8c965cc920f",
    "97bd097bd097c35b0b6fc920fb0722", "9778397bd097c36b0b6fc9274c91aa",
    "97b6b97bd19801ec9210c965cc920e", "97bcf97c359801ec95f8c965cc920f",
    "97bd097bd097c35b0b6fc920fb0722", "9778397bd097c36b0b6fc9274c91aa",
    "97b6b97bd19801ec9210c965cc920e", "97bcf97c359801ec95f8c965cc920f",
    "97bd097bd07f595b0b6fc920fb0722", "9778397bd097c36b0b6fc9210c8dc2",
    "9778397bd19801ec9210c9274c920e", "97b6b97bd19801ec95f8c965cc920f",
    "97bd07f5307f595b0b0bc920fb0722", "7f0e397bd097c36b0b6fc9210c8dc2",
    "9778397bd097c36c9210c9274c920e", "97b6b97bd19801ec95f8c965cc920f",
    "97bd07f5307f595b0b0bc920fb0722", "7f0e397bd097c36b0b6fc9210c8dc2",
    "9778397bd097c36c9210c9274c91aa", "97b6b97bd19801ec9210c965cc920e",
    "97bd07f1487f595b0b0bc920fb0722", "7f0e397bd097c36b0b6fc9210c8dc2",
    "9778397bd097c36b0b6fc9274c91aa", "97b6b97bd19801ec9210c965cc920e",
    "97bcf7f1487f595b0b0bb0b6fb0722", "7f0e397bd097c35b0b6fc920fb0722",
    "9778397bd097c36b0b6fc9274c91aa", "97b6b97bd19801ec9210c965cc920e",
    "97bcf7f1487f595b0b0bb0b6fb0722", "7f0e397bd097c35b0b6fc920fb0722",
    "9778397bd097c36b0b6fc9274c91aa", "97b6b97bd19801ec9210c965cc920e",
    "97bcf7f1487f531b0b0bb0b6fb0722", "7f0e397bd097c35b0b6fc920fb0722",
    "9778397bd097c36b0b6fc9274c91aa", "97b6b97bd19801ec9210c965cc920e",
    "97bcf7f1487f531b0b0bb0b6fb0722", "7f0e397bd07f595b0b6fc920fb0722",
    "9778397bd097c36b0b6fc9274c91aa", "97b6b97bd19801ec9210c9274c920e",
    "97bcf7f0e47f531b0b0bb0b6fb0722", "7f0e397bd07f595b0b0bc920fb0722",
    "9778397bd097c36b0b6fc9210c91aa", "97b6b97bd197c36c9210c9274c920e",
    "97bcf7f0e47f531b0b0bb0b6fb0722", "7f0e397bd07f595b0b0bc920fb0722",
    "9778397bd097c36b0b6fc9210c8dc2", "9778397bd097c36c9210c9274c920e",
    "97b6b7f0e47f531b0723b0b6fb0722", "7f0e37f5307f595b0b0bc920fb0722",
    "7f0e397bd097c36b0b6fc9210c8dc2", "9778397bd097c36b0b70c9274c91aa",
    "97b6b7f0e47f531b0723b0b6fb0721", "7f0e37f1487f595b0b0bb0b6fb0722",
    "7f0e397bd097c35b0b6fc9210c8dc2", "9778397bd097c36b0b6fc9274c91aa",
    "97b6b7f0e47f531b0723b0b6fb0721", "7f0e27f1487f595b0b0bb0b6fb0722",
    "7f0e397bd097c35b0b6fc920fb0722", "9778397bd097c36b0b6fc9274c91aa",
    "97b6b7f0e47f531b0723b0b6fb0721", "7f0e27f1487f531b0b0bb0b6fb0722",
    "7f0e397bd097c35b0b6fc920fb0722", "9778397bd097c36b0b6fc9274c91aa",
    "97b6b7f0e47f531b0723b0b6fb0721", "7f0e27f1487f531b0b0bb0b6fb0722",
    "7f0e397bd097c35b0b6fc920fb0722", "9778397bd097c36b0b6fc9274c91aa",
    "97b6b7f0e47f531b0723b0b6fb0721", "7f0e27f1487f531b0b0bb0b6fb0722",
    "7f0e397bd07f595b0b0bc920fb0722", "9778397bd097c36b0b6fc9274c91aa",
    "97b6b7f0e47f531b0723b0787b0721", "7f0e27f0e47f531b0b0bb0b6fb0722",
    "7f0e397bd07f595b0b0bc920fb0722", "9778397bd097c36b0b6fc9210c91aa",
    "97b6b7f0e47f149b0723b0787b0721", "7f0e27f0e47f531b0723b0b6fb0722",
    "7f0e397bd07f595b0b0bc920fb0722", "9778397bd097c36b0b6fc9210c8dc2",
    "977837f0e37f149b0723b0787b0721", "7f07e7f0e47f531b0723b0b6fb0722",
    "7f0e37f5307f595b0b0bc920fb0722", "7f0e397bd097c35b0b6fc9210c8dc2",
    "977837f0e37f14998082b0787b0721", "7f07e7f0e47f531b0723b0b6fb0721",
    "7f0e37f1487f595b0b0bb0b6fb0722", "7f0e397bd097c35b0b6fc9210c8dc2",
    "977837f0e37f14998082b0787b06bd", "7f07e7f0e47f531b0723b0b6fb0721",
    "7f0e27f1487f531b0b0bb0b6fb0722", "7f0e397bd097c35b0b6fc920fb0722",
    "977837f0e37f14998082b0787b06bd", "7f07e7f0e47f531b0723b0b6fb0721",
    "7f0e27f1487f531b0b0bb0b6fb0722", "7f0e397bd097c35b0b6fc920fb0722",
    "977837f0e37f14998082b0787b06bd", "7f07e7f0e47f531b0723b0b6fb0721",
    "7f0e27f1487f531b0b0bb0b6fb0722", "7f0e397bd07f595b0b0bc920fb0722",
    "977837f0e37f14998082b0787b06bd", "7f07e7f0e47f531b0723b0b6fb0721",
    "7f0e27f1487f531b0b0bb0b6fb0722", "7f0e397bd07f595b0b0bc920fb0722",
    "977837f0e37f14998082b0787b06bd", "7f07e7f0e47f149b0723b0787b0721",
    "7f0e27f0e47f531b0b0bb0b6fb0722", "7f0e397bd07f595b0b0bc920fb0722",
    "977837f0e37f14998082b0723b06bd", "7f07e7f0e37f149b0723b0787b0721",
    "7f0e27f0e47f531b0723b0b6fb0722", "7f0e397bd07f595b0b0bc920fb0722",
    "977837f0e37f14898082b0723b02d5", "7ec967f0e37f14998082b0787b0721",
    "7f07e7f0e47f531b0723b0b6fb0722", "7f0e37f1487f595b0b0bb0b6fb0722",
    "7f0e37f0e37f14898082b0723b02d5", "7ec967f0e37f14998082b0787b0721",
    "7f07e7f0e47f531b0723b0b6fb0722", "7f0e37f1487f531b0b0bb0b6fb0722",
    "7f0e37f0e37f14898082b0723b02d5", "7ec967f0e37f14998082b0787b06bd",
    "7f07e7f0e47f531b0723b0b6fb0721", "7f0e37f1487f531b0b0bb0b6fb0722",
    "7f0e37f0e37f14898082b072297c35", "7ec967f0e37f14998082b0787b06bd",
    "7f07e7f0e47f531b0723b0b6fb0721", "7f0e27f1487f531b0b0bb0b6fb0722",
    "7f0e37f0e37f14898082b072297c35", "7ec967f0e37f14998082b0787b06bd",
    "7f07e7f0e47f531b0723b0b6fb0721", "7f0e27f1487f531b0b0bb0b6fb0722",
    "7f0e37f0e366aa89801eb072297c35", "7ec967f0e37f14998082b0787b06bd",
    "7f07e7f0e47f149b0723b0787b0721", "7f0e27f1487f531b0b0bb0b6fb0722",
    "7f0e37f0e366aa89801eb072297c35", "7ec967f0e37f14998082b0723b06bd",
    "7f07e7f0e47f149b0723b0787b0721", "7f0e27f0e47f531b0723b0b6fb0722",
    "7f0e37f0e366aa89801eb072297c35", "7ec967f0e37f14998082b0723b06bd",
    "7f07e7f0e37f14998083b0787b0721", "7f0e27f0e47f531b0723b0b6fb0722",
    "7f0e37f0e366aa89801eb072297c35", "7ec967f0e37f14898082b0723b02d5",
    "7f07e7f0e37f14998082b0787b0721", "7f07e7f0e47f531b0723b0b6fb0722",
    "7f0e36665b66aa89801e9808297c35", "665f67f0e37f14898082b0723b02d5",
    "7ec967f0e37f14998082b0787b0721", "7f07e7f0e47f531b0723b0b6fb0722",
    "7f0e36665b66a449801e9808297c35", "665f67f0e37f14898082b0723b02d5",
    "7ec967f0e37f14998082b0787b06bd", "7f07e7f0e47f531b0723b0b6fb0721",
    "7f0e36665b66a449801e9808297c35", "665f67f0e37f14898082b072297c35",
    "7ec967f0e37f14998082b0787b06bd", "7f07e7f0e47f531b0723b0b6fb0721",
    "7f0e26665b66a449801e9808297c35", "665f67f0e37f1489801eb072297c35",
    "7ec967f0e37f14998082b0787b06bd", "7f07e7f0e47f531b0723b0b6fb0721",
    "7f0e27f1487f531b0b0bb0b6fb0722"};

const char* DAY_NAMES_ONE[] = {"\u65e5", "\u4e00", "\u4e8c", "\u4e09",
                               "\u56db", "\u4e94", "\u516d", "\u4e03",
                               "\u516b", "\u4e5d", "\u5341"};
const char* DAY_NAMES_TEN[] = {"\u521d", "\u5341", "\u5eff", "\u5345"};
const char* MONTN_NAMES[] = {"\u6b63", "\u4e8c", "\u4e09", "\u56db",
                             "\u4e94", "\u516d", "\u4e03", "\u516b",
                             "\u4e5d", "\u5341", "\u51ac", "\u814a"};

enum class 兩儀 { 陰, 陽 };
enum class 四象 { 少陽, 少陰, 太陽, 太陰 };
enum class 五行 { 金, 木, 水, 火, 土 };
enum class 天干 { 甲, 乙, 丙, 丁, 戊, 己, 庚, 辛, 壬, 癸 };
enum class 地支 { 子, 丑, 寅, 卯, 辰, 巳, 午, 未, 申, 酉, 戌, 亥 };
enum class 八卦 { 乾, 兑, 离, 震, 巽, 坎, 艮, 坤 };
enum class 六十四卦 {
  乾 = 0,
  坤,
  屯,
  蒙,
  需,
  訟,
  師,
  比,
  小畜,
  履,
  泰,
  否,
  同人,
  大有,
  謙,
  豫,
  隨,
  蠱,
  臨,
  觀,
  噬嗑,
  賁,
  剝,
  復,
  無妄,
  大畜,
  頤,
  大過,
  坎,
  離,
  咸,
  恆,
  遯,
  大壯,
  晉,
  明夷,
  家人,
  睽,
  蹇,
  解,
  損,
  益,
  夬,
  姤,
  萃,
  升,
  困,
  井,
  革,
  鼎,
  震,
  艮,
  漸,
  歸妹,
  豐,
  旅,
  巽,
  兌,
  渙,
  節,
  中孚,
  至,
  小過,
  既濟,
  未濟
};

// https://zh.wiktionary.org/zh-hant/Appendix:Unicode/%E6%98%93%E7%B6%93%E5%85%AD%E5%8D%81%E5%9B%9B%E5%8D%A6%E7%AC%A6%E8%99%9F
std::ostream& operator<<(std::ostream& out, const 六十四卦& self) {
  const unsigned int first = static_cast<unsigned int>("U+4DC0");
  const unsigned int id = static_cast<unsigned int>(self);
  // const wchar_t ic = static_cast<wchar_t>(first + id) out << id;
  // out << ic;
  return out;
}
enum class 納甲 {};

}  // namespace lunar
}  // namespace palm
