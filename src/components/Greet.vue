<template>
    <form class="row" @submit.prevent="load_path">
        <button>Load recipes</button>
    </form>

    <p>{{ returnMessage }}</p>
    <small>{{ currentRecipe }}</small>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from '@tauri-apps/api/window'

import { open } from '@tauri-apps/api/dialog';

export default {
    name: "Greet",
    data() {
        return {
            returnMessage: "",
            currentRecipe: ""
        }
    },
    methods: {
        async load_path() {
            const selected = await open({
                directory: true,
                multiple: false,
            });
            if (selected === null) {
                // user cancelled the selection
                console.log("No directory selected!");
            } else {
                // user selected a single directory
                console.log(selected);
                await invoke("load_path", { dataPath: selected });
            }
        }
    },
    mounted() {
        appWindow.listen("loading://progress", (data) => {
            this.returnMessage = `Loading ${data.payload.progress}/${data.payload.total}`;
            this.currentRecipe = data.payload.path;
        });
    }
};
</script>
