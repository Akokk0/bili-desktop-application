<script>
	import '@/app.css';
	import { invoke } from '@tauri-apps/api/tauri';
	import QRCode from 'qrcode';
	import { onMount, onDestroy } from 'svelte';
	import { goto } from '$app/navigation';

	/**
	 * @type {HTMLCanvasElement}
	 */
	let canvas;
	let qrcodeUrl = '';
	let qrcodeKey = '';
	let qrcodeState = '';
	let showQRCode = true;

	/**
	 * @type {number | undefined}
	 */
	let checkLoginStatusTimer;

	onMount(async () => {
		console.log('onMount function called!');
		let isLogin = await invoke('is_login');
		console.log(isLogin);
		if (isLogin) {
			goto('/app');
		}
	});

	onDestroy(async () => {
		clearInterval(checkLoginStatusTimer);
	});

	// Get QRCode
	const getQRCode = async () => {
		const resp = await invoke('request', {
			url: 'https://passport.bilibili.com/x/passport-login/web/qrcode/generate',
			reqType: 'GET'
		});
		const respObj = JSON.parse(resp);
		// console.log(respObj);

		qrcodeUrl = respObj.data.url;
		qrcodeKey = respObj.data.qrcode_key;
		qrcodeUrl && (await QRCode.toCanvas(canvas, qrcodeUrl));
		checkLoginStatusTimer = setInterval(getLoginStatus, 500);
	};

	// Get login status
	const getLoginStatus = async () => {
		const resp = await invoke('request', {
			url: `https://passport.bilibili.com/x/passport-login/web/qrcode/poll?qrcode_key=${qrcodeKey}`,
			reqType: 'GET'
		});
		const respObj = JSON.parse(resp);
		qrcodeState = respObj.data.message;
		// if respObj.data.code === 0, it's means login in success
		if (respObj.data.code === 0) {
			await invoke('save_cookies');
			goto('/app');
		}
	};

	// Check cookies staus
	const checkCookiesStatus = async () => {
		await invoke('check_cookies_status')
	}
</script>

<div class="flex flex-col items-center justify-center h-screen">
	<div class="card w-96 bg-base-100 shadow-xl">
		<figure>
			<div>请选择登录方式</div>
		</figure>
		<div class="card-body">
			{#if showQRCode}
				<h2 class="card-title">{qrcodeState}</h2>
				<canvas class="rounded-box" bind:this={canvas}></canvas>
			{/if}
			<div class="card-actions justify-end">
				<button on:click={getQRCode} class="btn btn-primary">获取二维码</button>
			</div>
		</div>
	</div>

	<button class="absolute top-0 left-0 btn btn-primary" on:click={checkCookiesStatus}
		>Check login status</button
	>
	<button class="absolute bottom-0 left-0 btn btn-primary" on:click={getLoginStatus}
		>Get login state</button
	>
	<a class="absolute bottom-0 right-0 btn btn-primary" href="./app">To App</a>
</div>
