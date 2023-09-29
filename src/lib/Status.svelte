<script>
    export let isLoading, isSuccess, message;
</script>

<p class="status" class:loading={isLoading}>
    {#if !isLoading}
        <span>Status:</span>
        <span
                class="status__info"
                class:success={isSuccess}
        >{message}</span>
    {:else}
        <span>Checking...</span>
    {/if}
</p>

<style>
    .status {
        font-size: var(--default-size);

        display: flex;
        align-items: center;
        gap: 7px;

        &.loading {
            color: var(--info-color);
        }

        & .status__info {
            color: var(--danger-color);

            &.success {
                color: var(--success-color);
            }
        }
    }
</style>
