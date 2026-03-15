<script>
    let repoLink = "https://github.com/Bonsa-Dereje/co-binge";

    function openRepo() {
        window.open(repoLink, "_blank");
    }
</script>

<div class="page-wrapperNF">

    <div class="warning-wrapperNF">

        <!-- Warning Icon -->
        <div class="warning-iconNF">
            ⚠
        </div>

        <!-- Message -->
        <div class="warning-textNF">
            Netflix support will rollout soon
        </div>

        <div class="warning-subtextNF">
            Feel free to contribute
        </div>

        <!-- Link -->
        <button class="repo-linkNF" on:click={openRepo}>
            github.com/Bonsa-Dereje/co-binge
        </button>

    </div>

</div>

<style src="./netflixPage.css"></style>