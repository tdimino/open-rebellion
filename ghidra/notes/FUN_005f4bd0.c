
BOOL __cdecl FUN_005f4bd0(char *param_1)

{
  int iVar1;
  LPCSTR lpFileName;
  BOOL BVar2;
  undefined4 local_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655758;
  pvStack_c = ExceptionList;
  BVar2 = 0;
  ExceptionList = &pvStack_c;
  FUN_005f35b0(local_18,param_1);
  local_4 = 0;
  iVar1 = FUN_00583c40((int)local_18);
  if (iVar1 != 0) {
    lpFileName = (LPCSTR)FUN_00583c40((int)local_18);
    BVar2 = DeleteFileA(lpFileName);
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = pvStack_c;
  return BVar2;
}

