<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { globalUserName, globalDeviceId } from '$lib/stores/user';
    import { isVideoStore } from '$lib/stores/isVideo';
    import { listen } from "@tauri-apps/api/event";

    let entering = true;
    let darkMode = false;
    let username = '';
    let deviceId = '';

    let dragging = false;
    let isVideo = true;

    // ✅ NEW TEXT STATE
    let dropMessage = "drag and drop";
    let isError = false;

    let hasInitialized = false;

    let unsubscribeUser: () => void;
    let unsubscribeDevice: () => void;
    let unsubscribeVideo: () => void;

    let unlistenEnter: () => void;
    let unlistenOver: () => void;
    let unlistenLeave: () => void;
    let unlistenDrop: () => void;

    onMount(async () => {
        requestAnimationFrame(() => {
            entering = false;
        });

        // ✅ AUTO TRIGGER AFTER 1 SECOND
        setTimeout(() => {
            console.log("Simulating file drop → setting isVideo = true");
            isVideoStore.set(true);
        }, 1000);

        unsubscribeUser = globalUserName.subscribe(value => {
            username = value || 'user name';
        });

        unsubscribeDevice = globalDeviceId.subscribe(value => {
            deviceId = value || '01D4TH879';
        });

        unsubscribeVideo = isVideoStore.subscribe(value => {
            if (!hasInitialized) {
                isVideo = value;
                hasInitialized = true;
                return;
            }

            if (value === false) {
                showUnsupportedMessage();
            }

            if (value === true) {
                isError = false;
                dropMessage = "drag and drop";
            }

            isVideo = value;
        });

        window.addEventListener("dragover", preventDefault);
        window.addEventListener("drop", preventDefault);

        unlistenEnter = await listen("tauri://drag-enter", () => {
            dragging = true;
        });

        unlistenOver = await listen("tauri://drag-over", () => {
            dragging = true;
        });

        unlistenLeave = await listen("tauri://drag-leave", () => {
            dragging = false;
        });

        unlistenDrop = await listen<string[]>("tauri://drag-drop", (event) => {
            dragging = false;

            const files = event.payload;
            if (!files || files.length === 0) return;

            const filePath = files[0];
            handleFile(filePath);
        });

        await listen<boolean>('file:isVideo', (event) => {
            isVideoStore.set(event.payload);
        });
    });

    onDestroy(() => {
        unsubscribeUser && unsubscribeUser();
        unsubscribeDevice && unsubscribeDevice();
        unsubscribeVideo && unsubscribeVideo();

        unlistenEnter && unlistenEnter();
        unlistenOver && unlistenOver();
        unlistenLeave && unlistenLeave();
        unlistenDrop && unlistenDrop();

        window.removeEventListener("dragover", preventDefault);
        window.removeEventListener("drop", preventDefault);
    });

    function preventDefault(e: DragEvent) {
        e.preventDefault();
    }

    function toggleDarkMode() {
        darkMode = !darkMode;
        document.body.classList.toggle("dark-mode", darkMode);
    }

    function showUnsupportedMessage() {
        isError = true;
        dropMessage = "unsupported file";
    }

    function handleDragEnter(e: DragEvent) {
        e.preventDefault();
        dragging = true;
    }

    function handleDragLeave(e: DragEvent) {
        e.preventDefault();
        dragging = false;
    }

    function handleDragOver(e: DragEvent) {
        e.preventDefault();
    }

    function handleDrop(e: DragEvent) {
        e.preventDefault();
        dragging = false;

        const files = e.dataTransfer?.files;
        if (!files || files.length === 0) return;

        const file = files[0];
        handleFile(file);
    }

    function handleFile(file: any) {
        if (typeof file === "string") {
            console.log("Handling Tauri path:", file);
        } else {
            console.log("Handling browser file:", file.name);
        }
    }
</script>

<div class="page-wrapper" class:entering={entering}>
    <div class="card">

        <!-- Toggle -->
        <div class="toggle-wrapper">
            <button
                class="toggle"
                class:active={darkMode}
                on:click={toggleDarkMode}
                aria-label="Toggle dark mode"
            >
                <div class="toggle-circle"></div>
            </button>
        </div>

        <!-- Drag Drop -->
        <div
            class="drop-zone"
            class:dragging={dragging}
            class:error={isError}
            on:dragenter={handleDragEnter}
            on:dragleave={handleDragLeave}
            on:dragover={handleDragOver}
            on:drop={handleDrop}
        >
            <svg viewBox="0 0 24 24" class="folder-icon">
                <path d="M3 7h6l2 2h10v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"
                fill="none" stroke="currentColor" stroke-width="1.5"/>
                <path d="M3 7V5a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2"
                fill="none" stroke="currentColor" stroke-width="1.5"/>
            </svg>

            <div class="drop-text">
                {#if dragging}
                    drop file here
                {:else}
                    {dropMessage}
                {/if}
            </div>
        </div>

        <!-- Host -->
        <div class="host-wrapper">
            <button class="host-button">Host</button>
        </div>

        <!-- Profile -->
        <div class="profile-wrapperDrag">
            <img src="/avatars/girlAvatar.png" alt="avatar" class="profile-avatar"/>
            <div class="profile-nameDrag">{username}</div>
            <div class="profile-deviceDrag">device id: {deviceId}</div>
        </div>

    </div>
</div>

<style src="./dragDrop.css"></style>