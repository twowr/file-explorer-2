<script setup>
import { invoke } from "@tauri-apps/api/tauri"
import { ref } from "vue";

const emit = defineEmits(["open"])

const mountPoints = ref(null)

invoke("get_mount_points").then(result => mountPoints.value = result)
</script>

<template>
    <div>
        <div class="mountPoint" @click="emit('open', mountPoint, true)" v-if="mountPoints != null" v-for="mountPoint in mountPoints">
            <img class="baseDirectory" src="../assets/drive_icon.png" alt="base directory">
            <span class="mountPointName">{{ mountPoint }}</span>
        </div>
    </div>
</template>

<style scoped>

img {
  max-width: 14vh;
  max-height: 14vh;
}

.mountPoint {
  display: flex;
  cursor: pointer;
}

.mountPoint:hover {
  background-color: #6d6d6d;
}

.mountPointName {
  padding-top: 5.25vh;
}

</style>