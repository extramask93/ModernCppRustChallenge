#include <algorithm>
#include <array>
#include <cstddef>
#include <iostream>
#include <iterator>
#include <memory>
#include <stdint.h>
#include <vector>
class Table {
public:
    explicit Table(std::vector<std::string> header) : m_header(std::move(header)) {
    }
    void addColumn(const std::vector<std::string> column, std::size_t col_num=0) {
        m_rows.resize(std::max(m_rows.size(),column.size()));
        std::size_t i =0;
        for(const auto& val: column) {
            m_rows[i++][col_num] = val;
        }
    };
    void print() {
        std::for_each(m_header.cbegin(), m_header.cend(), [] (const auto &elem) {std::cout<<elem<<'\t';});
        std::cout<<std::endl;
        std::for_each(m_rows.cbegin(), m_rows.cend(), [] (const auto &row) {
            std::for_each(row.cbegin(), row.cend(), [] (const auto &elem) {std::cout<<elem<<'\t';});
            std::cout<<std::endl;
                });
    }
private:
    std::vector<std::string> m_header;
    std::vector<std::vector<std::string>> m_rows;

};



template <typename T, size_t N>
std::unique_ptr<std::array<uint8_t, N>> to_binary(T number) {
  std::array<uint8_t, N> result;
  size_t i = 0;
  while (number != 0 || i < N) {
    T rest = number % 2;
    number = number / 2;
    result.at(i) = rest;
    ++i;
  }
  reverse(result.begin(), result.end());
  return std::make_unique<std::array<uint8_t, N>>(result);
}
template <typename T> T to_gray(T number) {
    number = number ^ (number>>1);
    return number;
}

typename std::array<uint8_t,5> binary;
int main() {
  std::vector<uint8_t> numbers;
  std::vector<std::unique_ptr<std::array<uint8_t,5>>> dest;
  auto print = [] (const auto &array) {
      std::for_each(array->begin(), array->end(), [] (const auto &c) {std::cout<<unsigned(c);});
      std::cout<<std::endl;
  };
  std::generate_n(std::back_inserter(numbers), 10, [] {static auto i =0; return i++;});
  std::transform(numbers.begin(), numbers.end(),std::back_inserter(dest),[] (auto x){return to_binary<uint8_t,5>(x);});
  std::for_each(dest.begin(), dest.end(),
                [&print](const auto &array) { print(array); });
  std::transform(numbers.begin(), numbers.end(),std::back_inserter(dest),[] (auto x){return to_binary<uint8_t,5>(to_gray(x));});
  std::for_each(dest.begin(), dest.end(),
                [&print](const auto &array) { print(array); });
  Table t({"Decimal", "Binary", "Gray"});
  return 0;
}
