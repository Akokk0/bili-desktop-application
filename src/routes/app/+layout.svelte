<script>
	import { goto } from '$app/navigation';
	import '@/app.css';
	import avatar from '@/assets/favicon.png';
	import { invoke } from '@tauri-apps/api/tauri';

	// Logout
	const logout = async () => {
		await invoke('logout');
		goto('/');
	};
</script>

<div class="flex w-full h-screen ml-3">
	<div class="grid h-full rounded-box place-items-center items-center">
		<!-- Menu -->
		<ul class="menu bg-base-200 rounded-box flex flex-col items-center">
			<!-- Menu items -->
			<li class="mb-4">
				<a href="/app">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-5 w-5"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
						/></svg
					>
				</a>
			</li>
			<li class="mb-4">
				<a href="/app/hot">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-5 w-5"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
						/></svg
					>
				</a>
			</li>
			<li class="mb-4">
				<a href="/app/personal">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-5 w-5"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
						/></svg
					>
				</a>
			</li>
			<li class="mb-4">
				<a href="/">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-5 w-5"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
						/></svg
					>
				</a>
			</li>
			<button class="btn btn-primary" on:click={logout}>Logout</button>
		</ul>
		<!-- Avatar -->
		<a class="absolute bottom-0 left-0" href="/app/personal">
			<img
				class="h-16 w-16 rounded-full mb-4 ml-5"
				src={avatar}
				alt="Avatar"
			/>
		</a>
	</div>
	<div class="divider divider-horizontal">Or</div>
	<div class="grid h- flex-grow card bg-base-300 rounded-box place-items-center">
		<slot />
	</div>
</div>
