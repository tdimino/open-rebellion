
bool __thiscall FUN_00502840(void *param_1,undefined4 param_2,void *param_3)

{
  int iVar1;
  uint uVar2;
  int *piVar3;
  bool bVar4;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006409d8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar1 = FUN_00557b90(param_1,param_2,param_3);
  bVar4 = iVar1 != 0;
  uVar2 = *(uint *)((int)param_1 + 0x40) & 0xff;
  if ((uVar2 != 0) && (uVar2 < 0x11)) {
    FUN_004f25c0(auStack_2c,param_1,3);
    uStack_4 = 0;
    FUN_00513120((int)auStack_2c);
    while (iStack_10 != 0) {
      iVar1 = FUN_0052bed0((int)auStack_2c);
      if ((*(uint *)(iVar1 + 0x50) >> 5 & 1) == 0) {
        piVar3 = (int *)FUN_0052bed0((int)auStack_2c);
        iVar1 = (**(code **)(*piVar3 + 0x2e8))(param_3);
        if ((iVar1 == 0) || (bVar4 == false)) {
          bVar4 = false;
        }
        else {
          bVar4 = true;
        }
      }
      FUN_005130d0((int)auStack_2c);
    }
    uStack_4 = 0xffffffff;
    FUN_004f26c0(auStack_2c);
  }
  ExceptionList = pvStack_c;
  return bVar4;
}

