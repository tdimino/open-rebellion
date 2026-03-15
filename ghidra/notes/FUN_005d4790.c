
void __fastcall FUN_005d4790(int param_1)

{
  int *piVar1;
  int iVar2;
  undefined4 uVar3;
  
  if (*(int *)(param_1 + 0x38) == 0) {
    return;
  }
  iVar2 = *(int *)(param_1 + 0x4c);
  *(undefined4 *)(param_1 + 0x38) = 0;
  if (iVar2 == 0) {
    piVar1 = *(int **)(param_1 + 0x2c);
    uVar3 = *(undefined4 *)(param_1 + 0x40);
    iVar2 = *piVar1;
  }
  else if (iVar2 == 1) {
    piVar1 = *(int **)(param_1 + 0x2c);
    uVar3 = *(undefined4 *)(param_1 + 0x44);
    iVar2 = *piVar1;
  }
  else {
    if (iVar2 != 2) goto LAB_005d47e2;
    piVar1 = *(int **)(param_1 + 0x2c);
    uVar3 = *(undefined4 *)(param_1 + 0x48);
    iVar2 = *piVar1;
  }
  iVar2 = (**(code **)(iVar2 + 0xa4))(piVar1,uVar3,0);
  FUN_005d8b00(iVar2);
LAB_005d47e2:
  FUN_005f5970((void *)(DAT_006bcbd4 + 0x5f4),*(int *)(param_1 + 0x18));
  return;
}

