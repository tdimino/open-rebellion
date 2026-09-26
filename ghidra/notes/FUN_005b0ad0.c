
void __fastcall FUN_005b0ad0(int param_1)

{
  undefined4 uVar1;
  int *piVar2;
  int iVar3;
  int *piVar4;
  
  piVar2 = (int *)(**(code **)(**(int **)(param_1 + 0x440) + 0x10))();
  if (piVar2 != (int *)0x0) {
    do {
      iVar3 = (**(code **)(*piVar2 + 0x10))();
      FUN_005b8400((void *)(param_1 + 0x418),iVar3);
      piVar2 = (int *)(**(code **)(**(int **)(param_1 + 0x440) + 0x10))();
    } while (piVar2 != (int *)0x0);
  }
  if (*(int *)(param_1 + 0x380) != 0) {
    uVar1 = *(undefined4 *)(param_1 + 0xec);
    piVar2 = (int *)FUN_005aaf90();
    (**(code **)(*piVar2 + 0x28))(0x29,uVar1,*(undefined4 *)(param_1 + 0x380),1);
    *(undefined4 *)(param_1 + 0x3ac) = 1;
  }
  piVar2 = (int *)(param_1 + 900);
  iVar3 = 4;
  do {
    if (*piVar2 != 0) {
      uVar1 = *(undefined4 *)(param_1 + 0xec);
      piVar4 = (int *)FUN_005aaf90();
      (**(code **)(*piVar4 + 0x28))(0x2b,uVar1,*piVar2,1);
    }
    piVar2 = piVar2 + 1;
    iVar3 = iVar3 + -1;
    *(undefined4 *)(param_1 + 0x3a8) = 1;
  } while (iVar3 != 0);
  return;
}

