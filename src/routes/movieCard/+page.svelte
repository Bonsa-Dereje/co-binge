<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { searchKeyword } from "$lib/stores/searchKeyword";
    import { globalUserName, globalDeviceId } from "$lib/stores/user";

    let entering = true;
    let darkMode = false;

    let keyword = "";
    let username = "";
    let deviceId = "";

    let movie = {
        title: "",
        rating: "",
        date: "",
        description: "",
        poster: ""
    };

    const API_KEY = import.meta.env.VITE_TMDB_API_KEY;

    let results: any[] = [];
    let currentIndex = 0;

    let imageLoaded = false;

    let unsubSearch: () => void;
    let unsubUser: () => void;
    let unsubDevice: () => void;

    onMount(() => {
        requestAnimationFrame(() => {
            entering = false;
        });

        unsubSearch = searchKeyword.subscribe(async (value) => {
            keyword = value;
            if (keyword && keyword.trim().length > 0) {
                await fetchMovies(keyword);
            }
        });

        unsubUser = globalUserName.subscribe((value) => {
            username = value;
        });

        unsubDevice = globalDeviceId.subscribe((value) => {
            deviceId = value;
        });
    });

    onDestroy(() => {
        unsubSearch?.();
        unsubUser?.();
        unsubDevice?.();
    });

    async function fetchMovies(query: string) {
        try {
            const res = await fetch(
                `https://api.themoviedb.org/3/search/movie?api_key=${API_KEY}&query=${encodeURIComponent(query)}`
            );
            const data = await res.json();

            if (data.results && data.results.length > 0) {
                results = data.results;
                currentIndex = 0;
                setMovie(results[currentIndex]);
            }
        } catch (err) {
            console.error("TMDB fetch error:", err);
        }
    }

    function setMovie(item: any) {
        imageLoaded = false;

        movie = {
            title: item.title,
            rating: item.vote_average ? `${item.vote_average}/10` : "N/A",
            date: item.release_date || "Unknown",
            description: item.overview || "No description available.",
            poster: item.poster_path
                ? `https://image.tmdb.org/t/p/w500${item.poster_path}`
                : "/posters/default-fallback-image.png"
        };
    }

    function nextMovie() {
        if (results.length === 0) return;
        currentIndex = (currentIndex + 1) % results.length;
        setMovie(results[currentIndex]);
    }

    function prevMovie() {
        if (results.length === 0) return;
        currentIndex =
            (currentIndex - 1 + results.length) % results.length;
        setMovie(results[currentIndex]);
    }

    function toggleDarkMode() {
        darkMode = !darkMode;
        document.body.classList.toggle("dark-mode", darkMode);
    }
</script>

<div class="page-wrapperMC" class:entering={entering}>

    <!-- DARK MODE -->
    <div class="toggle-wrapperMC">
        <button
            class="toggleMC"
            class:active={darkMode}
            on:click={toggleDarkMode}
        >
            <div class="toggle-circleMC"></div>
        </button>
    </div>

    <!-- MOVIE CARD -->
    <div class="movie-cardMC">

        <div class="poster-wrapperMC">
            {#if !imageLoaded}
                <div class="poster-loaderMC"></div>
            {/if}

            <img
                src={movie.poster}
                alt="movie poster"
                class="poster-imgMC"
                on:load={() => (imageLoaded = true)}
                style:opacity={imageLoaded ? 1 : 0}
            >
        </div>

        <div class="movie-infoMC">

            <div class="movie-titleMC">
                {movie.title || "Search for a movie"}
            </div>

            <div class="movie-metaMC">
                {movie.rating} {movie.date ? `• ${movie.date}` : ""}
            </div>

            <div class="movie-descriptionMC">
                {movie.description}
            </div>

            <button class="movieHostMC">
                Host
            </button>

        </div>
    </div>

<!-- NAVIGATION BUTTONS -->
<div class="nav-buttonsMC">
    <button on:click={prevMovie}>&lt;</button>
    <button on:click={nextMovie}>&gt;</button>
</div>

    <!-- PROFILE -->
    <div class="profile-wrapperMC">

        <img
            src="/avatars/girlAvatar.png"
            alt="avatar"
            class="profile-avatarMC"
        />

        <div class="profile-nameMC">
            {username || "Guest"}
        </div>

        <div class="profile-deviceMC">
            device id: {deviceId || "N/A"}
        </div>

    </div>

</div>

<style src="./movieCard.css"></style>