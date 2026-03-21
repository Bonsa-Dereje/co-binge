<script lang="ts">
    import { onDestroy } from 'svelte';
    import { isVideoStore } from '$lib/stores/isVideo';
    import { fade, fly } from 'svelte/transition';

    let isVideo = true;

    const unsubscribe = isVideoStore.subscribe(value => {
        isVideo = value;
    });

    onDestroy(() => {
        unsubscribe();
    });
</script>

{#if !isVideo}
    <div class="overlay" in:fade out:fade>
        <div class="modal" in:fly={{ y: 40, duration: 200 }}>
            <h2>Unsupported File</h2>
            <p>Please drop a valid video file.</p>
        </div>
    </div>
{/if}

<style>
    .overlay {
        position: fixed;
        inset: 0;
        background: rgba(0, 0, 0, 0.65);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 9999; /* ensures it's above everything */
        backdrop-filter: blur(4px);
    }

    .modal {
        background: #1e1e1e;
        color: white;
        padding: 2rem 2.5rem;
        border-radius: 12px;
        box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);
        text-align: center;
        max-width: 320px;
        width: 90%;
        border: 1px solid rgba(255, 255, 255, 0.1);
    }

    h2 {
        margin-bottom: 0.5rem;
        font-size: 1.4rem;
    }

    p {
        font-size: 0.95rem;
        opacity: 0.8;
    }
</style>