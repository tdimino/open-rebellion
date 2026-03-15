
void __fastcall FUN_005fb980(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00655e68;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0066de78;
  local_4 = 0;
  if (DAT_006be5a0 != (HGDIOBJ)0x0) {
    DeleteObject(DAT_006be5a0);
  }
  if ((undefined *)param_1[0x25] != (undefined *)0x0) {
    FUN_00618b60((undefined *)param_1[0x25]);
  }
  DAT_006be3b8 = 0;
  FUN_00601f10();
  local_4 = 0xffffffff;
  FUN_005ffb60(param_1);
  ExceptionList = local_c;
  return;
}

