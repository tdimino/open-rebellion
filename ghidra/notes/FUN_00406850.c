
bool FUN_00406850(void)

{
  DWORD dwLen;
  undefined *lpData;
  BOOL BVar1;
  int iVar2;
  int iVar3;
  bool bVar4;
  ushort *local_10c;
  uint local_108;
  DWORD local_104;
  CHAR local_100 [256];
  bool bVar5;
  
  bVar5 = true;
  bVar4 = true;
  dwLen = GetFileVersionInfoSizeA(s_textstra_dll_006a7ecc,&local_104);
  if (dwLen != 0) {
    lpData = (undefined *)FUN_00618b70(dwLen);
    BVar1 = GetFileVersionInfoA(s_textstra_dll_006a7ecc,0,dwLen,lpData);
    bVar4 = bVar5;
    if (BVar1 != 0) {
      BVar1 = VerQueryValueA(lpData,s__VarFileInfo_Translation_006a7eb0,&local_10c,&local_108);
      if (BVar1 != 0) {
        VerLanguageNameA((uint)*local_10c,(LPSTR)&szLang_006b06f8,0xff);
        wsprintfA(local_100,s__StringFileInfo__04X_04X_FileVer_006a7e88,(uint)*local_10c,
                  (uint)local_10c[1]);
        BVar1 = VerQueryValueA(lpData,local_100,&local_10c,&local_108);
        if (BVar1 != 0) {
          *(undefined1 *)((int)local_10c + 1) = 0;
          *(undefined1 *)(local_10c + 2) = 0;
          iVar2 = FUN_00619600((byte *)(local_10c + 1));
          iVar3 = FUN_00619600((byte *)((int)local_10c + 5));
          bVar4 = iVar3 == 0 && iVar2 == 0;
        }
      }
    }
    FUN_00618b60(lpData);
  }
  return bVar4;
}

