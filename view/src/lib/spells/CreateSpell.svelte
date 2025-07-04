<script lang="ts">
  import { Combobox } from "@skeletonlabs/skeleton-svelte";

  import Checkbox from "$components/Checkbox.svelte";
  import CheckboxGroup from "$components/CheckboxGroup.svelte";
  import Input from "$components/Input.svelte";
  import SelectInput from "$components/SelectInput.svelte";
  import { CastingTimes } from "$types/CastingTime";
  import { Durations } from "$types/Duration";
  import { MagicSchools } from "$types/MagicSchool";
  import { Spell } from "$types/Spell.svelte";
  import { SpellLevels } from "$types/SpellLevel";
  import { SpellcastingClasses } from "$types/Class";

  let spell = new Spell();

  const castingTimesComboBoxData: { label: string; value: string }[] =
    CastingTimes.map((castingTime) => ({
      label: castingTime,
      value: castingTime,
    }));
  let selectedCastingTime: string[] = $state([]);
  let customCastingTime: string = $state("");
</script>

<form class="py-2 space-y-2">
  <!-- Basic Information -->
  <h2 class="h2">Basic Information</h2>
  <div class="input-group grid-cols-32">
    <!-- Name -->
    <Input
      label="Name"
      bind:value={spell.name}
      type="text"
      placeholder="Fireball"
      labelSize={2}
      inputSize={16}
    />

    <!-- School -->
    <SelectInput
      title="School"
      items={MagicSchools}
      bind:value={spell.school}
      labelSize={2}
      inputSize={6}
    />

    <!-- Level -->
    <SelectInput
      title="Level"
      items={SpellLevels}
      bind:value={spell.level}
      labelSize={2}
      inputSize={4}
    />

    <!-- Casting Time -->
    <!-- <SelectInput -->
    <!--   title="Casting Time" -->
    <!--   items={CastingTimes} -->
    <!--   bind:value={spell.castingTime} -->
    <!--   labelSize={2} -->
    <!--   inputSize={4} -->
    <!-- /> -->
    <!---->
    <!-- <!-- Duration -->
    <!-- <SelectInput -->
    <!--   title="Duration" -->
    <!--   items={Durations} -->
    <!--   bind:value={spell.duration} -->
    <!--   labelSize={2} -->
    <!--   inputSize={4} -->
    <!-- /> -->

    <!-- Range -->
    <!-- <SelectInput -->
    <!--   title="Range" -->
    <!--   items={Durations} -->
    <!--   bind:value={spell.duration} -->
    <!--   labelSize={2} -->
    <!--   inputSize={4} -->
    <!-- /> -->

    <hr class="hr col-span-32" />

    <!-- Casting Time -->
    <div class="ig-cell preset-tonal col-span-4">Casting Time</div>
    <div class="col-span-6">
      <Combobox
        multiple={false}
        data={castingTimesComboBoxData}
        value={selectedCastingTime}
        allowCustomValue={true}
        onValueChange={(details) => (selectedCastingTime = details.value)}
        onInputValueChange={(details) =>
          (customCastingTime = details.inputValue)}
      >
        {#snippet item(option)}
          {option.label}
        {/snippet}
      </Combobox>
    </div>

    <!-- castingTime: string | null; -->
    <!-- duration: string | null; -->
    <!-- range: string | null; -->
    <!-- area: string | null; -->
    <!-- areaType: string | null; -->
  </div>

  <h6 class="h6">Requirements</h6>
  <div class="input-group grid-cols-7 px-2">
    <!-- Verbal -->
    <Checkbox label="Verbal" bind:value={spell.verbal} />

    <!-- Somatic -->
    <Checkbox label="Somatic" bind:value={spell.somatic} />

    <!-- Ritual -->
    <Checkbox label="Ritual" bind:value={spell.ritual} />

    <!-- Concentration -->
    <Checkbox label="Concentration" bind:value={spell.concentration} />

    <!-- Material -->
    <Input
      label="Material"
      bind:value={spell.material}
      type="text"
      placeholder="a tiny ball of bat guano and sulfur"
      labelSize={1}
      inputSize={2}
    />
  </div>

  <h6 class="h6">Class Restrictions</h6>
  <CheckboxGroup
    items={SpellcastingClasses}
    bind:checkedItems={spell.classes}
    columns={3}
  />
</form>
