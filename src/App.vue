<script setup>

import { sep } from "@tauri-apps/api/path"
import { readDir } from "@tauri-apps/api/fs"
import sidebar from "./components/Sidebar.vue"
import topbar from "./components/Topbar.vue"
import fileView from "./components/FileView.vue"
import homePage from "./components/HomePage.vue"
import { ref, watch } from "vue"

const path = ref("")

function on_back() {
    let segments = path.value.split(sep)

    let pop = segments.pop()

    if (segments.length == 1 & pop == "") {
        path.value = ""
        return
    }

    if (segments.length == 1 & pop != "") {
        segments.push("")
        
    }

    path.value = segments.join(sep)
}

function on_search(keyword) {
    //todo
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
            <Suspense v-if="path != ''">
                <fileView class="fileView" @open="open" :entries="entries"/>
            </Suspense>
            <homePage class="homePage" @open="open" v-else/>
        </div>
    </div>
</template>

<style scoped>

</style>