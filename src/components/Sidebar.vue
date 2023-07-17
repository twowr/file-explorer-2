<script setup>

const emit = defineEmits(["back"])

</script>

<template>
    <div>
        <button class="backButton" @click="emit('back')">back</button>
    </div>
</template>

<style scoped>

.backButton {
  background-color: #C67006;
  width: 7vh;
  height: 7vh;
}

</style>