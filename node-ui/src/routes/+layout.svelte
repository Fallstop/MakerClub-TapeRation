<script lang="ts">
	import { serverConnect } from "$lib/api";
	import { onMount } from "svelte";

	onMount(() => {
		serverConnect();
	});
</script>

<div class="app">
	<main>
		<slot />
	</main>
</div>

<style>
	.app {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
		text-align: center;
	}

	main {
		flex: 1;
	}
</style>
