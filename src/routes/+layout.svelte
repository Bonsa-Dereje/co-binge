<script>
	import favicon from '$lib/assets/favicon.svg';
	import '../style.css';
	import { goto } from '$app/navigation';

	let { children } = $props();

	function goBack() {
		history.back();
	}

	function goHome() {
		goto('/hostJoin');
	}

	function goGroups() {
		goto('/groupsList');
	}
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<!-- Bottom Navigation -->
<div class="bottom-nav">

	<button class="nav-btn" on:click={goBack}>
		<!-- outline back -->
		<svg viewBox="0 0 24 24" width="24" height="20" fill="none" stroke="currentColor" stroke-width="2">
			<path d="M15 18l-6-6 6-6"/>
		</svg>
	</button>

	<button class="nav-btn home" on:click={goHome}>
		<!-- outline home -->
		<svg viewBox="0 0 24 24" width="26" height="15" fill="none" stroke="currentColor" stroke-width="2">
			<path d="M3 11l9-7 9 7"/>
			<path d="M5 10v10h14V10"/>
		</svg>
	</button>

	<button class="nav-btn" on:click={goGroups}>
		<!-- outline groups -->
		<svg viewBox="0 0 24 24" width="26" height="20" fill="none" stroke="currentColor" stroke-width="2">
			<circle cx="9" cy="8" r="3"/>
			<circle cx="17" cy="10" r="3"/>
			<path d="M2 20c0-3 4-5 7-5"/>
			<path d="M22 20c0-3-4-5-7-5"/>
		</svg>
	</button>

</div>

{@render children()}

<style>

.bottom-nav{
	background-color: #121212;
	color: white;
	position: fixed;
	bottom: 12px;
	left: 50%;
	transform: translateX(-50%);

	display: flex;
	align-items: center;
	justify-content: center;

	gap: 32px;

	border-radius: 999px;  /* pill shape */
	padding: 7px 22px;

	z-index: 1000;
}




/* icon buttons */
.nav-btn{
	
	background: transparent;
	border: none;
	padding: 0;
	cursor: pointer;

	display: flex;
	align-items: center;
	justify-content: center;

	color: inherit;
}

/* slightly larger home icon */
.home svg{
	width: 28px;
	height: 20px;
}

</style>