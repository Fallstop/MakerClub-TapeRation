<script lang="ts">
	import { serverConnect } from "$lib/api";
	import { onMount } from "svelte";
	import "./styles.scss";
    import NoiseBackground from "$lib/components/NoiseBackground.svelte";

	onMount(() => {
		serverConnect();
	});
</script>

<div class="app">
	<slot />
	<NoiseBackground/>
</div>

<style>
	.app {
		position: relative;
		display: flex;
		flex-direction: column;
		min-height: 100vh;
		text-align: center;
		padding: 0 4rem;
	}
</style>
