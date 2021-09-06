let x = await Date.now();
console.log(`This is awaited now ${x}`);

async function getTime() {
    return await Date.now();
}