
undefined4 __thiscall
FUN_005fcfe0(void *this,int param_1,int param_2,uint param_3,int param_4,undefined1 param_5)

{
  uint uVar1;
  int iVar2;
  int iVar3;
  uint uVar4;
  undefined4 *puVar5;
  
  uVar1 = FUN_005fc9c0(this);
  iVar2 = FUN_005fc110(this);
  if (iVar2 != 0) {
    return 0;
  }
  iVar2 = FUN_005fc0f0(this);
  if (iVar2 < 1) {
    iVar2 = -iVar2;
  }
  if (iVar2 < param_4 + param_2) {
    param_4 = iVar2 - param_2;
  }
  iVar2 = FUN_005fc0e0(this);
  if (iVar2 < (int)(param_3 + param_1)) {
    iVar2 = FUN_005fc0e0(this);
    param_3 = iVar2 - param_1;
  }
  if (((0 < param_4) && (0 < (int)param_3)) && (iVar2 = 0, 0 < param_4)) {
    do {
      iVar3 = FUN_005fc9e0(this,iVar2 + param_2);
      puVar5 = (undefined4 *)(uVar1 * iVar3 + param_1 + *(int *)((int)this + 4));
      for (uVar4 = param_3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
        *puVar5 = CONCAT22(CONCAT11(param_5,param_5),CONCAT11(param_5,param_5));
        puVar5 = puVar5 + 1;
      }
      iVar2 = iVar2 + 1;
      for (uVar4 = param_3 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *(undefined1 *)puVar5 = param_5;
        puVar5 = (undefined4 *)((int)puVar5 + 1);
      }
    } while (iVar2 < param_4);
  }
  FUN_005fd170(this);
  if ((0 < param_4) && (0 < (int)param_3)) {
    return 1;
  }
  return 0;
}

