<script setup>
import { computed } from "vue"

const props = defineProps(["entries"])

const emit = defineEmits(["open"])

const entries = computed(() => {
    return props.entries
})
</script>

<template>
    <div class="entry" @click="emit('open', entry.path, entry.children != null)" v-if="entries != null" v-for="entry in entries">
        <img v-if="entry.children != null" src="../assets/folder_icon.png" alt="folder">
        <img v-else src="../assets/file_icon.png" alt="file">
        <span class="entry_name">{{ entry.name }}</span>
    </div>
</template>