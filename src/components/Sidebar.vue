<script setup>

const emit = defineEmits(["back"])

</script>

<template>
    <button class="back_button" @back="emit('back')">back</button>
</template>