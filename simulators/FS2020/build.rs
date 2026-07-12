use std::{env, path::PathBuf};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = PathBuf::from(manifest_dir).join("libsrc").join("lib");

    println!("cargo:rustc-link-search={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=SimConnect");

    let handle = std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| {
            let mut builder = bindgen::Builder::default()
                .header("libsrc/include/SimConnect.hpp")
                .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
                .impl_debug(true);

            // ==================== CONSTANTS / VARS ====================
            let vars = [
                "SIMCONNECT_UNUSED",
                "SIMCONNECT_OBJECT_ID_USER",
                "SIMCONNECT_CAMERA_IGNORE_FIELD",
                "SIMCONNECT_CLIENTDATA_MAX_SIZE",
                "SIMCONNECT_GROUP_PRIORITY_HIGHEST",
                "SIMCONNECT_GROUP_PRIORITY_HIGHEST_MASKABLE",
                "SIMCONNECT_GROUP_PRIORITY_STANDARD",
                "SIMCONNECT_GROUP_PRIORITY_DEFAULT",
                "SIMCONNECT_GROUP_PRIORITY_LOWEST",
                "MAX_METAR_LENGTH",
                "MAX_THERMAL_SIZE",
                "MAX_THERMAL_RATE",
                "INITPOSITION_AIRSPEED_CRUISE",
                "INITPOSITION_AIRSPEED_KEEP",
                "SIMCONNECT_CLIENTDATATYPE_INT8",
                "SIMCONNECT_CLIENTDATATYPE_INT16",
                "SIMCONNECT_CLIENTDATATYPE_INT32",
                "SIMCONNECT_CLIENTDATATYPE_INT64",
                "SIMCONNECT_CLIENTDATATYPE_FLOAT32",
                "SIMCONNECT_CLIENTDATATYPE_FLOAT64",
                "SIMCONNECT_CLIENTDATAOFFSET_AUTO",
                "SIMCONNECT_OPEN_CONFIGINDEX_LOCAL",
                "SIMCONNECT_RECV_ID_VOR_LIST_HAS_NAV_SIGNAL",
                "SIMCONNECT_RECV_ID_VOR_LIST_HAS_LOCALIZER",
                "SIMCONNECT_RECV_ID_VOR_LIST_HAS_GLIDE_SLOPE",
                "SIMCONNECT_RECV_ID_VOR_LIST_HAS_DME",
                "SIMCONNECT_WAYPOINT_NONE",
                "SIMCONNECT_WAYPOINT_SPEED_REQUESTED",
                "SIMCONNECT_WAYPOINT_THROTTLE_REQUESTED",
                "SIMCONNECT_WAYPOINT_COMPUTE_VERTICAL_SPEED",
                "SIMCONNECT_WAYPOINT_ALTITUDE_IS_AGL",
                "SIMCONNECT_WAYPOINT_ON_GROUND",
                "SIMCONNECT_WAYPOINT_REVERSE",
                "SIMCONNECT_WAYPOINT_WRAP_TO_FIRST",
                "SIMCONNECT_WAYPOINT_ALWAYS_BACKUP",
                "SIMCONNECT_WAYPOINT_KEEP_LAST_HEADING",
                "SIMCONNECT_WAYPOINT_YIELD_TO_USER",
                "SIMCONNECT_WAYPOINT_CAN_REVERSE",
                "SIMCONNECT_EVENT_FLAG_DEFAULT",
                "SIMCONNECT_EVENT_FLAG_FAST_REPEAT_TIMER",
                "SIMCONNECT_EVENT_FLAG_SLOW_REPEAT_TIMER",
                "SIMCONNECT_EVENT_FLAG_GROUPID_IS_PRIORITY",
                "SIMCONNECT_DATA_REQUEST_FLAG_DEFAULT",
                "SIMCONNECT_DATA_REQUEST_FLAG_CHANGED",
                "SIMCONNECT_DATA_REQUEST_FLAG_TAGGED",
                "SIMCONNECT_DATA_SET_FLAG_DEFAULT",
                "SIMCONNECT_DATA_SET_FLAG_TAGGED",
                "SIMCONNECT_CREATE_CLIENT_DATA_FLAG_DEFAULT",
                "SIMCONNECT_CREATE_CLIENT_DATA_FLAG_READ_ONLY",
                "SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_DEFAULT",
                "SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_CHANGED",
                "SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_TAGGED",
                "SIMCONNECT_CLIENT_DATA_SET_FLAG_DEFAULT",
                "SIMCONNECT_CLIENT_DATA_SET_FLAG_TAGGED",
                "SIMCONNECT_VIEW_SYSTEM_EVENT_DATA_COCKPIT_2D",
                "SIMCONNECT_VIEW_SYSTEM_EVENT_DATA_COCKPIT_VIRTUAL",
                "SIMCONNECT_VIEW_SYSTEM_EVENT_DATA_ORTHOGONAL",
                "SIMCONNECT_SOUND_SYSTEM_EVENT_DATA_MASTER",
                "UNKNOWN_SENDID",
                "UNKNOWN_INDEX",
                "UNKNOWN_GROUP",
                "SIMCONNECT_CLOUD_STATE_ARRAY_WIDTH",
                "SIMCONNECT_CLOUD_STATE_ARRAY_SIZE",
                "SIMCONNECT_PICK_GROUND",
                "SIMCONNECT_PICK_AI",
                "SIMCONNECT_PICK_SCENERY",
                "SIMCONNECT_PICK_ALL",
                "SIMCONNECT_PICK_COORDSASPIXELS",
            ];

            for v in vars {
                builder = builder.allowlist_var(v);
            }

            // ==================== TYPES / ENUMS ====================
            let types = [
                "HANDLE",
                "SIMCONNECT_RECV_ID",
                "SIMCONNECT_DATATYPE",
                "SIMCONNECT_EXCEPTION",
                "SIMCONNECT_SIMOBJECT_TYPE",
                "SIMCONNECT_STATE",
                "SIMCONNECT_PERIOD",
                "SIMCONNECT_MISSION_END",
                "SIMCONNECT_CLIENT_DATA_PERIOD",
                "SIMCONNECT_TEXT_TYPE",
                "SIMCONNECT_TEXT_RESULT",
                "SIMCONNECT_WEATHER_MODE",
                "SIMCONNECT_FACILITY_LIST_TYPE",
                "SIMCONNECT_FACILITY_DATA_TYPE",
                "SIMCONNECT_INPUT_EVENT_TYPE",
                "SIMCONNECT_VOR_FLAGS",
                "SIMCONNECT_WAYPOINT_FLAGS",
                "SIMCONNECT_EVENT_FLAG",
                "SIMCONNECT_DATA_REQUEST_FLAG",
                "SIMCONNECT_DATA_SET_FLAG",
                "SIMCONNECT_CREATE_CLIENT_DATA_FLAG",
                "SIMCONNECT_CLIENT_DATA_REQUEST_FLAG",
                "SIMCONNECT_CLIENT_DATA_SET_FLAG",
                "SIMCONNECT_VIEW_SYSTEM_EVENT_DATA",
                "SIMCONNECT_SOUND_SYSTEM_EVENT_DATA",
                "SIMCONNECT_PICK_FLAGS",
            ];

            for t in types {
                builder = builder.allowlist_type(t);
            }

            // ==================== STRUCTS ====================
            let structs = [
                "SIMCONNECT_RECV",
                "SIMCONNECT_RECV_EXCEPTION",
                "SIMCONNECT_RECV_OPEN",
                "SIMCONNECT_RECV_QUIT",
                "SIMCONNECT_RECV_EVENT",
                "SIMCONNECT_RECV_EVENT_FILENAME",
                "SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE",
                "SIMCONNECT_RECV_EVENT_FRAME",
                "SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED",
                "SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED",
                "SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED",
                "SIMCONNECT_RECV_EVENT_RACE_END",
                "SIMCONNECT_RECV_EVENT_RACE_LAP",
                "SIMCONNECT_RECV_SIMOBJECT_DATA",
                "SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE",
                "SIMCONNECT_RECV_CLIENT_DATA",
                "SIMCONNECT_RECV_WEATHER_OBSERVATION",
                "SIMCONNECT_RECV_CLOUD_STATE",
                "SIMCONNECT_RECV_ASSIGNED_OBJECT_ID",
                "SIMCONNECT_RECV_RESERVED_KEY",
                "SIMCONNECT_RECV_SYSTEM_STATE",
                "SIMCONNECT_RECV_CUSTOM_ACTION",
                "SIMCONNECT_RECV_EVENT_WEATHER_MODE",
                "SIMCONNECT_RECV_FACILITIES_LIST",
                "SIMCONNECT_DATA_FACILITY_AIRPORT",
                "SIMCONNECT_RECV_AIRPORT_LIST",
                "SIMCONNECT_DATA_FACILITY_WAYPOINT",
                "SIMCONNECT_RECV_WAYPOINT_LIST",
                "SIMCONNECT_DATA_FACILITY_NDB",
                "SIMCONNECT_RECV_NDB_LIST",
                "SIMCONNECT_DATA_FACILITY_VOR",
                "SIMCONNECT_RECV_VOR_LIST",
                "SIMCONNECT_RECV_EVENT_EX1",
                "SIMCONNECT_DATA_RACE_RESULT",
                "SIMCONNECT_RECV_FACILITY_DATA",
                "SIMCONNECT_RECV_FACILITY_DATA_END",
                "SIMCONNECT_ICAO",
                "SIMCONNECT_FACILITY_MINIMAL",
                "SIMCONNECT_RECV_FACILITY_MINIMAL_LIST",
                "SIMCONNECT_DATA_PBH",
                "SIMCONNECT_JETWAY_DATA",
                "SIMCONNECT_RECV_JETWAY_DATA",
                "SIMCONNECT_RECV_ACTION_CALLBACK",
                "SIMCONNECT_INPUT_EVENT_DESCRIPTOR",
                "SIMCONNECT_RECV_ENUMERATE_INPUT_EVENTS",
                "SIMCONNECT_RECV_GET_INPUT_EVENT",
                "SIMCONNECT_RECV_SUBSCRIBE_INPUT_EVENT",
                "SIMCONNECT_RECV_ENUMERATE_INPUT_EVENT_PARAMS",
                "SIMCONNECT_VERSION_BASE_TYPE",
                "SIMCONNECT_CONTROLLER_ITEM",
                "SIMCONNECT_RECV_CONTROLLERS_LIST",
                "SIMCONNECT_RECV_LIST_TEMPLATE",
                "SIMCONNECT_DATA_INITPOSITION",
                "SIMCONNECT_DATA_MARKERSTATE",
                "SIMCONNECT_DATA_WAYPOINT",
                "SIMCONNECT_DATA_LATLONALT",
                "SIMCONNECT_DATA_XYZ",
                "SIMCONNECT_RECV_PICK",
            ];

            for s in structs {
                builder = builder.allowlist_type(s);
            }

            // ==================== FUNCTIONS ====================
            let functions = [
                "SimConnect_MapClientEventToSimEvent",
                "SimConnect_TransmitClientEvent",
                "SimConnect_SetSystemEventState",
                "SimConnect_AddClientEventToNotificationGroup",
                "SimConnect_RemoveClientEvent",
                "SimConnect_SetNotificationGroupPriority",
                "SimConnect_ClearNotificationGroup",
                "SimConnect_RequestNotificationGroup",
                "SimConnect_AddToDataDefinition",
                "SimConnect_ClearDataDefinition",
                "SimConnect_RequestDataOnSimObject",
                "SimConnect_RequestDataOnSimObjectType",
                "SimConnect_SetDataOnSimObject",
                "SimConnect_MapInputEventToClientEvent",
                "SimConnect_SetInputGroupPriority",
                "SimConnect_RemoveInputEvent",
                "SimConnect_ClearInputGroup",
                "SimConnect_SetInputGroupState",
                "SimConnect_RequestReservedKey",
                "SimConnect_SubscribeToSystemEvent",
                "SimConnect_UnsubscribeFromSystemEvent",
                "SimConnect_WeatherRequestInterpolatedObservation",
                "SimConnect_WeatherRequestObservationAtStation",
                "SimConnect_WeatherRequestObservationAtNearestStation",
                "SimConnect_WeatherCreateStation",
                "SimConnect_WeatherRemoveStation",
                "SimConnect_WeatherSetObservation",
                "SimConnect_WeatherSetModeServer",
                "SimConnect_WeatherSetModeTheme",
                "SimConnect_WeatherSetModeGlobal",
                "SimConnect_WeatherSetModeCustom",
                "SimConnect_WeatherSetDynamicUpdateRate",
                "SimConnect_WeatherRequestCloudState",
                "SimConnect_WeatherCreateThermal",
                "SimConnect_WeatherRemoveThermal",
                "SimConnect_AICreateParkedATCAircraft",
                "SimConnect_AICreateEnrouteATCAircraft",
                "SimConnect_AICreateNonATCAircraft",
                "SimConnect_AICreateSimulatedObject",
                "SimConnect_AIReleaseControl",
                "SimConnect_AIRemoveObject",
                "SimConnect_AISetAircraftFlightPlan",
                "SimConnect_ExecuteMissionAction",
                "SimConnect_CompleteCustomMissionAction",
                "SimConnect_Close",
                "SimConnect_RetrieveString",
                "SimConnect_GetLastSentPacketID",
                "SimConnect_Open",
                "SimConnect_CallDispatch",
                "SimConnect_GetNextDispatch",
                "SimConnect_RequestResponseTimes",
                "SimConnect_InsertString",
                "SimConnect_CameraSetRelative6DOF",
                "SimConnect_MenuAddItem",
                "SimConnect_MenuDeleteItem",
                "SimConnect_MenuAddSubItem",
                "SimConnect_MenuDeleteSubItem",
                "SimConnect_RequestSystemState",
                "SimConnect_SetSystemState",
                "SimConnect_MapClientDataNameToID",
                "SimConnect_CreateClientData",
                "SimConnect_AddToClientDataDefinition",
                "SimConnect_ClearClientDataDefinition",
                "SimConnect_RequestClientData",
                "SimConnect_SetClientData",
                "SimConnect_FlightLoad",
                "SimConnect_FlightSave",
                "SimConnect_FlightPlanLoad",
                "SimConnect_Text",
                "SimConnect_SubscribeToFacilities",
                "SimConnect_UnsubscribeToFacilities",
                "SimConnect_RequestFacilitiesList",
                "SimConnect_TransmitClientEvent_EX1",
                "SimConnect_AddToFacilityDefinition",
                "SimConnect_RequestFacilityData",
                "SimConnect_SubscribeToFacilities_EX1",
                "SimConnect_UnsubscribeToFacilities_EX1",
                "SimConnect_RequestFacilitiesList_EX1",
                "SimConnect_RequestFacilityData_EX1",
                "SimConnect_RequestJetwayData",
                "SimConnect_EnumerateControllers",
                "SimConnect_MapInputEventToClientEvent_EX1",
                "SimConnect_ExecuteAction",
                "SimConnect_EnumerateInputEvents",
                "SimConnect_GetInputEvent",
                "SimConnect_SetInputEvent",
                "SimConnect_SubscribeInputEvent",
                "SimConnect_UnsubscribeInputEvent",
                "SimConnect_EnumerateInputEventParams",
                "SimConnect_AddFacilityDataDefinitionFilter",
                "SimConnect_ClearAllFacilityDataDefinitionFilters",
            ];

            for f in functions {
                builder = builder.allowlist_function(f);
            }

            let bindings = builder
                .generate()
                .expect("Unable to generate bindings");

            let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
            bindings
                .write_to_file(out_path.join("bindings.rs"))
                .expect("Couldn't write bindings!");
        })
        .expect("Failed to spawn bindgen thread");

    handle.join().expect("Bindgen thread panicked");
}
