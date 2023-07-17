<script setup>
const props = defineProps(["entries"])

const emit = defineEmits(["open"])
</script>

<template>
    <div>
        <div class="entry" @click="emit('open', entry.path, entry.children != null)" v-if="entries != null" v-for="entry in props.entries">
            <img v-if="entry.children != null" src="../assets/folder_icon.png" alt="folder">
            <img v-else src="../assets/file_icon.png" alt="file">
            <span class="entryName">{{ entry.name }}</span>
        </div>
        <span v-if="entries.length == 0">This directory is empty</span>
    </div>
</template>

<style scoped>

img {
  max-width: 7vh;
  max-height: 7vh;
}

span {
  display: flex;
  justify-content: center;
  padding-top: 1.75vh;
}

.entry {
  display: flex;
  cursor: pointer;
}

.entry:hover {
  background-color: #6d6d6d;
}

.entryName {
  padding-top: 1.75vh;
}

</style>