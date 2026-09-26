
void __thiscall FUN_0058c1e0(void *this,int param_1,undefined4 param_2,void *param_3)

{
  int iVar1;
  uint uVar2;
  bool bVar3;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064fea8;
  pvStack_c = ExceptionList;
  bVar3 = true;
  uVar2 = 0;
  ExceptionList = &pvStack_c;
  FUN_004ffe70(local_2c,param_1,1);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  do {
    if (local_10 == 0) break;
    iVar1 = FUN_0052bed0((int)local_2c);
    if ((*(uint *)(iVar1 + 0x58) >> 7 & 1) != 0) {
      if (uVar2 == 0) {
        iVar1 = FUN_0052bed0((int)local_2c);
        uVar2 = *(uint *)(iVar1 + 0x24) >> 6 & 3;
      }
      else {
        iVar1 = FUN_0052bed0((int)local_2c);
        bVar3 = uVar2 == (*(uint *)(iVar1 + 0x24) >> 6 & 3);
      }
    }
    FUN_005130d0((int)local_2c);
  } while (bVar3);
  local_4 = 0xffffffff;
  FUN_004fff70(local_2c);
  *(int *)((int)this + 0x10) = param_1;
  *(uint *)((int)this + 0x14) = uVar2;
  *(uint *)((int)this + 0x18) = *(uint *)(param_1 + 0x24) >> 6 & 3;
  FUN_0058c120(this,param_3);
  ExceptionList = pvStack_c;
  return;
}

