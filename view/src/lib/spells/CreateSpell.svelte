<script lang="ts">
  import { Combobox } from "@skeletonlabs/skeleton-svelte";

  import Checkbox from "$components/Checkbox.svelte";
  import CheckboxGroup from "$components/CheckboxGroup.svelte";
  import Input from "$components/Input.svelte";
  import SelectInput from "$components/SelectInput.svelte";
  import { Areas } from "$types/Area";
  import { CastingTimes } from "$types/CastingTime";
  import { Durations } from "$types/Duration";
  import { MagicSchools } from "$types/MagicSchool";
  import { Ranges } from "$types/Range";
  import { Shapes } from "$types/Shape";
  import { Spell } from "$types/Spell.svelte";
  import { SpellLevels } from "$types/SpellLevel";
  import { SpellcastingClasses } from "$types/Class";

  let spell = new Spell();

  type ComboboxData = { label: string; value: string };

  const castingTimesComboboxData: ComboboxData[] = CastingTimes.map(
    (castingTime) => ({
      label: castingTime,
      value: castingTime,
    }),
  );
  let selectedCastingTime: string[] = $state([]);
  let customCastingTime: string = $state("");

  const durationComboboxData: ComboboxData[] = Durations.map((duration) => ({
    label: duration,
    value: duration,
  }));
  let selectedDuration: string[] = $state([]);
  let customDuration: string = $state("");

  const rangeComboboxData: ComboboxData[] = Ranges.map((range) => ({
    label: range,
    value: range,
  }));
  let selectedRange: string[] = $state([]);
  let customRange: string = $state("");

  const areaComboboxData: ComboboxData[] = Areas.map((area) => ({
    label: area,
    value: area,
  }));
  let selectedArea: string[] = $state([]);
  let customArea: string = $state("");

  const shapeComboboxData: ComboboxData[] = Shapes.map((shape) => ({
    label: shape,
    value: shape,
  }));
  let selectedShape: string[] = $state([]);
  let customShape: string = $state("");
</script>

<form class="py-2 space-y-2">
  <!-- Basic Information -->
  <h2 class="h2">Basic Information</h2>
  <div class="input-group grid-cols-16">
    <!-- Name -->
    <Input
      label="Name"
      bind:value={spell.name}
      type="text"
      placeholder="Fireball"
      labelSize={1}
      inputSize={8}
    />

    <!-- School -->
    <SelectInput
      title="School"
      items={MagicSchools}
      bind:value={spell.school}
      labelSize={1}
      inputSize={3}
    />

    <!-- Level -->
    <SelectInput
      title="Level"
      items={SpellLevels}
      bind:value={spell.level}
      labelSize={1}
      inputSize={2}
    />

    <hr class="hr col-span-16" />

    <!-- Casting Time -->
    <div class="ig-cell preset-tonal col-span-2">Casting Time</div>
    <div class="col-span-3">
      <Combobox
        multiple={false}
        data={castingTimesComboboxData}
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

    <!-- Duration -->
    <div class="ig-cell preset-tonal col-span-2">Duration</div>
    <div class="col-span-3">
      <Combobox
        multiple={false}
        data={durationComboboxData}
        value={selectedDuration}
        allowCustomValue={true}
        onValueChange={(details) => (selectedDuration = details.value)}
        onInputValueChange={(details) => (customDuration = details.inputValue)}
      >
        {#snippet item(option)}
          {option.label}
        {/snippet}
      </Combobox>
    </div>

    <!-- Range -->
    <div class="ig-cell preset-tonal col-span-1">Range</div>
    <div class="col-span-3">
      <Combobox
        multiple={false}
        data={rangeComboboxData}
        value={selectedRange}
        allowCustomValue={true}
        onValueChange={(details) => (selectedRange = details.value)}
        onInputValueChange={(details) => (customRange = details.inputValue)}
      >
        {#snippet item(option)}
          {option.label}
        {/snippet}
      </Combobox>
    </div>

    <!-- Ritual -->
    <Checkbox
      label="Ritual"
      bind:value={spell.ritual}
      columns={2}
      center={true}
    />

    <hr class="hr col-span-16" />

    <!-- Area -->
    <div class="ig-cell preset-tonal col-span-1">Area</div>
    <div class="col-span-4">
      <Combobox
        multiple={false}
        data={areaComboboxData}
        value={selectedArea}
        allowCustomValue={true}
        onValueChange={(details) => (selectedArea = details.value)}
        onInputValueChange={(details) => (customArea = details.inputValue)}
      >
        {#snippet item(option)}
          {option.label}
        {/snippet}
      </Combobox>
    </div>

    <!-- Shape -->
    <div class="ig-cell preset-tonal col-span-1">Shape</div>
    <div class="col-span-3">
      <Combobox
        multiple={false}
        data={shapeComboboxData}
        value={selectedShape}
        allowCustomValue={true}
        onValueChange={(details) => (selectedShape = details.value)}
        onInputValueChange={(details) => (customShape = details.inputValue)}
      >
        {#snippet item(option)}
          {option.label}
        {/snippet}
      </Combobox>
    </div>

    <!-- areaType: string | null; -->
  </div>

  <h6 class="h6">Requirements</h6>
  <div class="input-group grid-cols-16 px-2">
    <!-- Verbal -->
    <Checkbox label="Verbal" bind:value={spell.verbal} columns={2} />

    <!-- Somatic -->
    <Checkbox label="Somatic" bind:value={spell.somatic} columns={2} />

    <!-- Concentration -->
    <Checkbox
      label="Concentration"
      bind:value={spell.concentration}
      columns={2}
    />

    <!-- Material -->
    <Input
      label="Material"
      bind:value={spell.material}
      type="text"
      placeholder="a tiny ball of bat guano and sulfur"
      labelSize={2}
      inputSize={5}
    />

    <!-- Material Consumed -->
    <Checkbox
      label="Materials consumed"
      bind:value={spell.materialConsumed}
      columns={3}
      center={true}
    />
  </div>

  <h6 class="h6">Class Restrictions</h6>
  <CheckboxGroup
    items={SpellcastingClasses}
    bind:checkedItems={spell.classes}
    columns={3}
  />
</form>
