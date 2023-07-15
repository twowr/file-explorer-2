<script setup>

import { sep } from "@tauri-apps/api/path"
import sidebar from "./components/Sidebar.vue"
import dir_view from "./components/Dir_View.vue"
import { ref } from "vue"

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

function open(dir, is_directory) {
    if (is_directory) {
        path.value = dir
    }
}

</script>

<template>
    <div class="container">
        <sidebar class="sidebar"/>
        <div class="view_panel">
            <Suspense>
                <dir_view class="dir_view" @open="open" @back="on_back" @submit="open" :dir="path"/>
            </Suspense>
        </div>
    </div>
</template>

<style scoped>

</style>