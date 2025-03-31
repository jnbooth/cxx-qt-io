#pragma once
#include <QtCore/QObject>
#include <memory>

namespace rust {
namespace cxxqtio1 {

template<typename Sub, typename Base>
const Base*
upcast(const Sub* sub)
{
  static_assert(std::is_base_of_v<Base, Sub>);
  return static_cast<const Base*>(sub);
}

template<typename Sub, typename Base>
const Sub*
downcast(const Base* base)
{
  static_assert(std::is_base_of_v<Base, Sub>);
  return qobject_cast<const Sub*>(base);
}

}
}
