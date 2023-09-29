<script>
  export let isCorrectOptions;
</script>

<button class="button" class:success={isCorrectOptions} on:click>Check</button>

<style>
  .button {
    font-family: 'JetBrains Mono', sans-serif;
    font-size: var(--default-size);

    background: var(--danger-color);
    color: white;
    padding: 9px 0;
    border: none;
    border-radius: var(--default-border-radius);
    cursor: pointer;
    transition: 0.5s;

    &.success {
      background: var(--success-color);
    }
  }
</style>
