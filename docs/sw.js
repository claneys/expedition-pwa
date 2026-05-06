const CACHE_NAME = 'expedition-v1';
const ASSETS = [
    '/',
    '/index.html',
    '/manifest.json',
    '/icons/icon-192x192.png',
    '/icons/icon-512x512.png'
];

// Installation du service worker
self.addEventListener('install', (event) => {
    event.waitUntil(
        caches.open(CACHE_NAME)
            .then((cache) => {
                return cache.addAll(ASSETS);
            })
            .catch((err) => {
                console.log('Cache error:', err);
            })
    );
    self.skipWaiting();
});

// Activation du service worker
self.addEventListener('activate', (event) => {
    event.waitUntil(
        caches.keys().then((cacheNames) => {
            return Promise.all(
                cacheNames
                    .filter((name) => name !== CACHE_NAME)
                    .map((name) => caches.delete(name))
            );
        })
    );
    self.clients.claim();
});

// Interception des requêtes
self.addEventListener('fetch', (event) => {
    event.respondWith(
        caches.match(event.request)
            .then((response) => {
                // Retourner le cache si trouvé
                if (response) {
                    return response;
                }

                // Sinon faire la requête réseau
                return fetch(event.request)
                    .then((networkResponse) => {
                        // Ne pas mettre en cache les requêtes API
                        if (event.request.url.includes('/api/')) {
                            return networkResponse;
                        }

                        // Mettre en cache les nouvelles ressources
                        return caches.open(CACHE_NAME).then((cache) => {
                            cache.put(event.request, networkResponse.clone());
                            return networkResponse;
                        });
                    })
                    .catch(() => {
                        // Si offline et pas en cache, retourner la page offline
                        if (event.request.mode === 'navigate') {
                            return caches.match('/');
                        }
                    });
            })
    );
});
