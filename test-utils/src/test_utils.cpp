#include "cxx-qt-io-test-utils/test_utils.h"

#include <QtCore/QTimer>

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

void
TestContext::exit(int returnCode)
{
  eventLoop.exit(returnCode);
}

int
TestContext::run(QEvent* event)
{
  static ClosureHandler* receiver = new ClosureHandler();
  QCoreApplication::postEvent(receiver, event);

  return eventLoop.exec();
}

}
}
}
