<script>
    import { goto } from '$app/navigation';

    let userName = '';
    let deviceId = '01D4TH879';
    let darkMode = false;
    let leaving = false;

    function handleCreateAccount() {
        console.log('Creating account for:', userName);

        leaving = true;

        // wait for animation to finish
        setTimeout(() => {
            goto('/hostJoin');
        }, 600);
    }

    function toggleDarkMode() {
        darkMode = !darkMode;
        document.body.classList.toggle('dark-mode', darkMode);
    }
</script>

<div class="page-wrapper" class:leaving={leaving}>

    <div class="card">

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

        <div class="avatar-wrapper">
            <img src="/avatars/girlAvatar.png" alt="Avatar" class="avatar" />
        </div>

        <div class="username-wrapper">
            <input 
                type="text"
                bind:value={userName}
                placeholder="user name"
                class="username-input"
            />
        </div>

        <div class="device-wrapper">
            <p class="device-id">device id: {deviceId}</p>
        </div>

        <div class="create-wrapper">
            <button 
                on:click={handleCreateAccount}
                class="create-button"
            >
                Create Account
            </button>
        </div>

    </div>

</div>

<style src="../style.css"></style>