# Registry Command

mreg [type] [hkey] [regpath] (valuename for -w) (dwtype for -w) (value for -w)

    - type can be -w to write into the registry, -c to create a registry, and -d to delete a registry

    - hkey can be HKLM, HKCU, HKU, HKCR, HKCC

    - regpath is the path of the registry value you want to mess with

    - valuename is for the key you want to change (only for -w)

    - dwtype is the type you want to make the valuename. It cna be DWORD, SZ, EXPAND_SZ, and qword (only for -w)
    
    - value is what you want to write into that key (only for -w)