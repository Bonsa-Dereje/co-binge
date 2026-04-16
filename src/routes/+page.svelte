<script>
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';

    onMount(() => {
        const timer = setTimeout(() => {
            goto('/hostJoin');
        }, 3000);

        return () => clearTimeout(timer);
    });
</script>

<div class="container">
    <div class="title">coBinge</div>
</div>

<style src="../style.css"></style>