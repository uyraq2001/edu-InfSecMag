use exe::imphash::imphash_resolve;
use exe::pe::VecPE;
use exe::types::{CCharString, ImportData, ImportDirectory};
use relative_path::RelativePath;
use std::collections::HashSet;
use std::env::{self, current_dir};
use std::path::PathBuf;

fn main() {
    let targets = HashSet::from([
        "DeleteIPAddress".to_string(),
        "FreeMibTable".to_string(),
        "GetAdaptersAddresses".to_string(),
        "GetAnycastIpAddressEntry".to_string(),
        "GetAnycastIpAddressTable".to_string(),
        "GetBestRoute2".to_string(),
        "GetHostNameW".to_string(),
        "GetIpAddrTable".to_string(),
        "GetIpStatisticsEx".to_string(),
        "GetUnicastIpAddressTable".to_string(),
        "IcmpCloseHandle".to_string(),
        "IcmpCreateFile".to_string(),
        "IcmpCloseHandle".to_string(),
        "IcmpSendEcho".to_string(),
        "MultinetGetConnectionPerformance".to_string(),
        "MultinetGetConnectionPerformanceW".to_string(),
        "NetAlertRaise".to_string(),
        "NetAlertRaiseEx".to_string(),
        "NetApiBufferAllocate".to_string(),
        "NetApiBufferFree".to_string(),
        "NetApiBufferReallocate".to_string(),
        "NetApiBufferSize".to_string(),
        "NetFreeAadJoinInformation".to_string(),
        "NetGetAadJoinInformation".to_string(),
        "NetAddAlternateComputerName".to_string(),
        "NetCreateProvisioningPackage".to_string(),
        "NetEnumerateComputerNames".to_string(),
        "NetGetJoinableOUs".to_string(),
        "NetGetJoinInformation".to_string(),
        "NetJoinDomain".to_string(),
        "NetProvisionComputerAccount".to_string(),
        "NetRemoveAlternateComputerName".to_string(),
        "NetRenameMachineInDomain".to_string(),
        "NetRequestOfflineDomainJoin".to_string(),
        "NetRequestProvisioningPackageInstall".to_string(),
        "NetSetPrimaryComputerName".to_string(),
        "NetUnjoinDomain".to_string(),
        "NetValidateName".to_string(),
        "NetGetAnyDCName".to_string(),
        "NetGetDCName".to_string(),
        "NetGetDisplayInformationIndex".to_string(),
        "NetQueryDisplayInformation".to_string(),
        "NetGroupAdd".to_string(),
        "NetGroupAddUser".to_string(),
        "NetGroupDel".to_string(),
        "NetGroupDelUser".to_string(),
        "NetGroupEnum".to_string(),
        "NetGroupGetInfo".to_string(),
        "NetGroupGetUsers".to_string(),
        "NetGroupSetInfo".to_string(),
        "NetGroupSetUsers".to_string(),
        "NetLocalGroupAdd".to_string(),
        "NetLocalGroupAddMembers".to_string(),
        "NetLocalGroupDel".to_string(),
        "NetLocalGroupDelMembers".to_string(),
        "NetLocalGroupEnum".to_string(),
        "NetLocalGroupGetInfo".to_string(),
        "NetLocalGroupGetMembers".to_string(),
        "NetLocalGroupSetInfo".to_string(),
        "NetLocalGroupSetMembers".to_string(),
        "NetMessageBufferSend".to_string(),
        "NetMessageNameAdd".to_string(),
        "NetMessageNameDel".to_string(),
        "NetMessageNameEnum".to_string(),
        "NetMessageNameGetInfo".to_string(),
        "NetFileClose".to_string(),
        "NetFileEnum".to_string(),
        "NetFileGetInfo".to_string(),
        "NetRemoteComputerSupports".to_string(),
        "NetRemoteTOD".to_string(),
        "NetScheduleJobAdd".to_string(),
        "NetScheduleJobDel".to_string(),
        "NetScheduleJobEnum".to_string(),
        "NetScheduleJobGetInfo".to_string(),
        "GetNetScheduleAccountInformation".to_string(),
        "SetNetScheduleAccountInformation".to_string(),
        "NetServerDiskEnum".to_string(),
        "NetServerEnum".to_string(),
        "NetServerGetInfo".to_string(),
        "NetServerSetInfo".to_string(),
        "NetServerComputerNameAdd".to_string(),
        "NetServerComputerNameDel".to_string(),
        "NetServerTransportAdd".to_string(),
        "NetServerTransportAddEx".to_string(),
        "NetServerTransportDel".to_string(),
        "NetServerTransportEnum".to_string(),
        "NetWkstaTransportEnum".to_string(),
        "NetUseAdd".to_string(),
        "NetUseDel".to_string(),
        "NetUseEnum".to_string(),
        "NetUseGetInfo".to_string(),
        "NetUserAdd".to_string(),
        "NetUserChangePassword".to_string(),
        "NetUserDel".to_string(),
        "NetUserEnum".to_string(),
        "NetUserGetGroups".to_string(),
        "NetUserGetInfo".to_string(),
        "NetUserGetLocalGroups".to_string(),
        "NetUserSetGroups".to_string(),
        "NetUserSetInfo".to_string(),
        "NetUserModalsGet".to_string(),
        "NetUserModalsSet".to_string(),
        "NetValidatePasswordPolicyFree".to_string(),
        "NetValidatePasswordPolicy".to_string(),
        "NetWkstaGetInfo".to_string(),
        "NetWkstaSetInfo".to_string(),
        "NetWkstaUserEnum".to_string(),
        "NetWkstaUserGetInfo".to_string(),
        "NetWkstaUserSetInfo".to_string(),
        "NetAccessAdd".to_string(),
        "NetAccessCheck".to_string(),
        "NetAccessDel".to_string(),
        "NetAccessEnum".to_string(),
        "NetAccessGetInfo".to_string(),
        "NetAccessGetUserPerms".to_string(),
        "NetAccessSetInfo".to_string(),
        "NetAuditClear".to_string(),
        "NetAuditRead".to_string(),
        "NetAuditWrite".to_string(),
        "NetConfigGet".to_string(),
        "NetConfigGetAll".to_string(),
        "NetConfigSet".to_string(),
        "NetErrorLogClear".to_string(),
        "NetErrorLogRead".to_string(),
        "NetErrorLogWrite".to_string(),
        "NetLocalGroupAddMember".to_string(),
        "NetLocalGroupDelMember".to_string(),
        "NetServiceControl".to_string(),
        "NetServiceEnum".to_string(),
        "NetServiceGetInfo".to_string(),
        "NetServiceInstall".to_string(),
        "NetWkstaTransportAdd".to_string(),
        "NetWkstaTransportDel".to_string(),
        "NetpwNameValidate".to_string(),
        "NetapipBufferAllocate".to_string(),
        "NetpwPathType".to_string(),
        "NetApiBufferFree".to_string(),
        "NetApiBufferAllocate".to_string(),
        "NetApiBufferReallocate".to_string(),
        "WNetAddConnection2".to_string(),
        "WNetAddConnection2W".to_string(),
        "WNetAddConnection3".to_string(),
        "WNetAddConnection3W".to_string(),
        "WNetCancelConnection".to_string(),
        "WNetCancelConnectionW".to_string(),
        "WNetCancelConnection2".to_string(),
        "WNetCancelConnection2W".to_string(),
        "WNetCloseEnum".to_string(),
        "WNetCloseEnumW".to_string(),
        "WNetConnectionDialog".to_string(),
        "WNetConnectionDialogW".to_string(),
        "WNetConnectionDialog1".to_string(),
        "WNetConnectionDialog1W".to_string(),
        "WNetDisconnectDialog".to_string(),
        "WNetDisconnectDialogW".to_string(),
        "WNetDisconnectDialog1".to_string(),
        "WNetDisconnectDialog1W".to_string(),
        "WNetEnumResource".to_string(),
        "WNetEnumResourceW".to_string(),
        "WNetGetConnection".to_string(),
        "WNetGetConnectionW".to_string(),
        "WNetGetLastError".to_string(),
        "WNetGetLastErrorW".to_string(),
        "WNetGetNetworkInformation".to_string(),
        "WNetGetNetworkInformationW".to_string(),
        "WNetGetProviderName".to_string(),
        "WNetGetProviderNameW".to_string(),
        "WNetGetResourceInformation".to_string(),
        "WNetGetResourceInformationW".to_string(),
        "WNetGetResourceParent".to_string(),
        "WNetGetResourceParentW".to_string(),
        "WNetGetUniversalName".to_string(),
        "WNetGetUniversalNameW".to_string(),
        "WNetGetUser".to_string(),
        "WNetGetUserW".to_string(),
        "WNetOpenEnum".to_string(),
        "WNetOpenEnumW".to_string(),
        "WNetRestoreConnectionW".to_string(),
        "WNetUseConnection".to_string(),
        "WNetUseConnectionW".to_string(),
    ]);

    let args: Vec<String> = env::args().collect();
    let relative_path = RelativePath::new(&args[1]);
    let root = current_dir().unwrap_or_else(|error| {
        panic!("Attempt to get current dir failed with error: \n{error:?}");
    });
    let mut full_path = PathBuf::from(&args[1]);
    if !full_path.exists() {
        full_path = relative_path.to_path(&root);
    }

    let image = VecPE::from_disk_file(full_path).unwrap();
    let import_directory = ImportDirectory::parse(&image).unwrap();

    let mut imports = HashSet::new();
    for descriptor in import_directory.descriptors {
        let dll_name = descriptor.get_name(&image).unwrap().as_str().unwrap();
        for import in descriptor.get_imports(&image).unwrap() {
            match import {
                ImportData::Ordinal(ordinal) => {
                    _ = imports.insert(imphash_resolve(dll_name, ordinal));
                }
                ImportData::ImportByName(s) => _ = imports.insert(s.to_string()),
            }
        }
    }

    let intersection = targets.intersection(&imports);
    if intersection.clone().count() == 0 {
        println!("No targets were found");
        return;
    }
    for name in intersection {
        println!("{}", name);
    }
}
