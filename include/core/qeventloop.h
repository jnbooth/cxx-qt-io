#pragma once

#include "rust/cxx.h"
#include <QtCore/QCoreApplication>
#include <QtCore/QEvent>
#include <QtCore/QEventLoop>

using QEventLoopProcessEventsFlag = QEventLoop::ProcessEventsFlag;
using QEventLoopProcessEventsFlags = QEventLoop::ProcessEventsFlags;

namespace rust {
namespace cxxqtio1 {

template<typename T>
class QEventLoopClosureEvent : public QEvent
{
public:
  inline QEventLoopClosureEvent(QEventLoop& eventLoop,
                                T& context,
                                rust::Fn<void(T&)> closure)
    : QEvent(QEvent::User)
    , eventLoop(eventLoop)
    , context(context)
    , closure(closure) {};

  ~QEventLoopClosureEvent() override
  {
    (*closure)(context);
    eventLoop.quit();
  }

private:
  QEventLoop& eventLoop;
  T& context;
  rust::Fn<bool(T&)> closure;
};

template<typename T>
int
qeventloopExecWith(QEventLoop& eventLoop,
                   T& context,
                   rust::Fn<void(T&)> closure)
{
  QEvent* event = new QEventLoopClosureEvent(eventLoop, context, closure);
  QCoreApplication::postEvent(&eventLoop, event, INT_MAX);
  return eventLoop.exec();
}

}
}
