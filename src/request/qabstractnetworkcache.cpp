#include "cxx-qt-io/qabstractnetworkcache.h"

namespace rust {
namespace cxxqtio1 {
void
qabstractnetworkcacheClear(QAbstractNetworkCache& cache)
{
  cache.clear();
}

::std::int64_t
qabstractnetworkcacheCacheSize(const QAbstractNetworkCache& cache)
{
  return cache.cacheSize();
}

QIODevice*
qabstractnetworkcacheData(QAbstractNetworkCache& cache, const QUrl& url)
{
  return cache.data(url);
}

void
qabstractnetworkcacheInsert(QAbstractNetworkCache& cache, QIODevice* device)
{
  cache.insert(device);
}

QNetworkCacheMetaData
qabstractnetworkcacheMetaData(QAbstractNetworkCache& cache, const QUrl& url)
{
  return cache.metaData(url);
}

QIODevice*
qabstractnetworkcachePrepare(QAbstractNetworkCache& cache,
                             const QNetworkCacheMetaData& metaData)
{
  return cache.prepare(metaData);
}

bool
qabstractnetworkcacheRemove(QAbstractNetworkCache& cache, const QUrl& url)
{
  return cache.remove(url);
}

void
qabstractnetworkcacheUpdateMetaData(QAbstractNetworkCache& cache,
                                    const QNetworkCacheMetaData& metaData)
{
  return cache.updateMetaData(metaData);
}

}
}
