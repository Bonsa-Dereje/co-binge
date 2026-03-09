<script>
    let userName = '';
    let deviceId = '01D4TH879';
    let isToggleOn = true;

    function handleCreateAccount() {
        console.log('Creating account for:', userName);
    }
</script>

<div class="container">
    <div class="card">
        <!-- Toggle Switch -->
        <div class="toggle-container">
            <button 
                class="toggle" 
                class:active={isToggleOn}
                on:click={() => isToggleOn = !isToggleOn}
                aria-label="Toggle switch"
            >
                <div class="toggle-circle"></div>
            </button>
        </div>

        <!-- Avatar Image -->
        <div class="avatar-container">
            <img src="/avatars/girlAvatar.png" alt="Avatar" class="avatar-img" />
        </div>

        <!-- Username Input -->
        <div class="input-container">
            <input 
                type="text" 
                bind:value={userName}
                placeholder="user name"
                class="username-input"
            />
        </div>

        <!-- Device ID -->
        <p class="device-id">device id: {deviceId}</p>

        <!-- Create Account Button -->
        <button class="create-button" on:click={handleCreateAccount}>
            Create Account
        </button>
    </div>
</div>

<style>
    /* Page-specific adjustments */
    .avatar-img {
        width: 120px;
        height: auto;
        border-radius: 50%;
        display: block;
        margin: 0 auto;
    }

    .toggle-container {
        margin-bottom: 20px;
        display: flex;
        justify-content: center;
    }

    .toggle {
        width: 50px;
        height: 25px;
        border-radius: 25px;
        background-color: #ccc;
        border: none;
        position: relative;
        cursor: pointer;
        transition: background-color 0.3s;
    }

    .toggle.active {
        background-color: #212121;
    }

    .toggle-circle {
        width: 23px;
        height: 23px;
        background-color: white;
        border-radius: 50%;
        position: absolute;
        top: 1px;
        left: 1px;
        transition: transform 0.3s;
    }

    .toggle.active .toggle-circle {
        transform: translateX(25px);
    }

    .username-input {
        width: 100%;
        padding: 8px;
        font-family: "DM Mono", monospace;
        font-size: 16px;
        border: 1px solid #ccc;
        border-radius: 5px;
        margin-bottom: 15px;
    }

    .device-id {
        font-family: "DM Mono", monospace;
        font-size: 14px;
        text-align: center;
        color: #555;
        margin-bottom: 15px;
    }

    .create-button {
        width: 100%;
        padding: 10px;
        background-color: #212121;
        color: white;
        font-family: "DM Mono", monospace;
        font-size: 16px;
        border: none;
        border-radius: 25px;
        cursor: pointer;
        transition: background-color 0.3s;
    }

    .create-button:hover {
        background-color: #333;
    }
</style>