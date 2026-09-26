
void __thiscall FUN_005f4610(void *this,int param_1)

{
  int iVar1;
  undefined4 local_28;
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006556c0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f3de0(local_24,param_1);
  local_4 = 0;
  iVar1 = FUN_005f3fa0((int)local_24);
  while (iVar1 != 0) {
    FUN_005f42c0(local_24);
    iVar1 = FUN_005f3fa0((int)local_24);
    if (iVar1 != 0) {
      FUN_005f2f50(local_18);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_005f4350((int)local_24,local_18);
      local_28 = 0;
      iVar1 = FUN_005f4720(this,(int)local_18,&local_28);
      if (iVar1 == 1) {
        param_1 = 0;
        FUN_005f43c0((int)local_24,&param_1);
        FUN_005f45b0(this,(int)local_18,param_1);
      }
      local_4 = local_4 & 0xffffff00;
      FUN_005f2ff0(local_18);
    }
    FUN_005f42c0(local_24);
    iVar1 = FUN_005f3fa0((int)local_24);
  }
  local_4 = 0xffffffff;
  FUN_005f3e00(local_24);
  ExceptionList = local_c;
  return;
}

