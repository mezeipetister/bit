export class Notification {
    constructor(
        public id: number = 0,
        public date_created: Date = new Date(),
        public is_new: boolean = true,
        public subject: string = "",
        public location: string | null = null
    ) { }
}
