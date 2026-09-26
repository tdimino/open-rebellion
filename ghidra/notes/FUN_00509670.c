
undefined4 __thiscall FUN_00509670(void *this,int param_1)

{
  int *piVar1;
  int iVar2;
  undefined4 uVar3;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006419b8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_0052c170(local_2c,this,4);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    piVar1 = (int *)FUN_0052bed0((int)local_2c);
    iVar2 = (**(code **)(*piVar1 + 0x1ec))();
    if (iVar2 == param_1) break;
    FUN_005130d0((int)local_2c);
  }
  uVar3 = FUN_0052bed0((int)local_2c);
  local_4 = 0xffffffff;
  FUN_0052c1f0(local_2c);
  ExceptionList = pvStack_c;
  return uVar3;
}

