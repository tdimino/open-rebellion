
bool __fastcall FUN_0058a6c0(void *param_1)

{
  int iVar1;
  undefined4 uVar2;
  bool bVar3;
  undefined4 uVar4;
  undefined4 auStack_4c [7];
  int iStack_30;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0064fc90;
  pvStack_c = ExceptionList;
  bVar3 = true;
  ExceptionList = &pvStack_c;
  (**(code **)(*(int *)((int)param_1 + 0x68) + 4))();
  (**(code **)(*(int *)((int)param_1 + 0x60) + 4))();
  if (*(int *)((int)param_1 + 0x20) != 0) {
    iVar1 = FUN_0058a7e0(param_1,*(int *)((int)param_1 + 0x10));
    bVar3 = iVar1 != 0;
  }
  if (*(int *)((int)param_1 + 0x24) != 0) {
    FUN_004ffe70(auStack_2c,*(undefined4 *)((int)param_1 + 0x10),1);
    uStack_4 = 0;
    FUN_00513120((int)auStack_2c);
    while (iStack_10 != 0) {
      uVar4 = 1;
      uVar2 = FUN_0052bed0((int)auStack_2c);
      FUN_00502db0(auStack_4c,uVar2,uVar4);
      uStack_4 = CONCAT31(uStack_4._1_3_,1);
      FUN_00513120((int)auStack_4c);
      while (iStack_30 != 0) {
        iVar1 = FUN_0052bed0((int)auStack_4c);
        iVar1 = FUN_0058a7e0(param_1,iVar1);
        if ((iVar1 == 0) || (!bVar3)) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
        FUN_005130d0((int)auStack_4c);
      }
      uStack_4 = uStack_4 & 0xffffff00;
      FUN_00502eb0(auStack_4c);
      FUN_005130d0((int)auStack_2c);
    }
    uStack_4 = 0xffffffff;
    FUN_004fff70(auStack_2c);
  }
  ExceptionList = pvStack_c;
  return bVar3;
}

