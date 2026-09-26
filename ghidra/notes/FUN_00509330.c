
undefined4 __thiscall FUN_00509330(void *this,int param_1,int *param_2)

{
  int iVar1;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641938;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_2 = 0;
  FUN_004f2640(local_2c,this,3,*(uint *)((int)this + 0x24) >> 6 & 3);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while ((local_10 != 0 && (*param_2 == 0))) {
    iVar1 = FUN_0052bed0((int)local_2c);
    if (*(short *)(iVar1 + 0x96) == param_1) {
      iVar1 = FUN_0052bed0((int)local_2c);
      *param_2 = iVar1;
    }
    FUN_005130d0((int)local_2c);
  }
  local_4 = 0xffffffff;
  FUN_004f26c0(local_2c);
  ExceptionList = pvStack_c;
  return 1;
}

