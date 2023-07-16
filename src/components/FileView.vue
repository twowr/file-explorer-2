<script setup>
const props = defineProps(["entries"])

const emit = defineEmits(["open"])
</script>

<template>
    <div class="entry" @click="emit('open', entry.path, entry.children != null)" v-if="entries != null" v-for="entry in props.entries">
        <img v-if="entry.children != null" src="../assets/folder_icon.png" alt="folder">
        <img v-else src="../assets/file_icon.png" alt="file">
        <span class="entryName">{{ entry.name }}</span>
    </div>
</template>