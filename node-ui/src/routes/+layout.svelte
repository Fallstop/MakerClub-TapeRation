<script lang="ts">
	import { serverConnect } from "$lib/api";
	import { onMount } from "svelte";
	import "./styles.scss";

	onMount(() => {
		serverConnect();
	});
</script>

<div class="app">
		<slot />
</div>

<style>
	.app {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
		text-align: center;
		padding: 0 4rem;
	}
</style>
