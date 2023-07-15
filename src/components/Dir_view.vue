<script setup>
import { ref, onMounted, watch, computed } from "vue"
import { readDir } from "@tauri-apps/api/fs"

const props = defineProps(["dir"])

const emit = defineEmits(["open", "back", "submit"])

const path = computed(() => {
    return props.dir
})

onMounted(() => {
    let el = document.getElementsByClassName("address")[0];
    el.addEventListener("keydown", function(event) {
        if (event.key === "Enter") {
            emit("submit", el.value, true)
        }
    })
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
    })
})
</script>

<template>
    <div class="topbar">
        <button class="back_button" @click="emit('back')">back</button>
        <input class="address" :value="dir"/>
    </div>
    <div class="entry" @click="emit('open', entry.path, entry.children != null)" v-if="entries != null" v-for="entry in entries">
        <img v-if="entry.children != null" src="../assets/folder_icon.png" alt="folder">
        <img v-else src="../assets/file_icon.png" alt="file">
        <span class="entry_name">{{ entry.name }}</span>
    </div>
</template>