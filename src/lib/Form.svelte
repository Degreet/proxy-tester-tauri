<script>
  export let form;
</script>

<form class="form">
  <div class="form__input-block">
    <label for="ip" class="form__input-label">
      Enter proxy IP
      <span class="required">*</span>
    </label>
    <input type="text" class="form__input" placeholder="127.0.0.1" bind:value={form.ip} name="ip" id="ip" />
  </div>
  <div class="form__input-block">
    <label for="port" class="form__input-label">
      Enter proxy HTTP port
      <span class="required">*</span>
    </label>
    <input type="number" class="form__input" placeholder="3257" bind:value={form.port} name="port" id="port" />
  </div>
  <div class="form__input-block">
    <label for="username" class="form__input-label">Enter proxy username</label>
    <input
      type="text"
      class="form__input"
      placeholder="admin"
      bind:value={form.username}
      name="username"
      id="username"
    />
  </div>
  <div class="form__input-block">
    <label for="password" class="form__input-label">Enter proxy password</label>
    <input
      type="password"
      class="form__input"
      placeholder="*****"
      bind:value={form.password}
      name="password"
      id="password"
    />
  </div>
  <div class="form__input-block">
    <label for="url" class="form__input-label">
      Proxy check by URL
      <span class="required">*</span>
    </label>
    <input
      type="text"
      class="form__input"
      placeholder="https://example.com/"
      bind:value={form.url}
      name="url"
      id="url"
    />
  </div>
</form>

<style>
  .form {
    margin-top: 40px;
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--default-gap);
  }

  .form__input-block {
    display: flex;
    flex-direction: column;
    gap: 8px;

    & .required {
      color: var(--danger-color);
    }

    &:last-child {
      grid-column: 1 / -1;
    }
  }

  .form__input {
    background: transparent;
    border: 1.5px solid var(--secondary-color);
    border-radius: var(--default-border-radius);
    height: 37px;
    padding: 9px;
    color: var(--text-color);
    transition: 0.5s;

    font-family: 'JetBrains Mono', sans-serif;
    font-size: var(--default-size);

    &::-webkit-outer-spin-button,
    &::-webkit-inner-spin-button {
      -webkit-appearance: none;
      margin: 0;
    }

    &::placeholder {
      color: var(--secondary-color);
    }
  }
</style>
