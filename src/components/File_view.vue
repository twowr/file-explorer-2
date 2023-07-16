<script setup>
import { ref, watch, computed } from "vue"
import { readDir } from "@tauri-apps/api/fs"

const props = defineProps(["dir"])

const emit = defineEmits(["open"])

const path = computed(() => {
    return props.dir
})

const entries = ref(null)
readDir(path.value, { recursive: false }).then((result) => {
    //put sorted folders first then sorted files
    entries.value = result.filter(entry => entry.children != null).sort().concat(result.filter(entry => entry.children == null).sort())
})

watch(path, () => {
    readDir(props.dir, { recursive: false }).then((result) => {
        //put sorted folders first then sorted files
        entries.value = result.filter(entry => entry.children != null).sort().concat(result.filter(entry => entry.children == null).sort())
        let input = document.getElementsByClassName("address")[0]
        input.focus()
        input.setSelectionRange( input.value.length, input.value.length )
    })
})
</script>

<template>
    <div class="entry" @click="emit('open', entry.path, entry.children != null)" v-if="entries != null" v-for="entry in entries">
        <img v-if="entry.children != null" src="../assets/folder_icon.png" alt="folder">
        <img v-else src="../assets/file_icon.png" alt="file">
        <span class="entry_name">{{ entry.name }}</span>
    </div>
</template>