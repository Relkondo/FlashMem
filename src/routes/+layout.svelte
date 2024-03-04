
<script>
	import "../app.css";
	import { page } from '$app/stores';
</script>

<div class="w-fit">
	<nav class="h-12 mt-2">
		<a href="/"
			 class="hover:text-green-500 py-4 px-5 text-center align-middle transition-colors duration-300 rounded leading-loose
				{$page.url.pathname === '/' ? 'bg-gray-800':''}">
			Home</a>
		<a href="/saved_subs"
			 class="hover:text-green-500 py-4 px-5 text-center align-middle transition-colors duration-300 rounded leading-loose
				{$page.url.pathname === '/saved_subs' ? 'bg-gray-800':''}">
			Saved Subs</a>
		<a href="/advanced_settings"
			 class="hover:text-green-500 py-4 px-5 text-center align-middle transition-colors duration-300 rounded leading-loose
				{$page.url.pathname === '/advanced_settings' ? 'bg-gray-800':''}">
			Advanced Settings</a>
	</nav>
</div>

<slot />