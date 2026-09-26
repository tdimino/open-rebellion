
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005b1490(int param_1)

{
  undefined4 uVar1;
  uint uVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  
  if ((*(int *)(param_1 + 0x158) != 0) && (*(float *)(param_1 + 0x3f0) != _DAT_0066c4f0)) {
    iVar5 = 100;
    iVar4 = 1;
    FUN_005aaf90();
    uVar2 = FUN_005a8a70(iVar4,iVar5);
    if ((float)uVar2 <= *(float *)(param_1 + 0x3f0)) {
      uVar2 = *(uint *)(param_1 + 0x158);
      if (uVar2 < 2) {
        uVar2 = 1;
      }
      else {
        iVar4 = 1;
        FUN_005aaf90();
        uVar2 = FUN_005a8a70(iVar4,uVar2);
      }
      if (uVar2 <= *(uint *)(param_1 + 0x36c)) {
        uVar1 = *(undefined4 *)(param_1 + 0xec);
        piVar3 = (int *)FUN_005aaf90();
        (**(code **)(*piVar3 + 0x28))(0x26,uVar1,*(int *)(param_1 + 0x36c) + -1,1);
        return;
      }
      uVar2 = uVar2 - *(uint *)(param_1 + 0x36c);
      if (uVar2 <= *(uint *)(param_1 + 0x370)) {
        uVar1 = *(undefined4 *)(param_1 + 0xec);
        piVar3 = (int *)FUN_005aaf90();
        (**(code **)(*piVar3 + 0x28))(0x23,uVar1,*(int *)(param_1 + 0x370) + -1,1);
        return;
      }
      uVar2 = uVar2 - *(uint *)(param_1 + 0x370);
      if (uVar2 <= *(uint *)(param_1 + 0x374)) {
        uVar1 = *(undefined4 *)(param_1 + 0xec);
        piVar3 = (int *)FUN_005aaf90();
        (**(code **)(*piVar3 + 0x28))(0x27,uVar1,*(int *)(param_1 + 0x374) + -1,1);
        return;
      }
      uVar2 = uVar2 - *(uint *)(param_1 + 0x374);
      if (uVar2 <= *(uint *)(param_1 + 0x378)) {
        uVar1 = *(undefined4 *)(param_1 + 0xec);
        piVar3 = (int *)FUN_005aaf90();
        (**(code **)(*piVar3 + 0x28))(0x25,uVar1,*(int *)(param_1 + 0x378) + -1,1);
        return;
      }
      if (uVar2 - *(uint *)(param_1 + 0x378) <= *(uint *)(param_1 + 0x37c)) {
        uVar1 = *(undefined4 *)(param_1 + 0xec);
        piVar3 = (int *)FUN_005aaf90();
        (**(code **)(*piVar3 + 0x28))(0x24,uVar1,*(int *)(param_1 + 0x37c) + -1,1);
      }
    }
  }
  return;
}

