
int FUN_00420930(int param_1,void *param_2)

{
  int iVar1;
  void *pvVar2;
  int iVar3;
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b950;
  local_c = ExceptionList;
  iVar3 = 0;
  ExceptionList = &local_c;
  FUN_005f3e10(param_2,param_1);
  FUN_005f3dc0(local_24,param_1);
  local_4 = 0;
  iVar1 = FUN_005f3fa0((int)local_24);
  while ((((iVar1 != 0 && (iVar1 = FUN_005f3f80((int)local_24), iVar1 != 10)) &&
          (iVar1 = FUN_005f3f80((int)local_24), iVar1 != 0x20)) &&
         (iVar1 = FUN_005f3f80((int)local_24), iVar1 != 9))) {
    pvVar2 = FUN_005f3e70(local_24,local_18);
    local_4._0_1_ = 1;
    FUN_005f3e10(param_2,(int)pvVar2);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_005f3e00(local_18);
    iVar3 = iVar3 + 1;
    iVar1 = FUN_005f3fa0((int)local_24);
  }
  local_4 = 0xffffffff;
  FUN_005f3e00(local_24);
  ExceptionList = local_c;
  return iVar3;
}

