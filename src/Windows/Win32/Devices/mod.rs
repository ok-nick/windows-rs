#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Devices_AllJoyn")]
pub mod AllJoyn;
#[cfg(feature = "Win32_Devices_BiometricFramework")]
pub mod BiometricFramework;
#[cfg(feature = "Win32_Devices_Bluetooth")]
pub mod Bluetooth;
#[cfg(feature = "Win32_Devices_Communication")]
pub mod Communication;
#[cfg(feature = "Win32_Devices_DeviceAccess")]
pub mod DeviceAccess;
#[cfg(feature = "Win32_Devices_DeviceAndDriverInstallation")]
pub mod DeviceAndDriverInstallation;
#[cfg(feature = "Win32_Devices_DeviceQuery")]
pub mod DeviceQuery;
#[cfg(feature = "Win32_Devices_Display")]
pub mod Display;
#[cfg(feature = "Win32_Devices_Enumeration")]
pub mod Enumeration;
#[cfg(feature = "Win32_Devices_Fax")]
pub mod Fax;
#[cfg(feature = "Win32_Devices_FunctionDiscovery")]
pub mod FunctionDiscovery;
#[cfg(feature = "Win32_Devices_Geolocation")]
pub mod Geolocation;
#[cfg(feature = "Win32_Devices_HumanInterfaceDevice")]
pub mod HumanInterfaceDevice;
#[cfg(feature = "Win32_Devices_ImageAcquisition")]
pub mod ImageAcquisition;
#[cfg(feature = "Win32_Devices_PortableDevices")]
pub mod PortableDevices;
#[cfg(feature = "Win32_Devices_Properties")]
pub mod Properties;
#[cfg(feature = "Win32_Devices_Pwm")]
pub mod Pwm;
#[cfg(feature = "Win32_Devices_Sensors")]
pub mod Sensors;
#[cfg(feature = "Win32_Devices_SerialCommunication")]
pub mod SerialCommunication;
#[cfg(feature = "Win32_Devices_Tapi")]
pub mod Tapi;
#[cfg(feature = "Win32_Devices_Usb")]
pub mod Usb;
#[cfg(feature = "Win32_Devices_WebServicesOnDevices")]
pub mod WebServicesOnDevices;
