
int __thiscall FUN_0050bc60(void *this,void *param_1)

{
  int iVar1;
  void *this_00;
  int iVar2;
  int iVar3;
  uint uVar4;
  int iVar5;
  void *pvVar6;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641c38;
  pvStack_c = ExceptionList;
  iVar5 = 1;
  iVar2 = 0;
  iVar3 = 0;
  ExceptionList = &pvStack_c;
  FUN_004ffe70(local_2c,this,4);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    iVar1 = FUN_0052bed0((int)local_2c);
    uVar4 = *(uint *)(iVar1 + 0x24) >> 6 & 3;
    iVar1 = FUN_0052bed0((int)local_2c);
    if ((((*(uint *)(iVar1 + 0x58) >> 1 & 1) == 0) ||
        (iVar1 = FUN_0052bed0((int)local_2c), (*(uint *)(iVar1 + 0x58) >> 2 & 1) == 0)) &&
       (iVar5 != 0)) {
      iVar5 = 1;
    }
    else {
      iVar5 = 0;
    }
    if ((iVar5 != 0) &&
       (iVar1 = FUN_0052bed0((int)local_2c), (*(uint *)(iVar1 + 0x58) >> 1 & 1) != 0)) {
      iVar1 = FUN_0052bed0((int)local_2c);
      if ((*(uint *)(iVar1 + 0x50) >> 3 & 1) == 0) {
        iVar2 = iVar2 + (uint)(uVar4 == 1);
        iVar3 = iVar3 + (uint)(uVar4 == 2);
      }
      else {
        iVar1 = 0;
        pvVar6 = param_1;
        this_00 = (void *)FUN_0052bed0((int)local_2c);
        FUN_004fe2a0(this_00,iVar1,pvVar6);
      }
    }
    FUN_005130d0((int)local_2c);
  }
  if (iVar5 != 0) {
    if ((iVar2 < 2) && (iVar3 < 2)) {
      iVar5 = 1;
    }
    else {
      iVar5 = 0;
    }
  }
  if (iVar2 == 0) {
    iVar2 = FUN_00512d00(this,1,1,param_1);
    if ((iVar2 == 0) || (iVar5 == 0)) {
      iVar5 = 0;
    }
    else {
      iVar5 = 1;
    }
  }
  if (iVar3 == 0) {
    iVar2 = FUN_00512d00(this,2,1,param_1);
    if ((iVar2 == 0) || (iVar5 == 0)) {
      iVar5 = 0;
    }
    else {
      iVar5 = 1;
    }
  }
  local_4 = 0xffffffff;
  FUN_004fff70(local_2c);
  ExceptionList = pvStack_c;
  return iVar5;
}

