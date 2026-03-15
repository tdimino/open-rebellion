
bool FUN_00416950(void)

{
  LPCSTR pCVar1;
  LPBYTE _Source;
  UINT UVar2;
  HANDLE hFindFile;
  int iVar3;
  bool bVar4;
  _WIN32_FIND_DATAA *lpFindFileData;
  char *pcVar5;
  undefined4 local_28c [3];
  undefined4 local_280 [3];
  undefined4 local_274 [3];
  undefined4 local_268 [3];
  undefined4 auStack_25c [3];
  char local_250 [260];
  _WIN32_FIND_DATAA _Stack_14c;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062adb8;
  pvStack_c = ExceptionList;
  bVar4 = false;
  ExceptionList = &pvStack_c;
  FUN_005f2f50(local_268);
  local_4 = 0;
  FUN_005f35b0(local_280,s_Star_Wars_Rebellion_006a7c50);
  local_4._0_1_ = 1;
  FUN_005f35b0(local_274,&DAT_006a7c4c);
  local_4._0_1_ = 2;
  FUN_005f35b0(local_28c,s_SOFTWARE_LucasArts_Entertainment_006a7c1c);
  local_4._0_1_ = 3;
  FUN_005f30d0(local_268,(int)local_28c);
  FUN_005f30d0(local_268,(int)local_274);
  FUN_005f30d0(local_268,(int)local_280);
  local_4._0_1_ = 2;
  FUN_005f2ff0(local_28c);
  local_4._0_1_ = 1;
  FUN_005f2ff0(local_274);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(local_280);
  pcVar5 = s_TooBigLocation_006a7c74;
  pCVar1 = (LPCSTR)FUN_00583c40((int)local_268);
  _Source = FUN_005f3fc0(pCVar1,pcVar5);
  if (_Source != (LPBYTE)0x0) {
    pcVar5 = local_250;
    for (iVar3 = 0x41; iVar3 != 0; iVar3 = iVar3 + -1) {
      pcVar5[0] = '\0';
      pcVar5[1] = '\0';
      pcVar5[2] = '\0';
      pcVar5[3] = '\0';
      pcVar5 = pcVar5 + 4;
    }
    _strncpy(local_250,(char *)_Source,3);
    UVar2 = GetDriveTypeA(local_250);
    if (UVar2 == 3) {
      FUN_005f2f50(local_28c);
      local_4._0_1_ = 4;
      FUN_005f35b0(auStack_25c,s_MDATA_MDATA_000_006a8460);
      local_4._0_1_ = 5;
      FUN_005f35b0(local_274,&DAT_006a7c4c);
      local_4._0_1_ = 6;
      FUN_005f35b0(local_280,(char *)_Source);
      local_4._0_1_ = 7;
      FUN_005f30d0(local_28c,(int)local_280);
      FUN_005f30d0(local_28c,(int)local_274);
      FUN_005f30d0(local_28c,(int)auStack_25c);
      local_4._0_1_ = 6;
      FUN_005f2ff0(local_280);
      local_4._0_1_ = 5;
      FUN_005f2ff0(local_274);
      local_4 = CONCAT31(local_4._1_3_,4);
      FUN_005f2ff0(auStack_25c);
      lpFindFileData = &_Stack_14c;
      pCVar1 = (LPCSTR)FUN_00583c40((int)local_28c);
      hFindFile = FindFirstFileA(pCVar1,lpFindFileData);
      bVar4 = hFindFile != (HANDLE)0xffffffff;
      if (bVar4) {
        FindClose(hFindFile);
      }
      local_4 = local_4 & 0xffffff00;
      FUN_005f2ff0(local_28c);
    }
    FUN_00618b60(_Source);
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_268);
  ExceptionList = pvStack_c;
  return bVar4;
}

