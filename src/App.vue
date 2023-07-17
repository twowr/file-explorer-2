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

    if (pop == "") {
        segments.pop()
    }

    path.value = segments.join(sep)

    if (path.value != "") {
        path.value += sep
    }
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

.container {
  display: flex;
}

.sidebar {
  display: flex;
  width: 15vw;
  height: 100vh;
  margin-right: 0;
  color: #f6f6f6;
  background-color: #6C6C60;
  justify-content: center;
}

.viewPanel {
  width: 85vw;
  height: 100vh;
  overflow-x: hidden;
  overflow-y: scroll;
}

.topbar {
  display: flex;
  gap: 0.25vw;
  height: 9vh;
}

.fileView {
  color: #f6f6f6;
  height: 91vh;
}

.homePage {
  color: #f6f6f6;
  height: 91vh;
}

</style>