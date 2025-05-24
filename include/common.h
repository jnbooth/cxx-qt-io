#pragma once
#include <QtCore/QObject>
#include <memory>

namespace rust {
namespace cxxqtio1 {

template<typename A, typename B>
A
operatorPlus(A a, B b)
{
  return a + b;
}

template<typename A, typename B>
A
operatorMinus(A a, B b)
{
  return a - b;
}

}
}
