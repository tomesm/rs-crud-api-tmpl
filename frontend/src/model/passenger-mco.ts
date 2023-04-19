import { hub } from 'dom-native';
import { apiGet, apiPatch, apiDelete, apiPost } from '../api';

export interface Passenger {
    id: string;
    first_name: string;
    last_name: string;
    status: string;
}

export type PassengerPatch = Partial<Omit<Passenger, 'id'>>;

class PassengerMco {

    async list(): Promise<Passenger[]> {
        const data = await apiGet('passengers');
        return data as Passenger[];
    }

    async create(data: PassengerPatch): Promise<Passenger> {
         // guard (TODO - validate data)
        if (data.first_name == null || data.first_name.trim().length == 0) {
            throw new Error("Cannot create Todo with empty name");
        }
        // to server
        const newData = await apiPost('passengers', data);
        // sending event. no need for a complex state management
        hub('dataHub').pub('Passenger', 'create', newData);
    
        return newData as Passenger;
    }

    async update(id: string, data: PassengerPatch): Promise<Passenger> {
        // TODO - validate data
        // to server
        const newData = await apiPatch(`passengers/${id}`, data);
        // event
        hub('dataHub').pub('Passenger', 'update', newData);
    
        return newData as Passenger;
      }
    
      async delete(id: string): Promise<Passenger> {
        // to server
        const oldData = await apiDelete(`passengers/${id}`);
        // event
        hub('dataHub').pub('Passenger', 'delete', oldData);
    
        return oldData as Passenger;
      }
}

// export as singleton
export const passengerMco = new PassengerMco();
