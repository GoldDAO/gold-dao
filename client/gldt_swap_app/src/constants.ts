export const APP_MODE = import.meta.env.MODE
export const GLDT_SWAP_APP_FRONT_URL = import.meta.env.VITE_GLDT_SWAP_APP_FRONT_URL;

if (!(APP_MODE === "production")) {
    console.log(`APP_MODE=${APP_MODE}`)

    if (!GLDT_SWAP_APP_FRONT_URL)
        console.log("No GLDT_SWAP_APP_FRONT_URL environment variable. Set GLDT_SWAP_APP_FRONT_URL environment variable.")
    else
        console.log(`GLDT_SWAP_APP_FRONT_URL=${GLDT_SWAP_APP_FRONT_URL}`)
}
