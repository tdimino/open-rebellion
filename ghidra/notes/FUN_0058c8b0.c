
bool __thiscall FUN_0058c8b0(void *this,void *param_1)

{
  int iVar1;
  void *this_00;
  int iVar2;
  void *pvVar3;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064ff78;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar1 = FUN_0050a6d0(*(void **)((int)this + 0x10),0,param_1);
  FUN_004ffef0(local_2c,*(undefined4 *)((int)this + 0x10),4,*(undefined4 *)((int)this + 0x14));
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    iVar2 = 0;
    pvVar3 = param_1;
    this_00 = (void *)FUN_0052bed0((int)local_2c);
    FUN_004fe4d0(this_00,iVar2,pvVar3);
    FUN_005130d0((int)local_2c);
  }
  local_4 = 0xffffffff;
  FUN_004fff70(local_2c);
  ExceptionList = pvStack_c;
  return iVar1 != 0;
}

