
<script lang="ts">
    import { goto } from "$app/navigation";

	// Replace with actual wallet data
	let walletAddress = 'cj1qf4v2s8g3nxt4rghjw9vqps9lz5u6dhv95z8j2c';
	let walletBalance = 125.4302;
</script>

<main class="min-h-screen bg-gray-900 text-white flex flex-col items-center justify-center p-6 space-y-6">
	<h1 class="text-3xl font-bold border-b-2 border-gray-700 pb-2">
		Wallet Overview
	</h1>

	<section class="w-full max-w-xl bg-gray-800 rounded-xl border border-gray-600 p-6 shadow-md">
		<div class="mb-4">
			<h2 class="text-xl font-semibold text-gray-200 mb-2">Wallet Address</h2>
			<div class="bg-gray-700 p-4 rounded text-sm font-mono break-all select-all">
				{walletAddress}
			</div>
		</div>

		<div>
			<h2 class="text-xl font-semibold">Balance</h2>
			<div class="bg-gray-700 p-4 rounded text-xl font-bold text-green-400">
				{walletBalance.toFixed(4)} CJC
			</div>
		</div>
	</section>


	<div class="w-full max-w-md bg-gray-800 border border-gray-600 rounded-xl p-3">
		<div class="flex justify-center gap-2">
			<button on:click={() => history.back()} class="flex-1 bg-blue-600 hover:bg-blue-700 rounded font-semibold">Back</button>
			<button on:click={() => goto("/wallet_screen")} class="flex-1 bg-green-600 hover:bg-green-700 rounded font-semibold">Wallet Interface</button>
		</div>
	</div>
</main>


<!--
<main class="min-h-screen bg-gray-900 text-white flex flex-col items-center justify-center p-6 space-y-6">
	<h1 class="text-3xl font-bold border rounded p-6">Wallet Overview</h1>
	<div class="">Hi</div>
</main>
-->