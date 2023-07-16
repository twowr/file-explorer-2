<script setup>

import { sep } from "@tauri-apps/api/path"
import { readDir } from "@tauri-apps/api/fs"
import sidebar from "./components/Sidebar.vue"
import fileView from "./components/FileView.vue"
import topbar from "./components/Topbar.vue"
import { ref, watch } from "vue"

const path = ref("D:\\project\\software-projects\\file_explorer_2")

function on_back() {
    let segments = path.value.split(sep)

    if (segments.length > 1) {
        segments.pop()
        path.value = segments.join(sep)
        if (segments.length == 1) {
            path.value += sep
        }
    }
}

function on_search(keyword) {

}

function open(dir, isDirectory) {
    if (isDirectory) {
        path.value = dir
    }
}

//initialize entries and update entries whenever the value path change
const entries = ref(null)

readDir(path.value, { recursive: false }).then((result) => {
    //put sorted folders first then sorted files
    entries.value = result.filter(entry => entry.children != null).sort().concat(result.filter(entry => entry.children == null).sort())
})

watch(path, () => {
    readDir(path.value, { recursive: false }).then((result) => {
        //put sorted folders first then sorted files
        entries.value = result.filter(entry => entry.children != null).sort().concat(result.filter(entry => entry.children == null).sort())
    })
})

</script>

<template>
    <div class="container">
        <sidebar class="sidebar" @back="on_back"/>
        <div class="viewPanel">
            <topbar class="topbar" @open="open" @search="on_search" :dir="path"/>
            <Suspense>
                <fileView class="fileView" @open="open" :entries="entries"/>
            </Suspense>
        </div>
    </div>
</template>

<style scoped>

</style>