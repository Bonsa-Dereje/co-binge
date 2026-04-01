<script lang="ts">
    import { onMount } from "svelte";
    import { listen } from "@tauri-apps/api/event";

    let videoPath = "";

    onMount(() => {
        const unlisten = listen("file:videoPath", (event: any) => {
            videoPath = event.payload;
        });

        return () => {
            unlisten.then(f => f());
        };
    });
</script>

<div class="loading-container">
    <div class="spinner"></div>
    <p class="text">Opening video...</p>

    {#if videoPath}
        <p class="path">{videoPath}</p>
    {/if}
</div>

<style>
.loading-container {
    position: fixed;
    inset: 0;
    background: #0f0f0f;

    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;

    z-index: 999;
}

.spinner {
    width: 50px;
    height: 50px;

    border: 4px solid rgba(255,255,255,0.1);
    border-top: 4px solid #ffffff;

    border-radius: 50%;
    animation: spin 1s linear infinite;
}

.text {
    margin-top: 20px;
    color: #fff;
    font-size: 14px;
    opacity: 0.8;
}

.path {
    margin-top: 10px;
    color: #888;
    font-size: 11px;
    max-width: 80%;
    text-align: center;
    word-break: break-all;
}

@keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
}
</style>