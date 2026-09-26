
bool __thiscall FUN_004fe540(void *this,void *param_1)

{
  uint uVar1;
  bool bVar2;
  bool bVar3;
  int iVar4;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006404c8;
  pvStack_c = ExceptionList;
  bVar3 = false;
  ExceptionList = &pvStack_c;
  FUN_00502e30(local_2c,this,3,*(uint *)((int)this + 0x24) >> 6 & 3);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while ((bVar2 = bVar3, local_10 != 0 && (bVar2 = true, !bVar3))) {
    iVar4 = FUN_0052bed0((int)local_2c);
    if (((*(uint *)(iVar4 + 0x50) >> 2 & 1) == 0) ||
       (iVar4 = FUN_0052bed0((int)local_2c), (*(uint *)(iVar4 + 0x50) >> 5 & 1) != 0)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    FUN_005130d0((int)local_2c);
  }
  uVar1 = *(uint *)((int)this + 0x50);
  if (((((uVar1 & 0x40) == 0) || ((uVar1 & 4) == 0)) || ((uVar1 & 0x10) != 0)) || (!bVar2)) {
    iVar4 = 0;
  }
  else {
    iVar4 = 1;
  }
  iVar4 = FUN_004f7410(this,iVar4,param_1);
  local_4 = 0xffffffff;
  FUN_00502eb0(local_2c);
  ExceptionList = pvStack_c;
  return iVar4 != 0;
}

