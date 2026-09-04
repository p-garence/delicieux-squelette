import { writable } from "svelte/store";
import type { ChangeRoom } from "../../components/types";


export const roomChange = writable<ChangeRoom>({
    rules: "",
})