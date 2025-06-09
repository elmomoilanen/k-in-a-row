<div class="container">
    <div class="spinner"></div>
    <div class="spinner inner-2"></div>
    <div class="spinner outer-2"></div>
    <div class="spinner outer-1"></div>
    <span class="spinner-text">Loading...</span>
</div>

<style>
    .container {
        position: fixed;
        display: flex;
        flex-direction: column;
        align-items: center;
        text-align: center;
        top: 15vh;
        left: 0;
        right: 0;
    }
    .spinner {
        border: 3px solid #0f7a1b;
        width: 200px;
        height: 200px;
        margin: 0 auto;
        border-radius: 50%;
        border-left-color: transparent;
        border-right-color: transparent;
        animation: rotate 2s cubic-bezier(0.26, 1.36, 0.74, -0.29) infinite;
    }
    .spinner.inner-2 {
        border: 3px solid #1a1d1a;
        width: 220px;
        height: 220px;
        position: relative;
        top: -216px;
        border-left-color: transparent;
        border-right-color: transparent;
        animation: rotate2 2s cubic-bezier(0.26, 1.36, 0.74, -0.29) infinite;
    }
    .spinner.outer-2 {
        border: 3px solid #03ab17;
        width: 240px;
        height: 240px;
        position: relative;
        top: -452px;
        border-left-color: transparent;
        border-right-color: transparent;
        animation: rotate 2s cubic-bezier(0.26, 1.36, 0.74, -0.29) infinite;
    }
    .spinner.outer-1 {
        border: 3px solid #525352;
        width: 260px;
        height: 260px;
        position: relative;
        top: -708px;
        border-left-color: transparent;
        border-right-color: transparent;
        animation: rotate2 2s cubic-bezier(0.26, 1.36, 0.74, -0.29) infinite;
    }
    .spinner-text {
        color: black;
        font-family: Arial;
        font-size: 15px;
        position: relative;
        top: -857px;
    }
    @keyframes rotate {
        0% {
            transform: rotateZ(-360deg);
        }
        100% {
            transform: rotateZ(0deg);
        }
    }
    @keyframes rotate2 {
        0% {
            transform: rotateZ(360deg);
        }
        100% {
            transform: rotateZ(0deg);
        }
    }
    @media (prefers-color-scheme: dark) {
        .spinner.inner-2 {
            border: 3px solid #aec0b1;
            border-left-color: transparent;
            border-right-color: transparent;
        }
        .spinner.outer-1 {
            border: 3px solid #d5d8d6;
            border-left-color: transparent;
            border-right-color: transparent;
        }
        .spinner-text {
            color: #f2f5f2;
        }
    }
</style>
