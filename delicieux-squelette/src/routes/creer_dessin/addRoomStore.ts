import { writable } from "svelte/store";
import type { AddRoom } from "../../components/types";


export const roomAdd = writable<AddRoom>({
    name: "",
    admin_password: "",
    colors: ["#000000"],
    is_public: true,
    password_protected: false,
    resolution: 64,
    rules: "",
    password: ""
})