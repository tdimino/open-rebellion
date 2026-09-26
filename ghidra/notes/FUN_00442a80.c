
void __cdecl
FUN_00442a80(undefined4 param_1,undefined4 param_2,int param_3,undefined4 *param_4,
            undefined4 *param_5,uint *param_6,int *param_7)

{
  int *piVar1;
  void *this;
  int iVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  
  if (param_3 != 0) {
    uVar4 = 0;
    uVar5 = 0;
    *param_6 = 0;
    *param_7 = 0;
    iVar6 = 0;
    for (piVar1 = *(int **)(param_3 + 8); piVar1 != (int *)0x0; piVar1 = (int *)piVar1[2]) {
      iVar2 = FUN_004abf60(piVar1);
      if (iVar6 < iVar2) {
        iVar6 = iVar2;
      }
    }
    for (this = *(void **)(param_3 + 8); this != (void *)0x0; this = *(void **)((int)this + 8)) {
      FUN_004abfa0(this,iVar6);
    }
    for (iVar6 = *(int *)(param_3 + 8); iVar6 != 0; iVar6 = *(int *)(iVar6 + 8)) {
      uVar3 = FUN_004abb80(iVar6);
      if (uVar5 < uVar3) {
        uVar5 = uVar3;
      }
      uVar3 = FUN_004abbf0(iVar6);
      if (uVar4 < uVar3) {
        uVar4 = uVar3;
      }
    }
    iVar6 = 2;
    for (piVar1 = *(int **)(param_3 + 8); piVar1 != (int *)0x0; piVar1 = (int *)piVar1[2]) {
      iVar6 = FUN_004abc70(piVar1,iVar6,uVar5,uVar4);
      iVar2 = (**(code **)(*piVar1 + 0x10))();
      if (iVar2 != 0) {
        FUN_004abd30((int)piVar1);
      }
      *param_7 = *param_7 + uVar4;
    }
    *param_6 = uVar5;
    *param_7 = *param_7 + 2;
    *param_4 = param_1;
    *param_5 = param_2;
  }
  return;
}

