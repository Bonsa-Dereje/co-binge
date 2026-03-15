<script>
    import { onMount } from "svelte";

    let entering = true;
    let darkMode = false;
    let deviceId = "01D4TH879";

    let movie = {
        title: "Iron Man",
        rating: "8.4/10",
        date: "1h 56m • 2008",
        description:
            "When Tony Stark, an industrialist, is captured, he constructs a high-tech armoured suit to escape. Once he manages to escape, he decides to use his suit to fight against evil forces to save the world.",
        poster: "/posters/ironman.jpg"
    };

    onMount(() => {
        requestAnimationFrame(() => {
            entering = false;
        });
    });

    function toggleDarkMode() {
        darkMode = !darkMode;
        document.body.classList.toggle("dark-mode", darkMode);
    }
</script>

<div class="page-wrapper" class:entering={entering}>

    <!-- TOP TOGGLE -->
    <div class="toggle-wrapperMC">
        <button
            class="toggle"
            class:active={darkMode}
            on:click={toggleDarkMode}
            aria-label="Toggle dark mode"
        >
            <div class="toggle-circle"></div>
        </button>
    </div>

    <!-- CENTER MOVIE CARD -->
    <div class="movie-card">

        <div class="poster-wrapper">
            <img src={movie.poster} alt="movie poster" class="poster-img">
        </div>

        <div class="movie-info">

            <div class="movie-title">
                {movie.title}
            </div>

            <div class="movie-meta">
                {movie.rating} • {movie.date}
            </div>

            <div class="movie-description">
                {movie.description}
            </div>

            <button class="movieHost">
                Host
            </button>

        </div>

    </div>

    <!-- BOTTOM PROFILE -->
    <div class="profile-wrapperMC">

        <img
            src="/avatars/girlAvatar.png"
            alt="avatar"
            class="profile-avatarMC"
        />

        <div class="profile-nameMC">
            user name
        </div>

        <div class="profile-deviceMC">
            device id: {deviceId}
        </div>

    </div>

</div>

<style src="./movieCard.css"></style>