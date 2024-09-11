<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { open, save } from '@tauri-apps/api/dialog';
  import FileIcon from '../assets/file.svg';
  import Button from '$lib/components/ui/button/button.svelte';

  async function compress() {
    const files = await open({
      multiple: true,
      title: 'Выберите файл',
    });

    if (!files) {
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
      files: files,
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

  async function list() {
    const file = await open({
      multiple: false,
      filters: [
        {
          extensions: ['zip'],
          name: 'Архив',
        },
      ],
    });

    if (!file) return;

    const result: Array<string> = await invoke('contents', { filepath: file });
    contents = result;
  }

  let contents: Array<string> = [];
</script>

<div class="size-full">
  <div class="flex items-center gap-x-1 border-b p-1">
    <Button variant="ghost" size="sm" on:click={compress}>Сжать</Button>
    <Button variant="ghost" size="sm" on:click={decompress}
      >Разархивировать</Button
    >
    <Button variant="ghost" size="sm" on:click={list}>Открыть архив</Button>
  </div>

  {#if contents.length}
    <div class="space-y-2 p-1">
      {#each contents as file}
        <div class="flex items-center">
          <img
            class="pointer-events-none"
            src={FileIcon}
            width={20}
            height={20}
            alt="File"
          />
          {file}
        </div>
      {/each}
    </div>
  {/if}
</div>
