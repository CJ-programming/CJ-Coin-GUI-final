<!--
<script lang="ts">
	import { exit } from '@tauri-apps/api/process';
	import { goto } from '$app/navigation';

	import { password } from '$lib/store/password';

	let inputPath = '';

	async function exitApp() {
        await exit();
    }

	async function navigate(page: string) {
		goto(page); 
	}

	function handleSubmit() {
		password.set(inputPath)
	}

	$: {
		console.log($password)
	}

</script>

<div class="h-screen w-screen">
	<div class="flex flex-col h-full border-10 border-orange-200">
        <section class="flex-1 prose prose-p:my-1 max-w-[unset] py-10">
    		<div class="flex w-full h-full items-center justify-center-safe border">
				<div class="w-80 h-105 overflow-clip flex justify-center items-center border">
					<img alt="CJ-Coin" src="/pixil-frame-05.webp" class="w-auto h-auto m-0! block max-w-200 max-h-200">
				</div>
				<div class="flex flex-col border p-2">
					<h1 class="text-2xl text-center">Choose a password to encrypt your private key</h1>
					<div class="flex-1">	
						<div class="flex flex-row justify-center gap-1 [&>*]:text-center [&>*]:p-1 [&>*]:rounded">
							<input bind:value={inputPath} type="text" placeholder="Enter password"/>
							<button on:click={handleSubmit} class="border hover:bg-sky-100 cursor-grab">Submit</button>
						</div>
					</div>		
				</div>
    		</div>
		</section>
		<div class="p-2 flex flex-row justify-end bg-gray-100 ">
			<div class="flex flex-row justify-end text-lg items-center w-sm bg-gray-150 gap-2 [&>*]:w-32 [&>*]:border [&>*]:border-black [&>*]:text-center [&>*]:rounded [&>*]:text-white [&>*]:bg-sky-500 [&>*]:hover:bg-sky-700">
				<button on:click={exitApp}>Cancel</button>
				<button on:click={() => navigate("/choose_file_location")}>Back</button>
				<button on:click={() => navigate("/wallet_screen")}>Next</button>
			</div>
		</div>
    </div>
</div>
-->

<!--
<script lang="ts">
	import { exit } from '@tauri-apps/api/process';
	import { goto } from '$app/navigation';

	import { filePath } from '$lib/store/filePath';

	let inputPath = '';

	async function exitApp() {
        await exit();
    }

	async function navigate(page: string) {
		goto(page); 
	}

	function handleSubmit() {
		filePath.set(inputPath)
	}

	$: {
		console.log($filePath)
	}

</script>

<div class="h-screen w-screen">
	<div class="flex flex-col h-full border-10 border-orange-200">
        <section class="flex-1 prose prose-p:my-1 max-w-[unset] py-10">
    		<div class="flex w-full h-full items-center border">
				<div class="w-80 h-105 overflow-clip flex justify-center items-center border">
					<img alt="CJ-Coin" src="/pixil-frame-05.webp" class="w-auto h-auto m-0! block max-w-200 max-h-200">
				</div>
				<div class="flex flex-col border rounded p-2 m-2">
					<h1 class="text-3xl text-center">Choose a file location to set up</h1>
					<div class="flex-1">
						<div class="flex flex-row justify-center gap-1 [&>*]:text-center [&>*]:p-1 [&>*]:rounded">
							<input bind:value={inputPath} type="text" placeholder="Enter path"/>
							<button on:click={handleSubmit} class="border hover:bg-sky-100 cursor-grab">Submit</button>
						</div>
					</div>		
				</div>
    		</div>
		</section>
		<div class="p-2 flex flex-row justify-end bg-gray-100 ">
			<div class="flex flex-row justify-end text-lg items-center w-sm bg-gray-150 gap-2 [&>*]:w-32 [&>*]:border [&>*]:border-black [&>*]:text-center [&>*]:rounded [&>*]:text-white [&>*]:bg-sky-500 [&>*]:hover:bg-sky-700">
				<button on:click={exitApp}>Cancel</button>
				<button on:click={() => navigate("/")}>Back</button>
				<button on:click={() => navigate("/choose_password")}>Next</button>
			</div>
		</div>
    </div>
</div>
!-->

<script lang="ts">
	import { fly } from 'svelte/transition';
	import { onMount } from 'svelte';

	import { exit } from '@tauri-apps/api/process';
	import { goto } from '$app/navigation';
    import { filePath } from '$lib/store/filePath';

	import { open } from '@tauri-apps/api/dialog';

	let show = false;

	let inputPath = "";
	
	async function handleSubmit() {

	}

	let selectedFolder: string | null = null;

	async function selectFolder() {
	selectedFolder = await open({
		directory: true,
		multiple: false,
		title: 'Select Installation Folder'
	}) as string;
	}

	onMount(() => {
		show = true;
	});

	async function exitApp() {
        await exit();
    }

	async function navigate(page: string) {
		goto(page); 
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
			<div transition:fly={{ y: -300, duration: 800 }} class="flex flex-col items-center gap-3 text-center">
				<h1
					class="text-3xl font-bold bg-gray-800 rounded-xl border border-gray-600 p-2"
				>
					Congratulations, you have finished the installation for CJ-Coin!
				</h1>
			</div>
		{/if}
	</div>


	<div class="p-2 w-full flex flex-row justify-end text-sm bg-gray-800">
		{#if show}
			<div class="flex flex-row items-center text-center gap-3 rounded justify-end w-sm [&>*]:w-32 [&>*]:text-center [&>*]:rounded [&>*]:transition [&>*]:duration-300 [&>*]:shadow-md [&>*]:hover:shadow-lg">
				<button on:click={exitApp} class="bg-red-600 hover:bg-red-700">Cancel</button>
				<button on:click={() => history.back()} class="bg-blue-600 hover:bg-blue-700">Back</button>
				<button on:click={() => navigate("/wallet_info")} class="bg-green-600 hover:bg-green-700 font-semibold">Finish</button>
			</div>
		{/if}
	</div>
</main>
