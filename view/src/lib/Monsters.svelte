<script lang="ts">
  import Button from "./components/Button.svelte";

  function handleButtonClick(event: MouseEvent) {
    event.preventDefault();
    console.log("Hello");
  }
</script>

<div>
  <h1 class="dnd-font color-text title">Monsters</h1>

  <Button title="Create New Monster" onClick={handleButtonClick} />
</div>
