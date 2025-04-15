#include "cxx-qt-io-test-utils/test_utils.h"

#include <QtCore/QTimer>

const int TIMEOUT_SIGNAL = 123;

namespace rust {
namespace cxxqtio1 {
namespace test {

class ClosureHandler : public QObject
{
public:
  bool event(QEvent* event) override
  {
    event->setAccepted(true);
    return true;
  }
};

TestContext::~TestContext()
{
  for (QObject* object : held)
    delete object;
}

void
TestContext::exit()
{
  if (QCoreApplication::instance()) {
    QCoreApplication::quit();
  }
}

void
TestContext::hold(QObject* object)
{
  held.emplace_back(object);
}

bool
TestContext::run(QEvent* event)
{
  static ClosureHandler* receiver = new ClosureHandler();
  QCoreApplication::postEvent(receiver, event);

  int result = QCoreApplication::exec();
  return result != TIMEOUT_SIGNAL;
}

void
TestContext::timeoutAfter(int msecs)
{
  QCoreApplication* app = QCoreApplication::instance();
  QTimer::singleShot(msecs, [app]() {
    if (QCoreApplication::instance() == app)
      QCoreApplication::exit(TIMEOUT_SIGNAL);
  });
}

}
}
}
