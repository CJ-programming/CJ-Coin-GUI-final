<script lang="ts">
	import { fly } from 'svelte/transition';
	import { onMount } from 'svelte';

	import { exit } from '@tauri-apps/api/process';
	import { goto } from '$app/navigation';

	let show = false;

	onMount(() => {
		show = true;
	});

	async function exitApp() {
        await exit();
    }
</script>


<main
	class="min-h-screen bg-gray-900 text-white flex flex-col items-center justify-center p-2 pl-2 space-y-5"
>
	<div class="flex flex-1 items-center">
		<div class="p-4">
			<div class="w-55 h-80 overflow-clip flex justify-center items-center">
				<img
					alt="CJ-Coin"
					src="/cjcoinlogo.png"
					class="w-auto h-auto m-0! p-11 block max-w-80 max-h-90"
				/>
			</div>
		</div>
		{#if show}
			<div transition:fly={{ y: -300, duration: 800 }} class="flex flex-col items-center gap-3 text-center"> <!--Simple animation for the welcome text-->
				<h1
					class="text-3xl font-bold bg-gray-800 rounded-xl border border-gray-600 p-2"
				>
					Welcome to the CJ-Coin Core (64-bit) Setup Wizard
				</h1>
				<p class="bg-gray-800 rounded-xl border border-gray-600 p-2">
					It is recommended that you close all other applications before
					starting setup. This will make it possible to update relevant
					system files without having to reboot your computer.
				</p>
			</div>
		{/if}
	</div>


	<div class="p-2 w-full flex flex-row justify-end text-sm bg-gray-800 border-gray-600 rounded">
		{#if show}
			<div transition:fly={{ x: -300, duration: 800 }} class="flex flex-col items-center gap-3 text-center">
				<div class="flex flex-row justify-end w-sm gap-3 [&>*]:w-32 [&>*]:text-center
				[&>*]:rounded [&>*]:transition [&>*]:duration-200 [&>*]:shadow-md [&>*]:hover:shadow-lg">
					<button on:click={exitApp} class="bg-red-600 hover:bg-red-700">Cancel</button> <!--Exits out of the application-->
					<button on:click={() => goto("/choose_file_location")}
						class="bg-green-600 hover:bg-green-700 font-semibold">
						Next</button> <!--Goes to the next step of hte installation-->
				</div>
			</div>
		{/if}
	</div>
</main>