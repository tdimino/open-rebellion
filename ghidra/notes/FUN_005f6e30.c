
void __fastcall FUN_005f6e30(undefined4 *param_1)

{
  int iVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00655838;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0066ddb0;
  local_4 = 0;
  FUN_005f7190((int)param_1);
  if (param_1[0x70] != 0) {
    iVar1 = FUN_00610c10();
    if (iVar1 == 0) {
      FUN_00610be0(param_1[6],1,0);
    }
    FUN_00610ce0(param_1[0x6f]);
    FUN_00610c00(0);
  }
  local_4 = 0xffffffff;
  FUN_005ffb60(param_1);
  ExceptionList = local_c;
  return;
}

