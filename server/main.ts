import { Application } from "https://deno.land/x/oak@14.2.0/mod.ts";

const app = new Application();
const PORT = 8000;

app.use((ctx) => {
  ctx.response.body = "Hello world!";
})

console.log(`Server running on port ${PORT}`);

await app.listen({ port: PORT});
