<script lang="ts">
    import MovieCard from "./+page.svelte";

    let movieRef: any;

    function next() {
        movieRef?.nextMovie();
    }

    function prev() {
        movieRef?.prevMovie();
    }
</script>

<div class="overlay">

    <!-- MOVIE COMPONENT -->
    <MovieCard bind:this={movieRef} />

    <!-- NAV OVERLAY -->
    <div class="nav-overlay">
        <button class="nav-btn left" on:click={prev}></button>
        <button class="nav-btn right" on:click={next}></button>
    </div>

</div>

<style>
.overlay {
    position: fixed;
    inset: 0;
    z-index: 10;
}

/* NAV OVERLAY */
.nav-overlay {
    position: absolute;
    inset: 0;
    pointer-events: none;
}

/* BUTTONS */
.nav-btn {
    position: absolute;
    top: 50%;
    transform: translateY(-50%);
    width: 40px;
    height: 40px;
    background: transparent;
    border: none;
    cursor: pointer;
    pointer-events: auto;
    
}

/* minimalist arrow (line + angle) */
.nav-btn::before {
    content: "";
    display: block;
    width: 12px;
    height: 12px;
    border-top: 2px solid #aaa;
    border-right: 2px solid #aaa;
    margin: auto;
}

/* LEFT */
.left {
    left: 10px;
}
.left::before {
    transform: rotate(-135deg);
}

/* RIGHT */
.right {
    right: 10px;
}
.right::before {
    transform: rotate(45deg);
}
</style>