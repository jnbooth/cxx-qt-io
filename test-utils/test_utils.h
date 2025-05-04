#pragma once

#include <QtCore/QCoreApplication>
#include <QtCore/QEvent>
#include <QtCore/QIODevice>
#include <QtCore/QPointer>

#include "rust/cxx.h"

namespace rust {
namespace cxxqtio1 {
namespace test {
class TestContext
{
public:
  void exit(int returnCode = 0);
  int run(QEvent* event);

private:
  QEventLoop eventLoop;
};

template<typename T>
class ClosureEvent : public QEvent
{
public:
  inline ClosureEvent(TestContext& testContext,
                      T& closureContext,
                      rust::Fn<bool(T&)> closure)
    : QEvent(QEvent::User)
    , testContext(testContext)
    , closureContext(closureContext)
    , closure(closure) {};

  inline void setAccepted(bool accepted) override
  {
    if (accepted)
      testContext.exit(1 - (*closure)(closureContext));
  }

private:
  TestContext& testContext;
  T& closureContext;
  rust::Fn<bool(T&)> closure;
};

template<typename T>
int
run_inside_app(T& context, rust::Fn<bool(T&)> closure)
{
  TestContext testContext;
  return testContext.run(new ClosureEvent(testContext, context, closure));
}

}
}
}
