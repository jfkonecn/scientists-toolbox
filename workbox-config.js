// https://developer.chrome.com/docs/workbox/the-ways-of-workbox/ 
// https://developer.chrome.com/docs/workbox/reference/workbox-build/ 
// https://developer.chrome.com/docs/workbox/precaching-with-workbox/ 
module.exports = {
	globDirectory: 'dist/',
	globPatterns: [
		'**/*.{css,html,json,wasm,js,png,jpg}'
	],
	globIgnores: [
		'workbox-config.js',
		'node_modules/**'
	],
	swDest: 'dist/sw.js',
	swSrc: 'dist/sw.js',
};