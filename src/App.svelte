<script>
  import Form from './lib/Form.svelte';
  import CheckButton from './lib/CheckButton.svelte';
  import Status from './lib/Status.svelte';

  import { invoke } from '@tauri-apps/api';

  let form = {
    ip: null,
    port: null,
    username: null,
    password: null,
    url: 'https://google.com/',
  };

  let isLoading, isSuccess, message;
  $: isCorrectOptions = form.ip && form.port && !isNaN(form.port) && form.url && !isLoading;

  async function checkProxy() {
    if (!isCorrectOptions) return;
    isLoading = true;

    try {
      message = await invoke('check_proxy', { ...form });
      isSuccess = true;
    } catch (e) {
      console.error(e);
      message = e;
      isSuccess = false;
    } finally {
      isLoading = false;
    }
  }
</script>

<main class="container">
  <h1 class="title">Proxy Tester <span class="title__author">by Degreet</span></h1>

  <Form bind:form />

  <div class="result">
    <CheckButton on:click={checkProxy} {isCorrectOptions} />
    {#if message || isLoading}
      <Status {isLoading} {isSuccess} {message} />
    {/if}
  </div>
</main>

<style>
  .result {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: 37px;
    gap: var(--default-gap);
    margin-top: 35px;
  }
</style>
