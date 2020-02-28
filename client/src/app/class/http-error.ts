export class HttpError {
    constructor(
        public kind: string = "",
        public message: string = "",
        public code?: number,
        public raw?: string
    ) { }
}
