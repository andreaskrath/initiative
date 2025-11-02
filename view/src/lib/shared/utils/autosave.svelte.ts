import { toast } from "svelte-sonner";

export function useAutoSave<T extends { id?: string }>(
  getData: () => T,
  endpoint: string,
  debounceMs = 2000
) {
  let saveTimeoutId: ReturnType<typeof setTimeout> | undefined;
  let lastSaved: string | undefined = undefined;
  let isSaving = $state(false);

  $effect(() => {
    const snapshot = JSON.stringify(getData());

    // Skip initial run
    if (lastSaved === undefined) {
      lastSaved = snapshot;
      return;
    }

    // Skip if nothing changed
    if (snapshot === lastSaved) return;

    // Clear previous timeout
    if (saveTimeoutId !== undefined) {
      clearTimeout(saveTimeoutId);
    }

    // Debounce: save after specified delay
    saveTimeoutId = setTimeout(async () => {
      isSaving = true;
      try {
        const data = getData();
        const response = await fetch(`${endpoint}/${data.id}`, {
          method: "PUT",
          headers: { "Content-Type": "application/json" },
          body: snapshot,
        });

        if (!response.ok) throw new Error("Failed to save");

        lastSaved = snapshot;
        toast.success("Encounter saved", { duration: 1000 });
      } catch (error) {
        toast.error("Failed to save encounter");
        console.error(error);
      } finally {
        isSaving = false;
      }
    }, debounceMs);
  });

  // Cleanup on unmount
  $effect(() => {
    return () => {
      if (saveTimeoutId !== undefined) {
        clearTimeout(saveTimeoutId);
      }
    };
  });

  return {
    get isSaving() {
      return isSaving;
    },
  };
}
