<script lang="ts">
    import { goto } from "$app/navigation";

	let amount = "";
	let address = "";
	let fee = "";
	let message = "";

	const sendCrypto = async () => {
		// Replace this with actual Tauri backend logic
		message = `Sending ${amount} CJC to ${address} with fee ${fee}...`;
	};

	const mineBlock = async () => {
		// Replace this with actual Tauri backend logic
		message = "Mining block...";
	};
</script>

<main class="min-h-screen bg-gray-900 text-white flex flex-col items-center justify-center p-6 gap-2">
	<h1 class="text-3xl font-bold border-b-2 border-gray-700">CJC Wallet</h1>

	<div class="w-full max-w-md bg-gray-800 border border-gray-600 rounded-xl p-6 shadow-lg space-y-3">
		<h2 class="text-xl font-semibold border-b-2 border-gray-700">Send CJC</h2>

		<input
			class="w-full p-2 bg-gray-700 rounded border border-gray-600 focus:outline-none"
			placeholder="Recipient Address"
			bind:value={address}
		/>
		<input
			class="w-full p-2 bg-gray-700 rounded border border-gray-600 focus:outline-none"
			placeholder="Amount"
			type="number"
			bind:value={amount}
		/>
		<input
			class="w-full p-2 bg-gray-700 rounded border border-gray-600 focus:outline-none"
			placeholder="Transaction Fee"
			type="number"
			bind:value={fee}
		/>

		<button
			class="w-full bg-green-600 hover:bg-green-700 text-white font-semibold py-2 rounded"
			on:click={sendCrypto}
		>
			Send
		</button>
	</div>

	<div class="w-full max-w-md border border-gray-600 bg-gray-800 rounded-xl p-6 shadow-lg space-y-4">
		<h2 class="text-xl font-semibold border-b-2 border-gray-700">Mining</h2>

		<button
			class="w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 rounded"
			on:click={mineBlock}
		>
			Mine Block
		</button>
	</div>

	{#if message}
		<p class="text-green-400 font-mono">{message}</p>
	{/if}
	
	<div class="w-full max-w-md bg-gray-800 border border-gray-600 rounded-xl p-3">
		<div class="flex justify-center gap-2">
			<button on:click={() => history.back()} class="flex-1 bg-blue-600 hover:bg-blue-700 rounded font-semibold">Back</button>
			<button on:click={() => goto("/wallet_info")} class="flex-1 bg-green-600 hover:bg-green-700 rounded font-semibold">Wallet Info</button>
		</div>
	</div>
</main>