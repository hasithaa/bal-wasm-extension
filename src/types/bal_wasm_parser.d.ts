declare module "*/bal_wasm_parser" {
    export class BallerinaParser {
        constructor(source: string);
        parse(): Promise<void>;
    }

    export const init: () => Promise<void>;
    export const __wbg_init: () => Promise<void>;
    
    export default function(): Promise<void>;
}
