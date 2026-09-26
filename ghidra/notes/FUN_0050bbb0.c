
int __thiscall FUN_0050bbb0(void *this,void *param_1)

{
  uint uVar1;
  void *this_00;
  int iVar2;
  uint uVar3;
  void *pvVar4;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641c18;
  pvStack_c = ExceptionList;
  uVar1 = *(uint *)((int)this + 0x88);
  iVar2 = 1;
  ExceptionList = &pvStack_c;
  FUN_0053b6e0(local_2c,this,3);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    uVar3 = (uint)((uVar1 & 0x24) != 0);
    pvVar4 = param_1;
    this_00 = (void *)FUN_0052bed0((int)local_2c);
    iVar2 = FUN_0053a7f0(this_00,uVar3,pvVar4);
    FUN_005130d0((int)local_2c);
  }
  local_4 = 0xffffffff;
  FUN_0053b7e0(local_2c);
  ExceptionList = pvStack_c;
  return iVar2;
}

