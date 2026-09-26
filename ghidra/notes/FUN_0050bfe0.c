
int __thiscall FUN_0050bfe0(void *this,void *param_1)

{
  int iVar1;
  void *this_00;
  uint uVar2;
  int iVar3;
  void *pvVar4;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641c78;
  pvStack_c = ExceptionList;
  iVar3 = 1;
  ExceptionList = &pvStack_c;
  FUN_004ffe70(local_2c,this,4);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    uVar2 = 0;
    iVar1 = FUN_0052bed0((int)local_2c);
    if ((*(uint *)(iVar1 + 0x50) & 1) != 0) {
      uVar2 = *(uint *)((int)this + 0x88) >> 4 & 1;
    }
    pvVar4 = param_1;
    this_00 = (void *)FUN_0052bed0((int)local_2c);
    iVar1 = FUN_004fe380(this_00,uVar2,pvVar4);
    if ((iVar1 == 0) || (iVar3 == 0)) {
      iVar3 = 0;
    }
    else {
      iVar3 = 1;
    }
    FUN_005130d0((int)local_2c);
  }
  local_4 = 0xffffffff;
  FUN_004fff70(local_2c);
  ExceptionList = pvStack_c;
  return iVar3;
}

