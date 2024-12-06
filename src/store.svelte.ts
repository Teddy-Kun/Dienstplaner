import type { Employee } from "./utils";

class GlobalStore {
	public employees: Employee[] = [];
}

const store = new GlobalStore();

export default store;
