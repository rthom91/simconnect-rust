#![allow(clippy::too_many_arguments, clippy::missing_safety_doc)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;
use std::ptr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[derive(Debug)]
pub enum DispatchResult<'a> {
    Null,
    Exception(&'a SIMCONNECT_RECV_EXCEPTION),
    Open(&'a SIMCONNECT_RECV_OPEN),
    Quit(&'a SIMCONNECT_RECV_QUIT),
    Event(&'a SIMCONNECT_RECV_EVENT),
    EventObjectAddRemove(&'a SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE),
    EventFilename(&'a SIMCONNECT_RECV_EVENT_FILENAME),
    EventFrame(&'a SIMCONNECT_RECV_EVENT_FRAME),
    SimObjectData(&'a SIMCONNECT_RECV_SIMOBJECT_DATA),
    SimObjectDataByType(&'a SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE),
    WeatherObservation(&'a SIMCONNECT_RECV_WEATHER_OBSERVATION),
    CloudState(&'a SIMCONNECT_RECV_CLOUD_STATE),
    AssignedObjectId(&'a SIMCONNECT_RECV_ASSIGNED_OBJECT_ID),
    ReservedKey(&'a SIMCONNECT_RECV_RESERVED_KEY),
    CustomAction(&'a SIMCONNECT_RECV_CUSTOM_ACTION),
    SystemState(&'a SIMCONNECT_RECV_SYSTEM_STATE),
    ClientData(&'a SIMCONNECT_RECV_CLIENT_DATA),
    EventWeatherMode(&'a SIMCONNECT_RECV_EVENT_WEATHER_MODE),
    AirportList(&'a SIMCONNECT_RECV_AIRPORT_LIST),
    VorList(&'a SIMCONNECT_RECV_VOR_LIST),
    NdbList(&'a SIMCONNECT_RECV_NDB_LIST),
    WaypointList(&'a SIMCONNECT_RECV_WAYPOINT_LIST),
    EventMultiplayerServerStarted(&'a SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED),
    EventMultiplayerClientStarted(&'a SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED),
    EventMultiplayerSessionEnded(&'a SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED),
    EventRaceEnd(&'a SIMCONNECT_RECV_EVENT_RACE_END),
    EventRaceLap(&'a SIMCONNECT_RECV_EVENT_RACE_LAP),
    EventEx1(&'a SIMCONNECT_RECV_EVENT_EX1),
    FacilityData(&'a SIMCONNECT_RECV_FACILITY_DATA),
    FacilityDataEnd(&'a SIMCONNECT_RECV_FACILITY_DATA_END),
    FacilityMinimalList(&'a SIMCONNECT_RECV_FACILITY_MINIMAL_LIST),
    JetwayData(&'a SIMCONNECT_RECV_JETWAY_DATA),
    ActionCallback(&'a SIMCONNECT_RECV_ACTION_CALLBACK),
    EnumerateInputEvents(&'a SIMCONNECT_RECV_ENUMERATE_INPUT_EVENTS),
    GetInputEvent(&'a SIMCONNECT_RECV_GET_INPUT_EVENT),
    SubscribeInputEvent(&'a SIMCONNECT_RECV_SUBSCRIBE_INPUT_EVENT),
    EnumerateInputEventParams(&'a SIMCONNECT_RECV_ENUMERATE_INPUT_EVENT_PARAMS),
    ControllersList(&'a SIMCONNECT_RECV_CONTROLLERS_LIST),
}

#[derive(Debug)]
pub struct SimConnector {
    handle: HANDLE,
}

impl Default for SimConnector {
    fn default() -> Self {
        Self { handle: ptr::null_mut() }
    }
}

impl SimConnector {
    pub fn new() -> Self {
        Self::default()
    }

    // ==================== Connection & Lifecycle ====================

    pub fn connect(&mut self, program_name: &str) -> bool {
        let name = cstring(program_name);

        unsafe {
            SimConnect_Open(
                &mut self.handle,
                name.as_ptr(),
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                0,
            ) == 0
        }
    }

    pub fn close(&mut self) -> bool {
        if self.handle.is_null() {
            return true;
        }
        let result = unsafe { SimConnect_Close(self.handle) == 0 };
        self.handle = ptr::null_mut();
        result
    }

    // ==================== Data Definition ====================

    pub fn add_data_definition(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        datum_name: &str,
        units_name: &str,
        datum_type: SIMCONNECT_DATATYPE,
        epsilon: f32,
        datum_id: DWORD,
    ) -> bool {
        let name = cstring(datum_name);
        let units = cstring(units_name);

        unsafe {
            SimConnect_AddToDataDefinition(
                self.handle,
                define_id,
                name.as_ptr(),
                units.as_ptr(),
                datum_type,
                epsilon,
                datum_id,
            ) == 0
        }
    }

    pub fn clear_data_definition(&self, define_id: SIMCONNECT_DATA_DEFINITION_ID) -> bool {
        unsafe { SimConnect_ClearDataDefinition(self.handle, define_id) == 0 }
    }

    // ==================== Events ====================

    pub fn map_client_event_to_sim_event(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        event_name: &str,
    ) -> bool {
        let name = cstring(event_name);
        unsafe { SimConnect_MapClientEventToSimEvent(self.handle, event_id, name.as_ptr()) == 0 }
    }

    pub fn subscribe_to_system_event(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        event_name: &str,
    ) -> bool {
        let name = cstring(event_name);
        unsafe {
            SimConnect_SubscribeToSystemEvent(self.handle, event_id, name.as_ptr()) == 0
        }
    }

    pub fn unsubscribe_from_system_event(&self, event_id: SIMCONNECT_CLIENT_EVENT_ID) -> bool {
        unsafe { SimConnect_UnsubscribeFromSystemEvent(self.handle, event_id) == 0 }
    }

    pub fn set_system_event_state(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        state: SIMCONNECT_STATE,
    ) -> bool {
        unsafe { SimConnect_SetSystemEventState(self.handle, event_id, state) == 0 }
    }

    pub fn transmit_client_event(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        dw_data: DWORD,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        flags: SIMCONNECT_EVENT_FLAG,
    ) -> bool {
        unsafe {
            SimConnect_TransmitClientEvent(
                self.handle,
                object_id,
                event_id,
                dw_data,
                group_id,
                flags,
            ) == 0
        }
    }

    pub fn add_client_event_to_notification_group(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        maskable: bool,
    ) -> bool {
        unsafe {
            SimConnect_AddClientEventToNotificationGroup(
                self.handle,
                group_id,
                event_id,
                maskable as i32,
            ) == 0
        }
    }

    pub fn set_notification_group_priority(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        priority: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_SetNotificationGroupPriority(self.handle, group_id, priority) == 0
        }
    }

    pub fn remove_client_event(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
    ) -> bool {
        unsafe { SimConnect_RemoveClientEvent(self.handle, group_id, event_id) == 0 }
    }

    pub fn clear_notification_group(&self, group_id: SIMCONNECT_NOTIFICATION_GROUP_ID) -> bool {
        unsafe { SimConnect_ClearNotificationGroup(self.handle, group_id) == 0 }
    }

    pub fn request_notification_group(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        reserved: DWORD,
        flags: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_RequestNotificationGroup(self.handle, group_id, reserved, flags) == 0
        }
    }

    pub fn transmit_client_event_ex1(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        flags: SIMCONNECT_EVENT_FLAG,
        data0: DWORD,
        data1: DWORD,
        data2: DWORD,
        data3: DWORD,
        data4: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_TransmitClientEvent_EX1(
                self.handle,
                object_id,
                event_id,
                group_id,
                flags,
                data0,
                data1,
                data2,
                data3,
                data4,
            ) == 0
        }
    }

    pub fn request_reserved_key(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        key_choice1: &str,
        key_choice2: &str,
        key_choice3: &str,
    ) -> bool {
        let k1 = cstring(key_choice1);
        let k2 = cstring(key_choice2);
        let k3 = cstring(key_choice3);

        unsafe {
            SimConnect_RequestReservedKey(
                self.handle,
                event_id,
                k1.as_ptr(),
                k2.as_ptr(),
                k3.as_ptr(),
            ) == 0
        }
    }

    // ==================== Input Events ====================

    pub fn map_input_event_to_client_event(
        &self,
        group_id: SIMCONNECT_INPUT_GROUP_ID,
        input_definition: &str,
        down_event: SIMCONNECT_CLIENT_EVENT_ID,
        down_return_value: DWORD,
        up_event: SIMCONNECT_CLIENT_EVENT_ID,
        up_return_value: DWORD,
        maskable: bool,
    ) -> bool {
        let input = cstring(input_definition);
        unsafe {
            SimConnect_MapInputEventToClientEvent(
                self.handle,
                group_id,
                input.as_ptr(),
                down_event,
                down_return_value,
                up_event,
                up_return_value,
                maskable as i32,
            ) == 0
        }
    }

    pub fn map_input_event_to_client_event_ex1(
        &self,
        group_id: SIMCONNECT_INPUT_GROUP_ID,
        input_definition: &str,
        down_event: SIMCONNECT_CLIENT_EVENT_ID,
        down_return_value: DWORD,
        up_event: SIMCONNECT_CLIENT_EVENT_ID,
        up_return_value: DWORD,
        maskable: bool,
    ) -> bool {
        let input = cstring(input_definition);
        unsafe {
            SimConnect_MapInputEventToClientEvent_EX1(
                self.handle,
                group_id,
                input.as_ptr(),
                down_event,
                down_return_value,
                up_event,
                up_return_value,
                maskable as i32,
            ) == 0
        }
    }

    pub fn set_input_group_state(&self, group_id: SIMCONNECT_INPUT_GROUP_ID, state: DWORD) -> bool {
        unsafe { SimConnect_SetInputGroupState(self.handle, group_id, state) == 0 }
    }

    pub fn set_input_priority(&self, group_id: SIMCONNECT_INPUT_GROUP_ID, priority: DWORD) -> bool {
        unsafe { SimConnect_SetInputGroupPriority(self.handle, group_id, priority) == 0 }
    }

    pub fn remove_input_event(
        &self,
        group_id: SIMCONNECT_INPUT_GROUP_ID,
        input_definition: &str,
    ) -> bool {
        let input = cstring(input_definition);
        unsafe {
            SimConnect_RemoveInputEvent(self.handle, group_id, input.as_ptr()) == 0
        }
    }

    pub fn clear_input_group(&self, group_id: SIMCONNECT_INPUT_GROUP_ID) -> bool {
        unsafe { SimConnect_ClearInputGroup(self.handle, group_id) == 0 }
    }

    // ==================== Data Requests ====================

    pub fn request_data_on_sim_object(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        object_id: SIMCONNECT_OBJECT_ID,
        period: SIMCONNECT_PERIOD,
        flags: SIMCONNECT_DATA_REQUEST_FLAG,
        origin: DWORD,
        interval: DWORD,
        limit: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_RequestDataOnSimObject(
                self.handle,
                request_id,
                define_id,
                object_id,
                period,
                flags,
                origin,
                interval,
                limit,
            ) == 0
        }
    }

    pub fn request_data_on_sim_object_type(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        radius_in_meters: DWORD,
        object_type: SIMCONNECT_SIMOBJECT_TYPE,
    ) -> bool {
        unsafe {
            SimConnect_RequestDataOnSimObjectType(
                self.handle,
                request_id,
                define_id,
                radius_in_meters,
                object_type,
            ) == 0
        }
    }

    pub unsafe fn set_data_on_sim_object(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        object_id: SIMCONNECT_OBJECT_ID,
        flags: SIMCONNECT_DATA_SET_FLAG,
        array_count: DWORD,
        size: DWORD,
        pntr: *mut std::os::raw::c_void,
    ) -> bool {
        unsafe {
            SimConnect_SetDataOnSimObject(
                self.handle, define_id, object_id, flags, array_count, size, pntr,
            ) == 0
        }
    }

    // ==================== Client Data ====================

    pub fn create_client_data(
        &self,
        data_id: SIMCONNECT_CLIENT_DATA_ID,
        size: DWORD,
        flags: SIMCONNECT_CREATE_CLIENT_DATA_FLAG,
    ) -> bool {
        unsafe { SimConnect_CreateClientData(self.handle, data_id, size, flags) == 0 }
    }

    pub fn map_client_data_name_to_id(
        &self,
        client_data_name: &str,
        data_id: SIMCONNECT_CLIENT_DATA_ID,
    ) -> bool {
        let name = cstring(client_data_name);
        unsafe {
            SimConnect_MapClientDataNameToID(self.handle, name.as_ptr(), data_id) == 0
        }
    }

    pub fn add_to_client_data_definition(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        offset: DWORD,
        size_or_type: DWORD,
        epsilon: f32,
        datum_id: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_AddToClientDataDefinition(
                self.handle, define_id, offset, size_or_type, epsilon, datum_id,
            ) == 0
        }
    }

    pub fn clear_client_data_definition(&self, define_id: SIMCONNECT_DATA_DEFINITION_ID) -> bool {
        unsafe { SimConnect_ClearClientDataDefinition(self.handle, define_id) == 0 }
    }

    pub fn request_client_data(
        &self,
        data_id: SIMCONNECT_CLIENT_DATA_ID,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        period: SIMCONNECT_CLIENT_DATA_PERIOD,
        flags: SIMCONNECT_CLIENT_DATA_REQUEST_FLAG,
        origin: DWORD,
        interval: DWORD,
        limit: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_RequestClientData(
                self.handle,
                data_id,
                request_id,
                define_id,
                period,
                flags,
                origin,
                interval,
                limit,
            ) == 0
        }
    }

    pub unsafe fn set_client_data(
        &self,
        data_id: SIMCONNECT_CLIENT_DATA_ID,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        flags: DWORD,
        reserved: DWORD,
        unit_size: DWORD,
        data_set: *mut std::os::raw::c_void,
    ) -> bool {
        unsafe {
            SimConnect_SetClientData(
                self.handle, data_id, define_id, flags, reserved, unit_size, data_set,
            ) == 0
        }
    }

    // ==================== AI Objects ====================

    pub fn ai_create_parked_atc_aircraft(
        &self,
        container_title: &str,
        tail_number: &str,
        airport_id: &str,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        let title = cstring(container_title);
        let tail = cstring(tail_number);
        let airport = cstring(airport_id);

        unsafe {
            SimConnect_AICreateParkedATCAircraft(
                self.handle,
                title.as_ptr(),
                tail.as_ptr(),
                airport.as_ptr(),
                request_id,
            ) == 0
        }
    }

    pub fn ai_create_enroute_atc_aircraft(
        &self,
        container_title: &str,
        tail_number: &str,
        flight_number: i32,
        flight_plan_path: &str,
        flight_plan_position: f64,
        touch_and_go: bool,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        let title = cstring(container_title);
        let tail = cstring(tail_number);
        let plan = cstring(flight_plan_path);

        unsafe {
            SimConnect_AICreateEnrouteATCAircraft(
                self.handle,
                title.as_ptr(),
                tail.as_ptr(),
                flight_number,
                plan.as_ptr(),
                flight_plan_position,
                touch_and_go as i32,
                request_id,
            ) == 0
        }
    }

    pub fn ai_create_non_atc_aircraft(
        &self,
        container_title: &str,
        tail_number: &str,
        init_pos: SIMCONNECT_DATA_INITPOSITION,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        let title = cstring(container_title);
        let tail = cstring(tail_number);

        unsafe {
            SimConnect_AICreateNonATCAircraft(
                self.handle, title.as_ptr(), tail.as_ptr(), init_pos, request_id,
            ) == 0
        }
    }

    pub fn ai_create_simulated_object(
        &self,
        container_title: &str,
        init_pos: SIMCONNECT_DATA_INITPOSITION,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        let title = cstring(container_title);
        unsafe {
            SimConnect_AICreateSimulatedObject(self.handle, title.as_ptr(), init_pos, request_id) == 0
        }
    }

    pub fn ai_release_control(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe { SimConnect_AIReleaseControl(self.handle, object_id, request_id) == 0 }
    }

    pub fn ai_remove_object(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe { SimConnect_AIRemoveObject(self.handle, object_id, request_id) == 0 }
    }

    pub fn ai_set_aircraft_flight_plan(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        flight_plan_path: &str,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        let plan = cstring(flight_plan_path);
        unsafe {
            SimConnect_AISetAircraftFlightPlan(
                self.handle, object_id, plan.as_ptr(), request_id,
            ) == 0
        }
    }

    // ==================== Weather ====================

    pub fn weather_request_interpolated_observation(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        lat: f32,
        lon: f32,
        alt: f32,
    ) -> bool {
        unsafe {
            SimConnect_WeatherRequestInterpolatedObservation(self.handle, request_id, lat, lon, alt) == 0
        }
    }

    pub fn weather_request_observation_at_station(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        station: &str,
    ) -> bool {
        let station = cstring(station);
        unsafe {
            SimConnect_WeatherRequestObservationAtStation(self.handle, request_id, station.as_ptr()) == 0
        }
    }

    pub fn weather_request_observation_at_nearest_station(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        lat: f32,
        lon: f32,
    ) -> bool {
        unsafe {
            SimConnect_WeatherRequestObservationAtNearestStation(self.handle, request_id, lat, lon) == 0
        }
    }

    pub fn weather_create_station(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        icao: &str,
        name: &str,
        lat: f32,
        lon: f32,
        alt: f32,
    ) -> bool {
        let icao_c = cstring(icao);
        let name_c = cstring(name);

        unsafe {
            SimConnect_WeatherCreateStation(
                self.handle,
                request_id,
                icao_c.as_ptr(),
                name_c.as_ptr(),
                lat,
                lon,
                alt,
            ) == 0
        }
    }

    pub fn weather_remove_station(&self, request_id: SIMCONNECT_DATA_REQUEST_ID, station: &str) -> bool {
        let station = cstring(station);
        unsafe {
            SimConnect_WeatherRemoveStation(self.handle, request_id, station.as_ptr()) == 0
        }
    }

    pub fn weather_set_observation(&self, seconds: DWORD, observation: &str) -> bool {
        let obs = cstring(observation);
        unsafe {
            SimConnect_WeatherSetObservation(self.handle, seconds, obs.as_ptr()) == 0
        }
    }

    pub fn weather_set_mode_server(&self, port: DWORD, seconds: DWORD) -> bool {
        unsafe { SimConnect_WeatherSetModeServer(self.handle, port, seconds) == 0 }
    }

    pub fn weather_set_mode_theme(&self, theme_name: &str) -> bool {
        let theme = cstring(theme_name);
        unsafe { SimConnect_WeatherSetModeTheme(self.handle, theme.as_ptr()) == 0 }
    }

    pub fn weather_set_mode_global(&self) -> bool {
        unsafe { SimConnect_WeatherSetModeGlobal(self.handle) == 0 }
    }

    pub fn weather_set_mode_custom(&self) -> bool {
        unsafe { SimConnect_WeatherSetModeCustom(self.handle) == 0 }
    }

    pub fn weather_set_dynamic_update_rate(&self, rate: DWORD) -> bool {
        unsafe { SimConnect_WeatherSetDynamicUpdateRate(self.handle, rate) == 0 }
    }

    pub fn weather_request_cloud_state(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        min_x: f32,
        min_y: f32,
        min_z: f32,
        max_x: f32,
        max_y: f32,
        max_z: f32,
        flags: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_WeatherRequestCloudState(
                self.handle, request_id, min_x, min_y, min_z, max_x, max_y, max_z, flags,
            ) == 0
        }
    }

    pub fn weather_create_thermal(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        lat: f32,
        lon: f32,
        alt: f32,
        radius: f32,
        height: f32,
        core_rate: f32,
        core_turbulence: f32,
        sink_rate: f32,
        sink_turbulence: f32,
        core_size: f32,
        core_transition_size: f32,
        sink_layer_size: f32,
        sink_transition_size: f32,
    ) -> bool {
        unsafe {
            SimConnect_WeatherCreateThermal(
                self.handle,
                request_id,
                lat,
                lon,
                alt,
                radius,
                height,
                core_rate,
                core_turbulence,
                sink_rate,
                sink_turbulence,
                core_size,
                core_transition_size,
                sink_layer_size,
                sink_transition_size,
            ) == 0
        }
    }

    pub fn weather_remove_thermal(&self, object_id: SIMCONNECT_OBJECT_ID) -> bool {
        unsafe { SimConnect_WeatherRemoveThermal(self.handle, object_id) == 0 }
    }

    // ==================== Mission & System ====================

    pub fn execute_mission_action(&self, instance_id: GUID) -> bool {
        unsafe { SimConnect_ExecuteMissionAction(self.handle, instance_id) == 0 }
    }

    pub fn complete_custom_mission_action(&self, instance_id: GUID) -> bool {
        unsafe { SimConnect_CompleteCustomMissionAction(self.handle, instance_id) == 0 }
    }

    pub fn request_system_state(&self, request_id: SIMCONNECT_DATA_REQUEST_ID, state: &str) -> bool {
        let state = cstring(state);
        unsafe { SimConnect_RequestSystemState(self.handle, request_id, state.as_ptr()) == 0 }
    }

    pub fn set_system_state(&self, state: &str, integer: DWORD, float_val: f32, string_val: &str) -> bool {
        let state_c = cstring(state);
        let string_c = cstring(string_val);

        unsafe {
            SimConnect_SetSystemState(
                self.handle,
                state_c.as_ptr(),
                integer,
                float_val,
                string_c.as_ptr(),
            ) == 0
        }
    }

    pub fn flight_load(&self, file_name: &str) -> bool {
        let name = cstring(file_name);
        unsafe { SimConnect_FlightLoad(self.handle, name.as_ptr()) == 0 }
    }

    pub fn flight_save(&self, file_name: &str, title: &str, description: &str, flags: DWORD) -> bool {
        let name = cstring(file_name);
        let t = cstring(title);
        let d = cstring(description);
        unsafe { SimConnect_FlightSave(self.handle, name.as_ptr(), t.as_ptr(), d.as_ptr(), flags) == 0 }
    }

    pub fn flight_plan_load(&self, file_name: &str) -> bool {
        let name = cstring(file_name);
        unsafe { SimConnect_FlightPlanLoad(self.handle, name.as_ptr()) == 0 }
    }

    // ==================== Facilities ====================

    pub fn subscribe_to_facilities(
        &self,
        list_type: SIMCONNECT_FACILITY_LIST_TYPE,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe { SimConnect_SubscribeToFacilities(self.handle, list_type, request_id) == 0 }
    }

    pub fn unsubscribe_to_facilities(&self, list_type: SIMCONNECT_FACILITY_LIST_TYPE) -> bool {
        unsafe { SimConnect_UnsubscribeToFacilities(self.handle, list_type) == 0 }
    }

    pub fn request_facilities_list(
        &self,
        list_type: SIMCONNECT_FACILITY_LIST_TYPE,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe { SimConnect_RequestFacilitiesList(self.handle, list_type, request_id) == 0 }
    }

    pub fn add_to_facility_definition(&self, define_id: SIMCONNECT_DATA_DEFINITION_ID, field_name: &str) -> bool {
        let field = cstring(field_name);
        unsafe { SimConnect_AddToFacilityDefinition(self.handle, define_id, field.as_ptr()) == 0 }
    }

    pub fn request_facility_data(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        icao: &str,
        region: Option<&str>,
    ) -> bool {
        let icao = cstring(icao);
        let region = region.map(|s| cstring(s));
        unsafe {
            SimConnect_RequestFacilityData(
                self.handle,
                define_id,
                request_id,
                icao.as_ptr(),
                region.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
            ) == 0
        }
    }

    pub fn subscribe_to_facilities_ex1(
        &self,
        list_type: SIMCONNECT_FACILITY_LIST_TYPE,
        new_in_range_request_id: SIMCONNECT_DATA_REQUEST_ID,
        old_out_of_range_request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe {
            SimConnect_SubscribeToFacilities_EX1(
                self.handle,
                list_type,
                new_in_range_request_id,
                old_out_of_range_request_id,
            ) == 0
        }
    }

    pub fn unsubscribe_to_facilities_ex1(
        &self,
        list_type: SIMCONNECT_FACILITY_LIST_TYPE,
        unsubscribe_new_in_range: bool,
        unsubscribe_old_out_of_range: bool,
    ) -> bool {
        unsafe {
            SimConnect_UnsubscribeToFacilities_EX1(
                self.handle,
                list_type,
                unsubscribe_new_in_range,
                unsubscribe_old_out_of_range,
            ) == 0
        }
    }

    pub fn request_facilities_list_ex1(
        &self,
        list_type: SIMCONNECT_FACILITY_LIST_TYPE,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe {
            SimConnect_RequestFacilitiesList_EX1(self.handle, list_type, request_id) == 0
        }
    }

    pub fn request_facility_data_ex1(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        icao: &str,
        region: Option<&str>,
        type_: Option<i8>,
    ) -> bool {
        let icao = cstring(icao);
        let region = region.map(|s| cstring(s));
        unsafe {
            SimConnect_RequestFacilityData_EX1(
                self.handle,
                define_id,
                request_id,
                icao.as_ptr(),
                region.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                type_.unwrap_or(0) as ::std::os::raw::c_char,
            ) == 0
        }
    }

    pub unsafe fn request_jetway_data(
        &self,
        airport_icao: &str,
        array_count: DWORD,
        indexes: *mut i32,
    ) -> bool {
        let icao = cstring(airport_icao);
        unsafe {
            SimConnect_RequestJetwayData(self.handle, icao.as_ptr(), array_count, indexes) == 0
        }
    }

    pub fn add_facility_data_definition_filter(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        filter_path: &str,
        filter_data: *const std::os::raw::c_void,
        cb_unit_size: DWORD,
    ) -> bool {
        let path = cstring(filter_path);
        unsafe {
            SimConnect_AddFacilityDataDefinitionFilter(
                self.handle,
                define_id,
                path.as_ptr(),
                cb_unit_size,
                filter_data as *mut _,
            ) == 0
        }
    }

    pub fn clear_all_facility_data_definition_filters(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
    ) -> bool {
        unsafe {
            SimConnect_ClearAllFacilityDataDefinitionFilters(self.handle, define_id) == 0
        }
    }

    // ==================== Camera ====================

    pub fn camera_set_relative_6dof(
        &self,
        delta_x: f32,
        delta_y: f32,
        delta_z: f32,
        pitch: f32,
        bank: f32,
        heading: f32,
    ) -> bool {
        unsafe {
            SimConnect_CameraSetRelative6DOF(
                self.handle, delta_x, delta_y, delta_z, pitch, bank, heading,
            ) == 0
        }
    }

    // ==================== Menu ====================

    pub fn menu_add_item(
        &self,
        menu_item: &str,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        data: DWORD,
    ) -> bool {
        let item = cstring(menu_item);
        unsafe { SimConnect_MenuAddItem(self.handle, item.as_ptr(), event_id, data) == 0 }
    }

    pub fn menu_add_sub_item(
        &self,
        menu_event_id: SIMCONNECT_CLIENT_EVENT_ID,
        menu_item: &str,
        sub_menu_event_id: SIMCONNECT_CLIENT_EVENT_ID,
        data: DWORD,
    ) -> bool {
        let item = cstring(menu_item);
        unsafe {
            SimConnect_MenuAddSubItem(self.handle, menu_event_id, item.as_ptr(), sub_menu_event_id, data) == 0
        }
    }

    pub fn menu_delete_item(&self, event_id: SIMCONNECT_CLIENT_EVENT_ID) -> bool {
        unsafe { SimConnect_MenuDeleteItem(self.handle, event_id) == 0 }
    }

    pub fn menu_delete_sub_item(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        sub_event_id: SIMCONNECT_CLIENT_EVENT_ID,
    ) -> bool {
        unsafe { SimConnect_MenuDeleteSubItem(self.handle, event_id, sub_event_id) == 0 }
    }

    // ==================== Other ====================

    pub fn enumerate_controllers(&self) -> bool {
        unsafe { SimConnect_EnumerateControllers(self.handle) == 0 }
    }

    pub fn enumerate_input_events(&self, request_id: SIMCONNECT_DATA_REQUEST_ID) -> bool {
        unsafe { SimConnect_EnumerateInputEvents(self.handle, request_id) == 0 }
    }

    pub fn get_input_event(&self, request_id: SIMCONNECT_DATA_REQUEST_ID, hash: u64) -> bool {
        unsafe { SimConnect_GetInputEvent(self.handle, request_id, hash) == 0 }
    }

    pub unsafe fn set_input_event(
        &self,
        hash: u64,
        unit_size: DWORD,
        value: *mut std::os::raw::c_void,
    ) -> bool {
        unsafe { SimConnect_SetInputEvent(self.handle, hash, unit_size, value) == 0 }
    }

    pub fn subscribe_input_event(&self, hash: u64) -> bool {
        unsafe { SimConnect_SubscribeInputEvent(self.handle, hash) == 0 }
    }

    pub fn unsubscribe_input_event(&self, hash: u64) -> bool {
        unsafe { SimConnect_UnsubscribeInputEvent(self.handle, hash) == 0 }
    }

    pub fn enumerate_input_event_params(&self, hash: u64) -> bool {
        unsafe { SimConnect_EnumerateInputEventParams(self.handle, hash) == 0 }
    }

    pub unsafe fn execute_action(
        &self,
        request_id: DWORD,
        action_id: &str,
        unit_size: DWORD,
        param_values: *mut std::os::raw::c_void,
    ) -> bool {
        let action = cstring(action_id);
        unsafe {
            SimConnect_ExecuteAction(self.handle, request_id, action.as_ptr(), unit_size, param_values) == 0
        }
    }

    pub unsafe fn text(
        &self,
        text_type: SIMCONNECT_TEXT_TYPE,
        time_in_seconds: f32,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        unit_size: DWORD,
        data_set: *mut std::os::raw::c_void,
    ) -> bool {
        unsafe {
            SimConnect_Text(self.handle, text_type, time_in_seconds, event_id, unit_size, data_set) == 0
        }
    }

    pub unsafe fn get_last_sent_packet_id(&self, error: *mut DWORD) -> bool {
        unsafe { SimConnect_GetLastSentPacketID(self.handle, error) == 0 }
    }

    pub unsafe fn call_dispatch(
        &self,
        dispatch_callback: DispatchProc,
        context: *mut std::os::raw::c_void,
    ) -> bool {
        unsafe { SimConnect_CallDispatch(self.handle, dispatch_callback, context) == 0 }
    }

    pub unsafe fn request_response_times(&self, count: DWORD, elapsed_seconds: *mut f32) -> bool {
        unsafe { SimConnect_RequestResponseTimes(self.handle, count, elapsed_seconds) == 0 }
    }

    // ==================== String Utilities ====================

    pub unsafe fn retrieve_string(
        p_data: *mut SIMCONNECT_RECV,
        cb_data: DWORD,
        p_string_v: *mut std::os::raw::c_void,
        psz_string: *mut *mut std::os::raw::c_char,
        pcb_string: *mut DWORD,
    ) -> bool {
        unsafe {
            SimConnect_RetrieveString(p_data, cb_data, p_string_v, psz_string, pcb_string) == 0
        }
    }

    pub unsafe fn insert_string(
        p_dest: *mut std::os::raw::c_char,
        cb_dest: DWORD,
        pp_end: *mut *mut std::os::raw::c_void,
        pcb_string: *mut DWORD,
        p_source: *const std::os::raw::c_char,
    ) -> bool {
        unsafe {
            SimConnect_InsertString(p_dest, cb_dest, pp_end, pcb_string, p_source) == 0
        }
    }

    // ==================== Main Message Loop ====================

    pub fn get_next_message(&self) -> Result<DispatchResult<'_>, &str> {
        unsafe {
            let mut data_buf: *mut SIMCONNECT_RECV = ptr::null_mut();
            let mut size_buf: DWORD = 0;

            if SimConnect_GetNextDispatch(self.handle, &mut data_buf, &mut size_buf) != 0 {
                return Err("Failed getting data from SimConnect");
            }

            if data_buf.is_null() {
                return Ok(DispatchResult::Null);
            }

            match (*data_buf).dwID as SIMCONNECT_RECV_ID {
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NULL => Ok(DispatchResult::Null),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EXCEPTION => Ok(DispatchResult::Exception(&*(data_buf as *const SIMCONNECT_RECV_EXCEPTION))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_OPEN => Ok(DispatchResult::Open(&*(data_buf as *const SIMCONNECT_RECV_OPEN))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_QUIT => Ok(DispatchResult::Quit(&*(data_buf as *const SIMCONNECT_RECV_QUIT))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT => Ok(DispatchResult::Event(&*(data_buf as *const SIMCONNECT_RECV_EVENT))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_OBJECT_ADDREMOVE => Ok(DispatchResult::EventObjectAddRemove(&*(data_buf as *const SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_FILENAME => Ok(DispatchResult::EventFilename(&*(data_buf as *const SIMCONNECT_RECV_EVENT_FILENAME))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_FRAME => Ok(DispatchResult::EventFrame(&*(data_buf as *const SIMCONNECT_RECV_EVENT_FRAME))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA => Ok(DispatchResult::SimObjectData(&*(data_buf as *const SIMCONNECT_RECV_SIMOBJECT_DATA))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA_BYTYPE => Ok(DispatchResult::SimObjectDataByType(&*(data_buf as *const SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_WEATHER_OBSERVATION => Ok(DispatchResult::WeatherObservation(&*(data_buf as *const SIMCONNECT_RECV_WEATHER_OBSERVATION))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CLOUD_STATE => Ok(DispatchResult::CloudState(&*(data_buf as *const SIMCONNECT_RECV_CLOUD_STATE))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_ASSIGNED_OBJECT_ID => Ok(DispatchResult::AssignedObjectId(&*(data_buf as *const SIMCONNECT_RECV_ASSIGNED_OBJECT_ID))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_RESERVED_KEY => Ok(DispatchResult::ReservedKey(&*(data_buf as *const SIMCONNECT_RECV_RESERVED_KEY))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CUSTOM_ACTION => Ok(DispatchResult::CustomAction(&*(data_buf as *const SIMCONNECT_RECV_CUSTOM_ACTION))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SYSTEM_STATE => Ok(DispatchResult::SystemState(&*(data_buf as *const SIMCONNECT_RECV_SYSTEM_STATE))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CLIENT_DATA => Ok(DispatchResult::ClientData(&*(data_buf as *const SIMCONNECT_RECV_CLIENT_DATA))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_WEATHER_MODE => Ok(DispatchResult::EventWeatherMode(&*(data_buf as *const SIMCONNECT_RECV_EVENT_WEATHER_MODE))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_AIRPORT_LIST => Ok(DispatchResult::AirportList(&*(data_buf as *const SIMCONNECT_RECV_AIRPORT_LIST))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_VOR_LIST => Ok(DispatchResult::VorList(&*(data_buf as *const SIMCONNECT_RECV_VOR_LIST))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NDB_LIST => Ok(DispatchResult::NdbList(&*(data_buf as *const SIMCONNECT_RECV_NDB_LIST))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_WAYPOINT_LIST => Ok(DispatchResult::WaypointList(&*(data_buf as *const SIMCONNECT_RECV_WAYPOINT_LIST))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_MULTIPLAYER_SERVER_STARTED => Ok(DispatchResult::EventMultiplayerServerStarted(&*(data_buf as *const SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_MULTIPLAYER_CLIENT_STARTED => Ok(DispatchResult::EventMultiplayerClientStarted(&*(data_buf as *const SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_MULTIPLAYER_SESSION_ENDED => Ok(DispatchResult::EventMultiplayerSessionEnded(&*(data_buf as *const SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_RACE_END => Ok(DispatchResult::EventRaceEnd(&*(data_buf as *const SIMCONNECT_RECV_EVENT_RACE_END))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_RACE_LAP => Ok(DispatchResult::EventRaceLap(&*(data_buf as *const SIMCONNECT_RECV_EVENT_RACE_LAP))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_EX1 => Ok(DispatchResult::EventEx1(&*(data_buf as *const SIMCONNECT_RECV_EVENT_EX1))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_FACILITY_DATA => Ok(DispatchResult::FacilityData(&*(data_buf as *const SIMCONNECT_RECV_FACILITY_DATA))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_FACILITY_DATA_END => Ok(DispatchResult::FacilityDataEnd(&*(data_buf as *const SIMCONNECT_RECV_FACILITY_DATA_END))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_FACILITY_MINIMAL_LIST => Ok(DispatchResult::FacilityMinimalList(&*(data_buf as *const SIMCONNECT_RECV_FACILITY_MINIMAL_LIST))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_JETWAY_DATA => Ok(DispatchResult::JetwayData(&*(data_buf as *const SIMCONNECT_RECV_JETWAY_DATA))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_ACTION_CALLBACK => Ok(DispatchResult::ActionCallback(&*(data_buf as *const SIMCONNECT_RECV_ACTION_CALLBACK))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_ENUMERATE_INPUT_EVENTS => Ok(DispatchResult::EnumerateInputEvents(&*(data_buf as *const SIMCONNECT_RECV_ENUMERATE_INPUT_EVENTS))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_GET_INPUT_EVENT => Ok(DispatchResult::GetInputEvent(&*(data_buf as *const SIMCONNECT_RECV_GET_INPUT_EVENT))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SUBSCRIBE_INPUT_EVENT => Ok(DispatchResult::SubscribeInputEvent(&*(data_buf as *const SIMCONNECT_RECV_SUBSCRIBE_INPUT_EVENT))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_ENUMERATE_INPUT_EVENT_PARAMS => Ok(DispatchResult::EnumerateInputEventParams(&*(data_buf as *const SIMCONNECT_RECV_ENUMERATE_INPUT_EVENT_PARAMS))),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CONTROLLERS_LIST => Ok(DispatchResult::ControllersList(&*(data_buf as *const SIMCONNECT_RECV_CONTROLLERS_LIST))),
                _ => Err("Unhandled RECV_ID"),
            }
        }
    }
}

impl Drop for SimConnector {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            let _ = self.close();
        }
    }
}

#[inline]
fn cstring(s: &str) -> CString {
    CString::new(s).expect("string contained interior nul byte")
}