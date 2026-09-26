
void __fastcall FUN_004161b0(void *param_1)

{
  int iVar1;
  undefined4 uVar2;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062aca8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (((*(byte *)((int)param_1 + 0x128) & 0x10) == 0) &&
     (ExceptionList = &local_c, *(int *)((int)param_1 + 0xcc) == 0)) {
    ExceptionList = &local_c;
    iVar1 = FUN_00401c30();
    if (iVar1 != 0) {
      FUN_00401c50(0);
      FUN_005f35b0(local_18,PTR_s_Autosave_006a81d8);
      local_4 = 0;
      *(undefined4 *)((int)param_1 + 0xcc) = 6;
      FUN_005f3090((void *)((int)param_1 + 0xd0),(int)local_18);
      uVar2 = FUN_004fcee0();
      *(undefined4 *)((int)param_1 + 0x13c) = uVar2;
      local_4 = 0xffffffff;
      FUN_005f2ff0(local_18);
    }
  }
  if (*(int *)((int)param_1 + 200) != 0) {
    FUN_00415440(param_1);
  }
  if (*(int *)((int)param_1 + 0xcc) != 0) {
    FUN_00415600(param_1);
  }
  ExceptionList = local_c;
  return;
}

