
void __thiscall FUN_004eaaa0(int param_1,int *param_2,int *param_3)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  
  *param_2 = 0;
  *param_3 = 0;
  iVar1 = *(int *)(*(int *)(param_1 + 0xc) + 0x35c);
  iVar2 = *(int *)(*(int *)(param_1 + 0xc) + 0x220);
  if ((*param_2 == 0) && (iVar1 < iVar2)) {
    *param_2 = 1;
  }
  else if (iVar2 < iVar1) {
    *param_3 = 1;
  }
  if (*(int *)(param_1 + 4) == 2) {
    iVar1 = *(int *)(*(int *)(param_1 + 0xc) + 0x360);
    if ((((*param_2 == 0) && (*(int *)(*(int *)(param_1 + 0xc) + 0xa4) == 0)) &&
        (uVar3 = FUN_005f4960(), (uVar3 & 0x80) != 0)) && (iVar1 == 0)) {
      *param_2 = 6;
      return;
    }
    if (((*param_3 == 0) &&
        ((0 < *(int *)(*(int *)(param_1 + 0xc) + 0xa4) ||
         (uVar3 = FUN_005f4960(), (uVar3 & 0x80) == 0)))) && (iVar1 == 1)) {
      *param_3 = 6;
    }
  }
  return;
}

