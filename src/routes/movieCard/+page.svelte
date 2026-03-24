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

    let unsubSearch: () => void;
    let unsubUser: () => void;
    let unsubDevice: () => void;

    onMount(() => {
        // animate in
        requestAnimationFrame(() => {
            entering = false;
        });

        // subscribe to search keyword
        unsubSearch = searchKeyword.subscribe(async (value) => {
            keyword = value;
            if (keyword && keyword.trim().length > 0) {
                await fetchMovie(keyword);
            }
        });

        // subscribe to global username
        unsubUser = globalUserName.subscribe((value) => {
            username = value;
        });

        // subscribe to global device id
        unsubDevice = globalDeviceId.subscribe((value) => {
            deviceId = value;
        });
    });

    onDestroy(() => {
        unsubSearch?.();
        unsubUser?.();
        unsubDevice?.();
    });

    async function fetchMovie(query: string) {
        try {
            const res = await fetch(
                `https://api.themoviedb.org/3/search/movie?api_key=${API_KEY}&query=${encodeURIComponent(query)}`
            );
            const data = await res.json();

            if (data.results && data.results.length > 0) {
                const best = data.results[0];

                movie = {
                    title: best.title,
                    rating: best.vote_average
                        ? `${best.vote_average}/10`
                        : "N/A",
                    date: best.release_date
                        ? best.release_date
                        : "Unknown",
                    description: best.overview || "No description available.",
                    poster: best.poster_path
                        ? `https://image.tmdb.org/t/p/w500${best.poster_path}`
                        : "/posters/default-fallback-image.png"
                };
            }
        } catch (err) {
            console.error("TMDB fetch error:", err);
        }
    }

    function toggleDarkMode() {
        darkMode = !darkMode;
        document.body.classList.toggle("dark-mode", darkMode);
    }
</script>

<div class="page-wrapperMC" class:entering={entering}>

    <!-- DARK MODE TOGGLE -->
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
            <img src={movie.poster} alt="movie poster" class="poster-imgMC">
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