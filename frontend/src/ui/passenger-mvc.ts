import { BaseHTMLElement, OnEvent, customElement, first, html, onEvent, onHub, scanChild } from "dom-native";
import { Passenger, passengerMco } from '../model/passenger-mco';

@customElement("passenger-mvc")
class PassengercMvc extends BaseHTMLElement {
    #passengerInputEl!: PassengerInput;
    #passengerListEl!: HTMLElement;
    init() {
        let htmlContent: DocumentFragment = html`
            <div class="box"></div>
            <h1>Passengers</h1>
            <passenger-input></passenger-input>
            <passenger-list></passenger-list>
        `;
        [this.#passengerInputEl, this.#passengerListEl] = scanChild(htmlContent, "passenger-input", "passenger-list");
        this.append(htmlContent);
        this.refresh();
    }

    async refresh() {
        let passengers: Passenger[] = await passengerMco.list();
        let htmlContent = document.createDocumentFragment();
        for (const passenger of passengers) {
            const el = document.createElement("passenger-item") as PassengerItem;
            el.data = passenger; // passenger will be frozen
            htmlContent.append(el);
        }
        this.#passengerListEl.innerHTML = "";
        this.#passengerListEl.append(htmlContent);
    }

    // #region    --- UI Events
    @onEvent('pointerup', 'c-check')
    onCheckPassenger(evt: PointerEvent & OnEvent) {
        const passengerItem = evt.selectTarget.closest("todo-item")! as PassengerItem;
        const status = passengerItem.data.status === 'new' ? 'approved' : 'denied';
        // update to server
        passengerMco.update(passengerItem.data.id, { status });
    }
    // #endregion --- UI Events

    // #region    --- Data Events
    @onHub('dataHub', 'Passenger', 'update')
    onPassengerUpdate(data: Passenger) {
        // find the todo in the UI
        const passengerItem = first(`passenger-item.Passenger-${data.id}`) as PassengerItem | undefined;
        // if found, update it.
        if (passengerItem) {
        passengerItem.data = data; // data will be frozen
        }
    }

    @onHub('dataHub', 'Passenger', 'create')
    onPassengerCreate(data: Passenger) {
        this.refresh();
    }
  // #endregion --- Data Events
}
}

@customElement("passenger-input")
class PassengerInput extends BaseHTMLElement {
    #inputEl!: HTMLInputElement;
    init() {
        let htmlContent = html`
            <input type="text" placeholder="Enter name" />
        `;
        this.#inputEl = scanChild(htmlContent, "input");
        this.append(htmlContent);
    }

    // #region    --- UI Events
    @onEvent('keyup', 'input')
    onInputKeyUp(evt: KeyboardEvent) {
        if (evt.key == "Enter") {
        // get value from UI
        const first_name = this.#inputEl.value;
        // send create to server
        passengerMco.create({ first_name });
        // don't wait, reset value input
        this.#inputEl.value = '';
        }
    }
  // #endregion --- UI Events
}

// passenger-input tag
declare global {
    interface HTMLElementTagNameMap {
        "passenger-input": PassengerInput;
    }
}

@customElement("passenger-item")
export class PassengerItem extends BaseHTMLElement {
    #titleEl!: HTMLElement;
    #data!: Passenger;

    set data(data: Passenger) {
        let oldData = this.#data;
        this.#data = Object.freeze(data); // make date immutable for now
        if (this.isConnected) {
           this.refresh(oldData);
        }
    }

   get data() { return this.#data; }

    init() {
        let htmlContent = html`
            <c-check><c-ico name="ico-done"></c-ico></c-check>
            <div class="title">STATIC TITLE</div>
            <c-ico name="ico-delete"></c-ico>
        `;
        this.#titleEl = scanChild(htmlContent, "div");
        this.append(htmlContent);
        this.refresh();
    }

   refresh(old?: Passenger) {
        if (old) {
            this.classList.remove(`Passenger-${old.id}`);
            this.classList.remove(old.status);
        }
        //render new data
        const passenger = this.#data;
        this.classList.add(`Passenger-${passenger.id}`);
        this.classList.add(passenger.status);
        this.#titleEl.textContent = `${passenger.first_name} ${passenger.last_name}`;
    }

}

