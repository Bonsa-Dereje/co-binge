<script lang="ts">
    import { onMount } from "svelte";
    import { globalUserName, globalDeviceId } from '$lib/stores/user';
    import { ytLink } from '$lib/stores/ytLink';
    import { PUBLIC_YOUTUBE_API_KEY } from '$env/static/public';

    let entering = true;
    let darkMode = false;

    let username = '';
    let deviceId = '';

    let link = '';
    let videoId = '';

    let thumbnail = '';
    let title = '';
    let views = '';
    let date = '';

    function extractVideoId(url: string) {
        const regExp = /(?:youtube\.com\/watch\?v=|youtu\.be\/)([^&\n?#]+)/;
        const match = url.match(regExp);
        return match ? match[1] : '';
    }

    function formatViews(count: number) {
        if (count >= 1_000_000) {
            return (count / 1_000_000).toFixed(1).replace(/\.0$/, '') + 'M';
        }
        if (count >= 1_000) {
            return (count / 1_000).toFixed(1).replace(/\.0$/, '') + 'K';
        }
        return count.toString();
    }

    async function loadVideoData(id: string) {
        if (!id) return;

        thumbnail = `https://img.youtube.com/vi/${id}/maxresdefault.jpg`;

        try {
            const res = await fetch(
                `https://www.googleapis.com/youtube/v3/videos?part=snippet,statistics&id=${id}&key=${PUBLIC_YOUTUBE_API_KEY}`
            );

            const data = await res.json();
            const video = data.items?.[0];

            if (!video) throw new Error("No video");

            title = video.snippet.title;

            // Format views (K, M)
            const rawViews = Number(video.statistics.viewCount);
            views = formatViews(rawViews);

            // Format date
            const published = new Date(video.snippet.publishedAt);
            date = published.toLocaleDateString(undefined, {
                year: 'numeric',
                month: 'short',
                day: 'numeric'
            });

        } catch (e) {
            title = "Video unavailable";
            views = "—";
            date = "—";
        }
    }

    onMount(() => {
        requestAnimationFrame(() => {
            entering = false;
        });

        const unsubUser = globalUserName.subscribe(v => {
            username = v || 'user name';
        });

        const unsubDevice = globalDeviceId.subscribe(v => {
            deviceId = v || '01D4TH879';
        });

        const unsubLink = ytLink.subscribe(v => {
            link = v;

            videoId = extractVideoId(link);
            loadVideoData(videoId);
        });

        return () => {
            unsubUser();
            unsubDevice();
            unsubLink();
        };
    });

    function toggleDarkMode() {
        darkMode = !darkMode;
        document.body.classList.toggle("dark-mode", darkMode);
    }
</script>

<div class="page-wrapper ytCard" class:entering={entering}>
    <div class="card">

        <!-- Toggle -->
        <div class="toggle-wrapper">
            <button
                class="toggle"
                class:active={darkMode}
                on:click={toggleDarkMode}
            >
                <div class="toggle-circle"></div>
            </button>
        </div>

        <!-- Thumbnail -->
        <div class="thumbnail-card">
            {#if thumbnail}
                <img src={thumbnail} alt="video thumbnail" class="thumbnail-img">
            {:else}
                <div class="thumbnail-placeholder"></div>
            {/if}
        </div>

        <!-- Title -->
        <div class="yt-title">
            {title || "Loading..."}
        </div>

        <!-- Meta -->
        <div class="yt-meta">

            <div class="yt-meta-item">
                <svg viewBox="0 0 24 24" class="meta-icon">
                    <path d="M2 12s4-6 10-6 10 6 10 6-4 6-10 6S2 12 2 12z"
                        fill="none" stroke="currentColor" stroke-width="1.5"/>
                    <circle cx="12" cy="12" r="3"
                        fill="none" stroke="currentColor" stroke-width="1.5"/>
                </svg>
                {views || "—"}
            </div>

            <div class="yt-meta-item">
                <svg viewBox="0 0 24 24" class="meta-icon">
                    <rect x="3" y="4" width="18" height="18"
                        fill="none" stroke="currentColor" stroke-width="1.5"/>
                    <path d="M8 2v4M16 2v4M3 10h18"
                        fill="none" stroke="currentColor" stroke-width="1.5"/>
                </svg>
                {date || "—"}
            </div>

        </div>

        <!-- Host -->
        <div class="host-wrapper">
            <button class="host-button">Host</button>
        </div>

        <!-- Profile -->
        <div class="profile-wrapperYT">
            <img src="/avatars/girlAvatar.png" class="profile-avatarDrag"/>
            <div class="profile-nameYT">{username}</div>
            <div class="profile-deviceYT">device id: {deviceId}</div>
        </div>

    </div>
</div>

<style src="../style.css"></style>