const CACHE_NAME = 'five-psalms-v1';
const ASSETS_TO_CACHE = [
	'/',
	'/index.html',
];
self.addEventListener('install', (event) => {
	event.waitUntil(
		caches.open(CACHE_NAME).then((cache) => {
			return cache.addAll(ASSETS_TO_CACHE);
		})
	);
	self.skipWaiting();
});

self.addEventListener('activate', (event) => {
	event.waitUntil(
		caches.keys().then((keys) => {
			return Promise.all(
				keys.map((key) => {
					if (key !== CACHE_NAME) {
						return caches.delete(key);
					}
				})
			);
		})
	);
	return self.clients.claim();
});

self.addEventListener('fetch', (event) => {
	// Pass-through fetch for now
	event.respondWith(
		caches.match(event.request).then((cachedResponse) => {
			if (cachedResponse) {
				return cachedResponse; // Return cached file if found
			}
			return fetch(event.request).then((networkResponse) => {
				// Dynamically cache new assets as they are fetched
				return caches.open(CACHE_NAME).then((cache) => {
					cache.put(event.request, networkResponse.clone());
					return networkResponse;
				});
			});
		})
	);
});
