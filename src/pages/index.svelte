<script lang="ts">
  import { promisified, invoke, transformCallback } from 'tauri/api/tauri';
  import { open, save } from 'tauri/api/dialog';
  import { createDir, readDir, readTextFile } from 'tauri/api/fs';
  import { emit, listen } from 'tauri/api/event';
  import { sendNotification } from 'tauri/api/notification';
  import { beforeUrlChange, afterPageLoad, goto } from '@sveltech/routify'
  let demoPath: string = "";
  let demo;
  let submitted: boolean = false;

  const browseDemo = async () => {
    demoPath = (await open()).toString();
  }
  const importDemo = async () => {
    submitted = true;
    demo = await promisified({cmd: "loadDemo", path: demoPath });
    
  }
</script>

<style>
  .index {
    text-align: center;
  }
  h1 {
    margin: 0;
    padding: 100px 20px 100px 20px;
    color: #110011;
    font-weight: 300;
  }  
</style>

<div class="index">
  <h1>impresys utils</h1>

  <br/>
  <input bind:value={demoPath}/>
  <button on:click={browseDemo}>Browse demo</button>
  <br/><br/>
  <p> submitted: {submitted}</p>
  <p> demoPath: { demoPath }  </p>
  <button on:click={importDemo}>Load</button>
</div>
