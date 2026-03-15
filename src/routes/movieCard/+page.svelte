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
                {movie.title}
            </div>

            <div class="movie-metaMC">
                {movie.rating} • {movie.date}
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
            user name
        </div>

        <div class="profile-deviceMC">
            device id: {deviceId}
        </div>

    </div>

</div>

<style src="./movieCard.css"></style>