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
  ~TestContext();

  void exit();
  void hold(QObject* object);
  bool run(QEvent* event);
  void timeoutAfter(int msecs);

private:
  std::vector<QPointer<QObject>> held{};
};

template<typename T>
class ClosureEvent : public QEvent
{
public:
  inline ClosureEvent(TestContext& testContext,
                      T& closureContext,
                      rust::Fn<bool(T&, TestContext&)> closure)
    : QEvent(QEvent::User)
    , testContext(testContext)
    , closureContext(closureContext)
    , closure(closure) {};

  inline void setAccepted(bool accepted) override
  {
    if (accepted && !(*closure)(closureContext, testContext))
      QCoreApplication::exit(1);
  }

private:
  TestContext& testContext;
  T& closureContext;
  rust::Fn<bool(T&, TestContext&)> closure;
};

template<typename T>
bool
run_inside_app(T& context, rust::Fn<bool(T&, TestContext&)> closure)
{
  TestContext testContext;
  return testContext.run(new ClosureEvent(testContext, context, closure));
}

}
}
}
