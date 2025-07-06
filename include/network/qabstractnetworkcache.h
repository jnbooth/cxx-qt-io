#pragma once

#include <QtNetwork/QAbstractNetworkCache>

#include <QtCore/QIODevice>

namespace rust {
namespace cxxqtio1 {
void
qabstractnetworkcacheClear(QAbstractNetworkCache& cache);

qint64
qabstractnetworkcacheCacheSize(const QAbstractNetworkCache& cache);

QIODevice*
qabstractnetworkcacheData(QAbstractNetworkCache& cache, const QUrl& url);

void
qabstractnetworkcacheInsert(QAbstractNetworkCache& cache, QIODevice* device);

QNetworkCacheMetaData
qabstractnetworkcacheMetaData(QAbstractNetworkCache& cache, const QUrl& url);

QIODevice*
qabstractnetworkcachePrepare(QAbstractNetworkCache& cache,
                             const QNetworkCacheMetaData& metaData);

bool
qabstractnetworkcacheRemove(QAbstractNetworkCache& cache, const QUrl& url);

void
qabstractnetworkcacheUpdateMetaData(QAbstractNetworkCache& cache,
                                    const QNetworkCacheMetaData& metaData);
}
}
