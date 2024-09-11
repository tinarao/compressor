<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { open, save } from '@tauri-apps/api/dialog';

  async function compress() {
    const file = await open({
      multiple: false,
      title: 'Выберите файл',
    });

    if (!file) {
      return;
    }

    const to = await save({
      title: 'Выберите, куда сохранить архив',
      filters: [
        {
          name: 'Архив',
          extensions: ['zip'],
        },
      ],
    });

    const result = await invoke('compress', {
      targetPath: file,
      outputName: to,
    });

    console.log(result);
  }

  async function decompress() {
    const file = await open({
      multiple: false,
      title: 'Выберите файл',
      filters: [
        {
          extensions: ['zip'],
          name: 'Архив',
        },
      ],
    });

    if (!file) {
      return;
    }

    const to = await save({
      title: 'Выберите, куда сохранить архив',
    });

    const result = await invoke('unzip', {
      archivePath: file,
      targetDir: to,
    });

    console.log(result);
  }
</script>

<div class="container">
  <button class="button" on:click={compress}>Сжать</button>
  <button class="button" on:click={decompress}>Разархивировать</button>
</div>

<style>
  .container {
    height: 100vh;
    width: 100vw;

    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.3rem;
  }
  .button {
    width: 50%;
    padding: 0.4rem 0.8rem;
    border-radius: 0.4rem;
    border: none;
    background: black;
    font-weight: 500;
    color: white;
    cursor: pointer;

    &:hover {
      background: rgb(46, 46, 46);
    }
  }
</style>
