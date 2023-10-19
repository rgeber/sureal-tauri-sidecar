<template>
  <h1>SurrealDB Sidecar</h1>
  <p></p>
</template>

<script setup lang="ts">

import Surreal from "surrealdb.js";
const db = new Surreal();


import {invoke} from "@tauri-apps/api";
import {onBeforeMount} from "vue";

onBeforeMount(async () => {

  const surreal_db_password: string = await invoke('get_surrealdb_password');

  await db.connect("http://127.0.0.1:8877");

  console.log(surreal_db_password);

  await db.signin({
    user: "root",
    pass: surreal_db_password,
  });

  await db.use({ns: "test", db: "test"});

  let created = await db.create("person", {
    title: "Founder & CEO",
    name: {
      first: "Tobie",
      last: "Morgan Hitchcock",
    },
    marketing: true,
    identifier: Math.random().toString(36).substr(2, 10),
  });

  console.log(created);

});
</script>

<style scoped>

</style>
