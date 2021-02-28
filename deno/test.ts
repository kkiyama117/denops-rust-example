import { serve } from "https://deno.land/std/http/server.ts";
import * as pkg from '../pkg/index.js'

type Resp = {
    body: string;
}
await pkg.default();

const s = serve({ port: 8000 });
console.log("http://localhost:8000/");
for await (const req of s) {
    let r = {} as Resp;
    r.body = pkg.greet();
    req.respond(r);
}